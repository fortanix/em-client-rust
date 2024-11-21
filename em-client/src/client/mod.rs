/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(unused_extern_crates)]
extern crate mime;
extern crate chrono;
extern crate url;
extern crate uuid;

use hyper;
use hyper::client::{Request, Response};
use hyper::header::{Header, Headers, HeaderFormat, ContentType};
use hyper::method::Method;
use hyper::Url;
use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{Future, Stream};
use futures::{future, stream};
use std::borrow::Cow;
use std::io::{Read, Error, ErrorKind};
use std::error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use mimetypes;
use serde_json;

#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};

use ApiError;
use SimpleErrorType;

use {
    AccountsApi,
    AppApi,
    ApplicationConfigApi,
    ApprovalRequestsApi,
    AuthApi,
    BuildApi,
    CertificateApi,
    DatasetApi,
    NodeApi,
    RegistryApi,
    SystemApi,
    TaskApi,
    ToolsApi,
    UsersApi,
    WorkflowApi,
    WorkflowFinalApi,
    ZoneApi
};

use models;
use mbedtls::hash;

define_encode_set! {
    /// This encode set is used for object IDs
    ///
    /// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
    /// the vertical bar (|) is encoded.
    pub ID_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | {'|'}
}

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: &str, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Url, since a base path is a subset of Url.
    let url = Url::from_str(input)?;

    let scheme = url.scheme();

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = url.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = url.port().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}", scheme, host, port))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client {
    hyper_client: Arc<hyper::client::Client>,
    base_path: String,
    headers: Headers,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client {
            hyper_client: self.hyper_client.clone(),
            base_path: self.base_path.clone(),
            headers: self.headers.clone(),
        }
    }
}

impl Client {

    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    pub fn try_new_http(base_path: &str) -> Result<Client, ClientInitError> {
        Self::try_new_with_connector(
            base_path,
            Some("http"),
            hyper::net::HttpConnector,
        )
    }

    /// Create a client with a custom implementation of hyper::net::NetworkConnector.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with
    /// `swagger::{http_connector, https_connector, https_mutual_connector}`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - An instance of `C: hyper::net::NetworkConnection`
    pub fn try_new_with_connector<C, S>(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: C,
    ) -> Result<Client, ClientInitError>
    where
        C: hyper::net::NetworkConnector<Stream = S> + Send + Sync + 'static,
        S: hyper::net::NetworkStream
    {
        let hyper_client = hyper::Client::with_connector(connector);

        Ok(Client {
            hyper_client: Arc::new(hyper_client),
            base_path: into_base_path(base_path, protocol)?,
            headers: Headers::new(),
        })
    }

    /// Constructor for creating a `Client` by passing in a pre-made `hyper` client.
    ///
    /// One should avoid relying on this function if possible, since it adds a dependency on the underlying transport
    /// implementation, which it would be better to abstract away. Therefore, using this function may lead to a loss of
    /// code generality, which may make it harder to move the application to a serverless environment, for example.
    ///
    /// The reason for this function's existence is to support legacy test code, which did mocking at the hyper layer.
    /// This is not a recommended way to write new tests. If other reasons are found for using this function, they
    /// should be mentioned here.
    ///
    /// This function is deprecated in the upstream openapi-generator which
    /// uses newer hyper. However, the suggested replacement does not exist
    /// in hyper 0.9.
    pub fn try_new_with_hyper_client(
        hyper_client: Arc<hyper::client::Client>,
        base_path: &str
    ) -> Result<Client, ClientInitError>
    {
        Ok(Client {
            hyper_client: hyper_client,
            base_path: into_base_path(base_path, None)?,
            headers: Headers::new(),
        })
    }

    pub fn headers(&mut self) -> &mut Headers {
        &mut self.headers
    }
}


impl AccountsApi for Client {

    type Error = ApiError;


    fn create_account(&self, param_body: models::AccountRequest) -> Result<models::Account, ApiError> {
        let mut url = format!(
            "{}/v1/accounts",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_ACCOUNT.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_account(&self, param_account_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/accounts/{account_id}",
            self.base_path, account_id=utf8_percent_encode(&param_account_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_account(&self, param_account_id: uuid::Uuid) -> Result<models::Account, ApiError> {
        let mut url = format!(
            "{}/v1/accounts/{account_id}",
            self.base_path, account_id=utf8_percent_encode(&param_account_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_accounts(&self) -> Result<models::AccountListResponse, ApiError> {
        let mut url = format!(
            "{}/v1/accounts",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::AccountListResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn select_account(&self, param_account_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/accounts/select_account/{account_id}",
            self.base_path, account_id=utf8_percent_encode(&param_account_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_account(&self, param_account_id: uuid::Uuid, param_body: models::AccountUpdateRequest) -> Result<models::Account, ApiError> {
        let mut url = format!(
            "{}/v1/accounts/{account_id}",
            self.base_path, account_id=utf8_percent_encode(&param_account_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_ACCOUNT.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl AppApi for Client {

    type Error = ApiError;


    fn add_application(&self, param_body: models::AppRequest) -> Result<models::App, ApiError> {
        let mut url = format!(
            "{}/v1/apps",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::ADD_APPLICATION.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_app(&self, param_app_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/apps/{app_id}",
            self.base_path, app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_apps(&self, param_name: Option<String>, param_description: Option<String>, param_all_search: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>) -> Result<models::GetAllAppsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/apps",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllAppsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_app(&self, param_app_id: uuid::Uuid) -> Result<models::App, ApiError> {
        let mut url = format!(
            "{}/v1/apps/{app_id}",
            self.base_path, app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_app_certificate(&self, param_node_id: uuid::Uuid, param_app_id: uuid::Uuid) -> Result<models::Certificate, ApiError> {
        let mut url = format!(
            "{}/v1/apps/{app_id}/node/{node_id}/certificate",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET), app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_app_node_certificate_details(&self, param_node_id: uuid::Uuid, param_app_id: uuid::Uuid) -> Result<models::CertificateDetails, ApiError> {
        let mut url = format!(
            "{}/v1/apps/{app_id}/node/{node_id}/certificate-details",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET), app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::CertificateDetails>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, ApiError> {
        let mut url = format!(
            "{}/v1/apps/unique_labels/count",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::LabelsCount>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_app(&self, param_app_id: uuid::Uuid, param_body: models::AppBodyUpdateRequest) -> Result<models::App, ApiError> {
        let mut url = format!(
            "{}/v1/apps/{app_id}",
            self.base_path, app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_APP.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl ApplicationConfigApi for Client {

    type Error = ApiError;


    fn create_application_config(&self, param_body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, ApiError> {
        let mut url = format!(
            "{}/v1/app_configs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_APPLICATION_CONFIG.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApplicationConfigResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_application_config(&self, param_config_id: String) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/app_configs/{config_id}",
            self.base_path, config_id=utf8_percent_encode(&param_config_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_application_configs(&self, param_name: Option<String>, param_description: Option<String>, param_image_id: Option<uuid::Uuid>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/app_configs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(image_id) = param_image_id {
            query_string.append_pair("image_id", &image_id.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllApplicationConfigsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_application_config(&self, param_config_id: String) -> Result<models::ApplicationConfigResponse, ApiError> {
        let mut url = format!(
            "{}/v1/app_configs/{config_id}",
            self.base_path, config_id=utf8_percent_encode(&param_config_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApplicationConfigResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, ApiError> {
        let mut url = format!(
            "{}/v1/runtime/app_configs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        let raw_config = request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        String::from_utf8(body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                                  code,
                                                  headers,
                                                  match result {
                                                      Ok(_) => match str::from_utf8(&body) {
                                                          Ok(body) => Cow::from(body),
                                                          Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                                      },
                                                      Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                                                  }), err_type))
                    }
                }
            })?;

        let mut hash = vec![0; 32];
        hash::Md::hash(mbedtls::hash::Type::Sha256,
                       raw_config.as_bytes(), &mut hash)
            .map_err(|e| ApiError::new(format!("Unable to hash app config: {}", e), SimpleErrorType::Permanent))?;

        if hash != expected_hash {
            Err(ApiError::new(format!("App config hash mismatch. Expected {:?}, but got {:?}", hash, expected_hash).to_string(), SimpleErrorType::Permanent))
        } else {
            serde_json::from_str::<models::RuntimeAppConfig>(&raw_config)
                .map_err(|e| e.into())
        }
    }

    fn get_specific_runtime_application_config(&self, param_config_id: String) -> Result<models::RuntimeAppConfig, ApiError> {
        let mut url = format!(
            "{}/v1/runtime/app_configs/{config_id}",
            self.base_path, config_id=utf8_percent_encode(&param_config_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::RuntimeAppConfig>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_application_config(&self, param_config_id: String, param_body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, ApiError> {
        let mut url = format!(
            "{}/v1/app_configs/{config_id}",
            self.base_path, config_id=utf8_percent_encode(&param_config_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_APPLICATION_CONFIG.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApplicationConfigResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl ApprovalRequestsApi for Client {

    type Error = ApiError;


    fn approve_approval_request(&self, param_request_id: uuid::Uuid, param_body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests/{request_id}/approve",
            self.base_path, request_id=utf8_percent_encode(&param_request_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = param_body.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
                .into_bytes()
        });

        if let Some(body) = body.as_ref() {
            request = request.body(body.as_slice());
        }

        request = request.header(ContentType(mimetypes::requests::APPROVE_APPROVAL_REQUEST.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApprovalRequest>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn create_approval_request(&self, param_body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_APPROVAL_REQUEST.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    201 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApprovalRequest>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_approval_request(&self, param_request_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests/{request_id}",
            self.base_path, request_id=utf8_percent_encode(&param_request_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn deny_approval_request(&self, param_request_id: uuid::Uuid, param_body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests/{request_id}/deny",
            self.base_path, request_id=utf8_percent_encode(&param_request_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = param_body.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
                .into_bytes()
        });

        if let Some(body) = body.as_ref() {
            request = request.body(body.as_slice());
        }

        request = request.header(ContentType(mimetypes::requests::DENY_APPROVAL_REQUEST.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApprovalRequest>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_approval_requests(&self, param_requester: Option<uuid::Uuid>, param_reviewer: Option<uuid::Uuid>, param_subject: Option<uuid::Uuid>, param_status: Option<String>, param_all_search: Option<String>, param_sort_by: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllApprovalRequests, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(requester) = param_requester {
            query_string.append_pair("requester", &requester.to_string());
        }
        if let Some(reviewer) = param_reviewer {
            query_string.append_pair("reviewer", &reviewer.to_string());
        }
        if let Some(subject) = param_subject {
            query_string.append_pair("subject", &subject.to_string());
        }
        if let Some(status) = param_status {
            query_string.append_pair("status", &status.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllApprovalRequests>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_approval_request(&self, param_request_id: uuid::Uuid) -> Result<models::ApprovalRequest, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests/{request_id}",
            self.base_path, request_id=utf8_percent_encode(&param_request_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApprovalRequest>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_approval_request_result(&self, param_request_id: uuid::Uuid) -> Result<models::ApprovableResult, ApiError> {
        let mut url = format!(
            "{}/v1/approval_requests/{request_id}/result",
            self.base_path, request_id=utf8_percent_encode(&param_request_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ApprovableResult>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl AuthApi for Client {

    type Error = ApiError;


    fn authenticate_user(&self, param_body: Option<models::AuthRequest>) -> Result<models::AuthResponse, ApiError> {
        let mut url = format!(
            "{}/v1/sys/auth",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = param_body.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
                .into_bytes()
        });

        if let Some(body) = body.as_ref() {
            request = request.body(body.as_slice());
        }

        request = request.header(ContentType(mimetypes::requests::AUTHENTICATE_USER.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::AuthResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl BuildApi for Client {

    type Error = ApiError;


    fn convert_app_build(&self, param_body: models::ConvertAppBuildRequest) -> Result<models::Build, ApiError> {
        let mut url = format!(
            "{}/v1/builds/convert-app",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CONVERT_APP_BUILD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn create_build(&self, param_body: models::CreateBuildRequest) -> Result<models::Build, ApiError> {
        let mut url = format!(
            "{}/v1/builds",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_BUILD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_build(&self, param_build_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/builds/{build_id}",
            self.base_path, build_id=utf8_percent_encode(&param_build_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_builds(&self, param_all_search: Option<String>, param_docker_image_name: Option<String>, param_config_id: Option<String>, param_deployed_status: Option<String>, param_status: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/builds",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(docker_image_name) = param_docker_image_name {
            query_string.append_pair("docker_image_name", &docker_image_name.to_string());
        }
        if let Some(config_id) = param_config_id {
            query_string.append_pair("config_id", &config_id.to_string());
        }
        if let Some(deployed_status) = param_deployed_status {
            query_string.append_pair("deployed_status", &deployed_status.to_string());
        }
        if let Some(status) = param_status {
            query_string.append_pair("status", &status.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllBuildsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_build(&self, param_build_id: uuid::Uuid) -> Result<models::Build, ApiError> {
        let mut url = format!(
            "{}/v1/builds/{build_id}",
            self.base_path, build_id=utf8_percent_encode(&param_build_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_build_deployments(&self, param_build_id: uuid::Uuid, param_status: Option<String>, param_all_search: Option<String>, param_sort_by: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/builds/deployments/{build_id}",
            self.base_path, build_id=utf8_percent_encode(&param_build_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(status) = param_status {
            query_string.append_pair("status", &status.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllBuildDeploymentsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_build(&self, param_build_id: uuid::Uuid, param_body: models::BuildUpdateRequest) -> Result<models::Build, ApiError> {
        let mut url = format!(
            "{}/v1/builds/{build_id}",
            self.base_path, build_id=utf8_percent_encode(&param_build_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_BUILD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl CertificateApi for Client {

    type Error = ApiError;


    fn get_certificate(&self, param_cert_id: uuid::Uuid) -> Result<models::Certificate, ApiError> {
        let mut url = format!(
            "{}/v1/certificates/{cert_id}",
            self.base_path, cert_id=utf8_percent_encode(&param_cert_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn new_certificate(&self, param_body: models::NewCertificateRequest) -> Result<models::TaskResult, ApiError> {
        let mut url = format!(
            "{}/v1/certificates",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::NEW_CERTIFICATE.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl DatasetApi for Client {

    type Error = ApiError;


    fn create_dataset(&self, param_body: models::CreateDatasetRequest) -> Result<models::Dataset, ApiError> {
        let mut url = format!(
            "{}/v1/datasets",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_DATASET.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Dataset>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_dataset(&self, param_dataset_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/datasets/{dataset_id}",
            self.base_path, dataset_id=utf8_percent_encode(&param_dataset_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_datasets(&self, param_name: Option<String>, param_description: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/datasets",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllDatasetsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_dataset(&self, param_dataset_id: uuid::Uuid) -> Result<models::Dataset, ApiError> {
        let mut url = format!(
            "{}/v1/datasets/{dataset_id}",
            self.base_path, dataset_id=utf8_percent_encode(&param_dataset_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Dataset>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_dataset(&self, param_dataset_id: uuid::Uuid, param_body: models::DatasetUpdateRequest) -> Result<models::Dataset, ApiError> {
        let mut url = format!(
            "{}/v1/datasets/{dataset_id}",
            self.base_path, dataset_id=utf8_percent_encode(&param_dataset_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_DATASET.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Dataset>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl NodeApi for Client {

    type Error = ApiError;


    fn deactivate_node(&self, param_node_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/nodes/{node_id}/deactivate",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_nodes(&self, param_name: Option<String>, param_description: Option<String>, param_sgx_version: Option<String>, param_all_search: Option<String>, param_status: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>) -> Result<models::GetAllNodesResponse, ApiError> {
        let mut url = format!(
            "{}/v1/nodes",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(sgx_version) = param_sgx_version {
            query_string.append_pair("sgx_version", &sgx_version.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(status) = param_status {
            query_string.append_pair("status", &status.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllNodesResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_node(&self, param_node_id: uuid::Uuid) -> Result<models::Node, ApiError> {
        let mut url = format!(
            "{}/v1/nodes/{node_id}",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Node>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_node_certificate(&self, param_node_id: uuid::Uuid) -> Result<models::Certificate, ApiError> {
        let mut url = format!(
            "{}/v1/nodes/{node_id}/certificate",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_node_certificate_details(&self, param_node_id: uuid::Uuid) -> Result<models::CertificateDetails, ApiError> {
        let mut url = format!(
            "{}/v1/nodes/{node_id}/certificate-details",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::CertificateDetails>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, ApiError> {
        let mut url = format!(
            "{}/v1/nodes/unique_labels/count",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::LabelsCount>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn provision_node(&self, param_body: models::NodeProvisionRequest) -> Result<models::TaskResult, ApiError> {
        let mut url = format!(
            "{}/v1/nodes",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::PROVISION_NODE.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_node(&self, param_node_id: uuid::Uuid, param_body: models::NodeUpdateRequest) -> Result<models::Node, ApiError> {
        let mut url = format!(
            "{}/v1/nodes/{node_id}",
            self.base_path, node_id=utf8_percent_encode(&param_node_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_NODE.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Node>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_node_status(&self, param_body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, ApiError> {
        let mut url = format!(
            "{}/v1/node/status",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_NODE_STATUS.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::NodeStatusResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl RegistryApi for Client {

    type Error = ApiError;


    fn create_registry(&self, param_registry_request: models::RegistryRequest) -> Result<models::Registry, ApiError> {
        let mut url = format!(
            "{}/v1/registry",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_registry_request).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_REGISTRY.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Registry>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_registry(&self, param_registry_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/registry/{registry_id}",
            self.base_path, registry_id=utf8_percent_encode(&param_registry_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_registries(&self) -> Result<Vec<models::Registry>, ApiError> {
        let mut url = format!(
            "{}/v1/registry",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<Vec<models::Registry>>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_registry(&self, param_registry_id: uuid::Uuid) -> Result<models::Registry, ApiError> {
        let mut url = format!(
            "{}/v1/registry/{registry_id}",
            self.base_path, registry_id=utf8_percent_encode(&param_registry_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Registry>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_registry_for_app(&self, param_app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, ApiError> {
        let mut url = format!(
            "{}/v1/registry/app/{app_id}",
            self.base_path, app_id=utf8_percent_encode(&param_app_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::AppRegistryResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_registry_for_image(&self, param_image_name: String) -> Result<models::ImageRegistryResponse, ApiError> {
        let mut url = format!(
            "{}/v1/image/registry",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());
        query_string.append_pair("image_name", &param_image_name.to_string());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ImageRegistryResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_registry(&self, param_registry_id: uuid::Uuid, param_body: models::UpdateRegistryRequest) -> Result<models::Registry, ApiError> {
        let mut url = format!(
            "{}/v1/registry/{registry_id}",
            self.base_path, registry_id=utf8_percent_encode(&param_registry_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_REGISTRY.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Registry>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl SystemApi for Client {

    type Error = ApiError;


    fn get_manager_version(&self) -> Result<models::VersionResponse, ApiError> {
        let mut url = format!(
            "{}/v1/sys/version",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::VersionResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl TaskApi for Client {

    type Error = ApiError;


    fn get_all_tasks(&self, param_task_type: Option<String>, param_status: Option<String>, param_requester: Option<String>, param_approver: Option<String>, param_all_search: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>, param_base_filters: Option<String>) -> Result<models::GetAllTasksResponse, ApiError> {
        let mut url = format!(
            "{}/v1/tasks",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(task_type) = param_task_type {
            query_string.append_pair("task_type", &task_type.to_string());
        }
        if let Some(status) = param_status {
            query_string.append_pair("status", &status.to_string());
        }
        if let Some(requester) = param_requester {
            query_string.append_pair("requester", &requester.to_string());
        }
        if let Some(approver) = param_approver {
            query_string.append_pair("approver", &approver.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        if let Some(base_filters) = param_base_filters {
            query_string.append_pair("base_filters", &base_filters.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllTasksResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_task(&self, param_task_id: uuid::Uuid) -> Result<models::Task, ApiError> {
        let mut url = format!(
            "{}/v1/tasks/{task_id}",
            self.base_path, task_id=utf8_percent_encode(&param_task_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Task>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_task_status(&self, param_task_id: uuid::Uuid) -> Result<models::TaskResult, ApiError> {
        let mut url = format!(
            "{}/v1/tasks/status/{task_id}",
            self.base_path, task_id=utf8_percent_encode(&param_task_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_task(&self, param_task_id: uuid::Uuid, param_body: models::TaskUpdateRequest) -> Result<models::TaskResult, ApiError> {
        let mut url = format!(
            "{}/v1/tasks/{task_id}",
            self.base_path, task_id=utf8_percent_encode(&param_task_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_TASK.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl ToolsApi for Client {

    type Error = ApiError;


    fn convert_app(&self, param_body: models::ConversionRequest) -> Result<models::ConversionResponse, ApiError> {
        let mut url = format!(
            "{}/v1/tools/converter/convert-app",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CONVERT_APP.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ConversionResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl UsersApi for Client {

    type Error = ApiError;


    fn accept_terms_and_conditions(&self) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/terms_and_conditions",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn change_password(&self, param_body: models::PasswordChangeRequest) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/change_password",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CHANGE_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn confirm_email(&self, param_body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, ApiError> {
        let mut url = format!(
            "{}/v1/users/confirm_email",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CONFIRM_EMAIL.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ConfirmEmailResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn create_user(&self, param_body: models::SignupRequest) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/users",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_USER.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    201 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_user_account(&self, param_user_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_user_from_account(&self, param_user_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}/accounts",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn forgot_password(&self, param_body: models::ForgotPasswordRequest) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/forgot_password",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::FORGOT_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_users(&self, param_all_search: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>) -> Result<models::GetAllUsersResponse, ApiError> {
        let mut url = format!(
            "{}/v1/users",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllUsersResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_logged_in_user(&self) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/user",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_user(&self, param_user_id: uuid::Uuid) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn invite_user(&self, param_body: models::InviteUserRequest) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/users/invite",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::INVITE_USER.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    201 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn process_invitations(&self, param_body: models::ProcessInviteRequest) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/process_invite",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::PROCESS_INVITATIONS.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn resend_confirm_email(&self) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/resend_confirm_email",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn resend_invitation(&self, param_user_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}/resend_invite",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn reset_password(&self, param_user_id: uuid::Uuid, param_body: models::PasswordResetRequest) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}/reset_password",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::RESET_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_user(&self, param_user_id: uuid::Uuid, param_body: models::UpdateUserRequest) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_USER.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn validate_password_reset_token(&self, param_user_id: uuid::Uuid, param_body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, ApiError> {
        let mut url = format!(
            "{}/v1/users/{user_id}/validate_token",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::VALIDATE_PASSWORD_RESET_TOKEN.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ValidateTokenResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl WorkflowApi for Client {

    type Error = ApiError;


    fn create_workflow_graph(&self, param_body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/draft/graphs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_WORKFLOW_GRAPH.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::WorkflowGraph>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_workflow_graph(&self, param_graph_id: uuid::Uuid) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/workflows/draft/graphs/{graph_id}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_workflow_graphs(&self, param_name: Option<String>, param_description: Option<String>, param_all_search: Option<String>, param_parent_graph_id: Option<String>, param_sort_by: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/draft/graphs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(parent_graph_id) = param_parent_graph_id {
            query_string.append_pair("parent_graph_id", &parent_graph_id.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllWorkflowGraphsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_workflow_graph(&self, param_graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/draft/graphs/{graph_id}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::WorkflowGraph>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_workflow_graph(&self, param_graph_id: uuid::Uuid, param_body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/draft/graphs/{graph_id}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Put, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_WORKFLOW_GRAPH.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::WorkflowGraph>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl WorkflowFinalApi for Client {

    type Error = ApiError;


    fn create_final_workflow_graph(&self, param_body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::CREATE_FINAL_WORKFLOW_GRAPH.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::FinalWorkflow>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn delete_final_workflow_graph(&self, param_graph_id: uuid::Uuid, param_version: String) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs/{graph_id}/{version}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET), version=utf8_percent_encode(&param_version.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    204 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_all_final_workflow_graphs(&self, param_name: Option<String>, param_description: Option<String>, param_all_search: Option<String>, param_sort_by: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(name) = param_name {
            query_string.append_pair("name", &name.to_string());
        }
        if let Some(description) = param_description {
            query_string.append_pair("description", &description.to_string());
        }
        if let Some(all_search) = param_all_search {
            query_string.append_pair("all_search", &all_search.to_string());
        }
        if let Some(sort_by) = param_sort_by {
            query_string.append_pair("sort_by", &sort_by.to_string());
        }
        if let Some(limit) = param_limit {
            query_string.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = param_offset {
            query_string.append_pair("offset", &offset.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::GetAllFinalWorkflowGraphsResponse>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_final_workflow_graph(&self, param_graph_id: uuid::Uuid, param_version: String) -> Result<models::VersionInFinalWorkflow, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs/{graph_id}/{version}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET), version=utf8_percent_encode(&param_version.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::VersionInFinalWorkflow>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_full_final_workflow_graph(&self, param_graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs/{graph_id}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::FinalWorkflow>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn update_final_workflow_graph(&self, param_graph_id: uuid::Uuid, param_body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, ApiError> {
        let mut url = format!(
            "{}/v1/workflows/final/graphs/{graph_id}",
            self.base_path, graph_id=utf8_percent_encode(&param_graph_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize")
            .into_bytes();
            request = request.body(body.as_slice());

        request = request.header(ContentType(mimetypes::requests::UPDATE_FINAL_WORKFLOW_GRAPH.clone()));

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::VersionInFinalWorkflow>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}

impl ZoneApi for Client {

    type Error = ApiError;


    fn get_zone(&self, param_zone_id: uuid::Uuid) -> Result<models::Zone, ApiError> {
        let mut url = format!(
            "{}/v1/zones/{zone_id}",
            self.base_path, zone_id=utf8_percent_encode(&param_zone_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::Zone>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_zone_join_token(&self, param_zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, ApiError> {
        let mut url = format!(
            "{}/v1/zones/{zone_id}/token",
            self.base_path, zone_id=utf8_percent_encode(&param_zone_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<models::ZoneJoinToken>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

    fn get_zones(&self) -> Result<Vec<models::Zone>, ApiError> {
        let mut url = format!(
            "{}/v1/zones",
            self.base_path
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError::new(format!("Unable to build URL: {}", err), SimpleErrorType::Permanent)),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError::new(format!("No response received: {}", e), SimpleErrorType::Permanent))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError::new(format!("Failed to read response: {}", e), SimpleErrorType::Temporary))?;
                        str::from_utf8(&body)
                            .map_err(|e| ApiError::new(format!("Response was not valid UTF8: {}", e), SimpleErrorType::Temporary))
                            .and_then(|body| {
                                 serde_json::from_str::<Vec<models::Zone>>(body)
                                         .map_err(|e| e.into())
                            })
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        let err_type = match response.status.is_server_error() {
                            false => SimpleErrorType::Permanent,
                            true => SimpleErrorType::Temporary,
                        };
                        Err(ApiError::new(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            }), err_type))
                    }
                }
        })

    }

}


#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUri(hyper::error::ParseError),
    MissingHost,
}

impl From<hyper::error::ParseError> for ClientInitError {
    fn from(err: hyper::error::ParseError) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl error::Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}
