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

use crate::ApiError;

use {
    AccountsApi,
    AppApi,
    AuthApi,
    BuildApi,
    CertificateApi,
    NodeApi,
    SystemApi,
    TaskApi,
    ToolsApi,
    UsersApi,
    ZoneApi
};

use models;

define_encode_set! {
    /// This encode set is used for object IDs
    ///
    /// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
    /// the vertical bar (|) is encoded.
    pub ID_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | {'|'}
}


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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CREATE_ACCOUNT.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::AccountListResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::UPDATE_ACCOUNT.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Account>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::ADD_APPLICATION.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllAppsResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::CertificateDetails>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::UPDATE_APP.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::App>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
                    }
                }
        })

    }

}

impl AuthApi for Client {

    type Error = ApiError;


    fn authenticate_user(&self) -> Result<models::AuthResponse, ApiError> {
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::AuthResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CONVERT_APP_BUILD.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CREATE_BUILD.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
                    }
                }
        })

    }

    fn get_all_builds(&self, param_all_search: Option<String>, param_docker_image_name: Option<String>, param_deployed_status: Option<String>, param_status: Option<String>, param_limit: Option<i32>, param_offset: Option<i32>, param_sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, ApiError> {
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllBuildsResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Build>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllBuildDeploymentsResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllNodesResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Node>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Certificate>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::CertificateDetails>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::VersionResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllTasksResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Task>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::UPDATE_TASK.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::TaskResult>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CONVERT_APP.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::ConversionResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
                    }
                }
        })

    }

}

impl UsersApi for Client {

    type Error = ApiError;


    fn blacklist_user(&self, param_body: models::UserBlacklistRequest) -> Result<(), ApiError> {
        let mut url = format!(
            "{}/v1/users/blacklist",
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::BLACKLIST_USER.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CHANGE_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CONFIRM_EMAIL.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::ConfirmEmailResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::CREATE_USER.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    201 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Delete, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::FORGOT_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::GetAllUsersResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::INVITE_USER.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    201 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::PROCESS_INVITATIONS.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::RESET_PASSWORD.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        Ok(())
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Patch, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::UPDATE_USER.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Post, url);
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");
            request = request.body(body.as_bytes());

        request = request.header(ContentType(mimetypes::requests::VALIDATE_PASSWORD_RESET_TOKEN.clone()));

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::ValidateTokenResponse>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
                    }
                }
        })

    }

    fn whitelist_user(&self, param_user_id: uuid::Uuid, param_user_token: Option<String>) -> Result<models::User, ApiError> {
        let mut url = format!(
            "{}/v1/users/whitelist/{user_id}",
            self.base_path, user_id=utf8_percent_encode(&param_user_id.to_string(), ID_ENCODE_SET)
        );

        let mut query_string = self::url::form_urlencoded::Serializer::new("".to_owned());

        if let Some(user_token) = param_user_token {
            query_string.append_pair("user_token", &user_token.to_string());
        }
        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let url = match Url::from_str(&url) {
            Ok(url) => url,
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::User>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::Zone>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<models::ZoneJoinToken>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
            Err(err) => return Err(ApiError(format!("Unable to build URL: {}", err))),
        };

        let mut request = self.hyper_client.request(Method::Get, url);
        request = request.headers(self.headers.clone());

        request.send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| {
                match response.status.to_u16() {
                    200 => {
                        let mut body = Vec::new();
                        response.read_to_end(&mut body)
                            .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                        str::from_utf8(&body)
                            .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                            .and_then(|body|
                                 serde_json::from_str::<Vec<models::Zone>>(body)
                                         .map_err(|e| e.into())

                                 )
                    },
                    code => {
                        let headers = response.headers.clone();
                        let mut body = Vec::new();
                        let result = response.read_to_end(&mut body);
                        Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                            code,
                            headers,
                            match result {
                                Ok(_) => match str::from_utf8(&body) {
                                    Ok(body) => Cow::from(body),
                                    Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                                },
                                Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                            })))
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
