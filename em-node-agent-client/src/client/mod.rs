/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(unused_extern_crates)]
extern crate chrono;
extern crate mime;
extern crate url;
extern crate uuid;

use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use crate::{ApiError, ApiErrorDetails, CertificateApi, EnclaveApi, ServerErrorResponse, SimpleErrorType, SystemApi, mimetypes, models};
use async_trait::async_trait;
use headers::ContentType;
use hyper::{HeaderMap, Method};
use simple_hyper_client::Client as HttpClient;
use simple_hyper_client::{HttpConnector, NetworkConnector};
use std::borrow::Cow;
use std::str::FromStr;
use std::sync::Arc;
use std::{error, fmt, str};
use url::{ParseError as UrlParseError, Url};

define_encode_set! {
    /// This encode set is used for object IDs
    ///
    /// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
    /// the vertical bar (|) is encoded.
    pub ID_ENCODE_SET = [PATH_SEGMENT_ENCODE_SET] | {'|'}
}

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(
    input: &str,
    correct_scheme: Option<&'static str>,
) -> Result<String, ClientInitError> {
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
    hyper_client: Arc<HttpClient>,
    base_path: String,
    headers: HeaderMap,
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
        Self::try_new_with_connector(base_path, Some("http"), HttpConnector::new())
    }

    /// Create a client with a custom implementation of simple_hyper_client::NetworkConnector.
    ///
    /// Use this constructor for custom transports, including TLS. The connector
    /// and API methods use asynchronous I/O. Await API calls inside a Tokio runtime.
    /// Hyper 0.10 connectors do not implement this trait and must be migrated.
    /// For ordinary HTTP connections, use `try_new_http`.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - An instance of `C: simple_hyper_client::NetworkConnector`
    pub fn try_new_with_connector<C>(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: C,
    ) -> Result<Client, ClientInitError>
    where
        C: NetworkConnector,
    {
        let hyper_client = HttpClient::with_connector(connector);

        Ok(Client {
            hyper_client: Arc::new(hyper_client),
            base_path: into_base_path(base_path, protocol)?,
            headers: HeaderMap::new(),
        })
    }

    /// Create a client using a configured `simple_hyper_client::Client`.
    ///
    /// The method name is retained for callers migrating from Hyper 0.10, but the
    /// argument is now the asynchronous client built on Hyper 1.x.
    ///
    /// ```
    /// extern crate em_node_agent_client;
    /// extern crate simple_hyper_client;
    ///
    /// use em_node_agent_client::Client;
    /// use simple_hyper_client::{Client as HttpClient, HttpConnector};
    /// use std::sync::Arc;
    ///
    /// let transport = HttpClient::with_connector(HttpConnector::new());
    /// let client = Client::try_new_with_hyper_client(
    ///     Arc::new(transport), "http://localhost:9092",
    /// ).unwrap();
    /// ```
    pub fn try_new_with_hyper_client(
        hyper_client: Arc<HttpClient>,
        base_path: &str,
    ) -> Result<Client, ClientInitError> {
        Ok(Client {
            hyper_client: hyper_client,
            base_path: into_base_path(base_path, None)?,
            headers: HeaderMap::new(),
        })
    }

    pub fn headers(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

#[async_trait]
impl CertificateApi for Client {
    type Error = ApiError;

    async fn get_issue_certificate_response(
        &self,
        param_task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, ApiError> {
        let operation_path = format!(
            "/v1/certificate/result/{task_id}",
            task_id = utf8_percent_encode(&param_task_id.to_string(), ID_ENCODE_SET)
        );
        let mut url = format!("{}{}", self.base_path, operation_path);

        let query_string_str = {
            let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let mut request = self.hyper_client.request(Method::POST, url)?;
        request = request.headers(self.headers.clone());

        let mut response = request.send().await.map_err(|e| {
            ApiError::new(
                SimpleErrorType::Permanent,
                ApiErrorDetails::Message(format!("No response received: {}", e)),
            )
        })?;
        match response.status().as_u16() {
            200 => {
                let body = simple_hyper_client::to_bytes(response.body_mut())
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Failed to read response: {}", e)),
                        )
                    })?;
                str::from_utf8(&body)
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Response was not valid UTF8: {}", e)),
                        )
                    })
                    .and_then(|body| {
                        serde_json::from_str::<models::IssueCertificateResponse>(body)
                            .map_err(|e| e.into())
                    })
            }
            code => {
                let headers = response.headers().clone();
                let status = response.status();
                let result = simple_hyper_client::to_bytes(response).await;
                let err_type = match status.is_server_error() {
                    false => SimpleErrorType::Permanent,
                    true => SimpleErrorType::Temporary,
                };

                let response_body = result.ok().map(|bytes| bytes.to_vec());
                let response = ApiErrorDetails::ServerResponse(ServerErrorResponse::new(
                    code,
                    response_body,
                    headers,
                ));
                let api_err = ApiError::new(err_type, response);
                Err(api_err)
            } // error handler
        }
    } // end: fn get_issue_certificate_response

    async fn issue_certificate(
        &self,
        param_body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, ApiError> {
        let operation_path = format!("/v1/certificate/issue");
        let mut url = format!("{}{}", self.base_path, operation_path);

        let query_string_str = {
            let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let mut request = self.hyper_client.request(Method::POST, url)?;
        request = request.headers(self.headers.clone());
        let body = serde_json::to_string(&param_body)
            .expect("impossible to fail to serialize")
            .into_bytes();
        request = request.body(simple_hyper_client::SharedBody::from(body.clone()));

        request = request.header(ContentType::from(
            mimetypes::requests::ISSUE_CERTIFICATE.clone(),
        ));

        let mut response = request.send().await.map_err(|e| {
            ApiError::new(
                SimpleErrorType::Permanent,
                ApiErrorDetails::Message(format!("No response received: {}", e)),
            )
        })?;
        match response.status().as_u16() {
            200 => {
                let body = simple_hyper_client::to_bytes(response.body_mut())
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Failed to read response: {}", e)),
                        )
                    })?;
                str::from_utf8(&body)
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Response was not valid UTF8: {}", e)),
                        )
                    })
                    .and_then(|body| {
                        serde_json::from_str::<models::IssueCertificateResponse>(body)
                            .map_err(|e| e.into())
                    })
            }
            code => {
                let headers = response.headers().clone();
                let status = response.status();
                let result = simple_hyper_client::to_bytes(response).await;
                let err_type = match status.is_server_error() {
                    false => SimpleErrorType::Permanent,
                    true => SimpleErrorType::Temporary,
                };

                let response_body = result.ok().map(|bytes| bytes.to_vec());
                let response = ApiErrorDetails::ServerResponse(ServerErrorResponse::new(
                    code,
                    response_body,
                    headers,
                ));
                let api_err = ApiError::new(err_type, response);
                Err(api_err)
            } // error handler
        }
    } // end: fn issue_certificate
}

#[async_trait]
impl EnclaveApi for Client {
    type Error = ApiError;

    async fn get_fortanix_attestation(
        &self,
        param_body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, ApiError> {
        let operation_path = format!("/v1/enclave/attest");
        let mut url = format!("{}{}", self.base_path, operation_path);

        let query_string_str = {
            let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let mut request = self.hyper_client.request(Method::POST, url)?;
        request = request.headers(self.headers.clone());
        // Body parameter
        let body = serde_json::to_string(&param_body)
            .expect("impossible to fail to serialize")
            .into_bytes();
        request = request.body(simple_hyper_client::SharedBody::from(body.clone()));

        request = request.header(ContentType::from(
            mimetypes::requests::GET_FORTANIX_ATTESTATION.clone(),
        ));

        let mut response = request.send().await.map_err(|e| {
            ApiError::new(
                SimpleErrorType::Permanent,
                ApiErrorDetails::Message(format!("No response received: {}", e)),
            )
        })?;
        match response.status().as_u16() {
            200 => {
                let body = simple_hyper_client::to_bytes(response.body_mut())
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Failed to read response: {}", e)),
                        )
                    })?;
                str::from_utf8(&body)
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Response was not valid UTF8: {}", e)),
                        )
                    })
                    .and_then(|body| {
                        serde_json::from_str::<models::GetFortanixAttestationResponse>(body)
                            .map_err(|e| e.into())
                    })
            }
            code => {
                let headers = response.headers().clone();
                let status = response.status();
                let result = simple_hyper_client::to_bytes(response).await;
                let err_type = match status.is_server_error() {
                    false => SimpleErrorType::Permanent,
                    true => SimpleErrorType::Temporary,
                };

                let response_body = result.ok().map(|bytes| bytes.to_vec());
                let response = ApiErrorDetails::ServerResponse(ServerErrorResponse::new(
                    code,
                    response_body,
                    headers,
                ));
                let api_err = ApiError::new(err_type, response);
                Err(api_err)
            } // error handler
        }
    } // end: fn get_fortanix_attestation

    async fn get_target_info(&self) -> Result<models::TargetInfo, ApiError> {
        let operation_path = format!("/v1/enclave/target-info");
        let mut url = format!("{}{}", self.base_path, operation_path);

        let query_string_str = {
            let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let mut request = self.hyper_client.request(Method::GET, url)?;
        request = request.headers(self.headers.clone());

        let mut response = request.send().await.map_err(|e| {
            ApiError::new(
                SimpleErrorType::Permanent,
                ApiErrorDetails::Message(format!("No response received: {}", e)),
            )
        })?;
        match response.status().as_u16() {
            200 => {
                let body = simple_hyper_client::to_bytes(response.body_mut())
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Failed to read response: {}", e)),
                        )
                    })?;
                str::from_utf8(&body)
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Response was not valid UTF8: {}", e)),
                        )
                    })
                    .and_then(|body| {
                        serde_json::from_str::<models::TargetInfo>(body).map_err(|e| e.into())
                    })
            }
            code => {
                let headers = response.headers().clone();
                let status = response.status();
                let result = simple_hyper_client::to_bytes(response).await;
                let err_type = match status.is_server_error() {
                    false => SimpleErrorType::Permanent,
                    true => SimpleErrorType::Temporary,
                };

                let response_body = result.ok().map(|bytes| bytes.to_vec());
                let response = ApiErrorDetails::ServerResponse(ServerErrorResponse::new(
                    code,
                    response_body,
                    headers,
                ));
                let api_err = ApiError::new(err_type, response);
                Err(api_err)
            } // error handler
        }
    } // end: fn get_target_info
}

#[async_trait]
impl SystemApi for Client {
    type Error = ApiError;

    async fn get_agent_version(&self) -> Result<models::VersionResponse, ApiError> {
        let operation_path = format!("/v1/sys/version");
        let mut url = format!("{}{}", self.base_path, operation_path);

        let query_string_str = {
            let mut query_string = url::form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string_str.is_empty() {
            url += "?";
            url += &query_string_str;
        }

        let mut request = self.hyper_client.request(Method::GET, url)?;
        request = request.headers(self.headers.clone());

        let mut response = request.send().await.map_err(|e| {
            ApiError::new(
                SimpleErrorType::Permanent,
                ApiErrorDetails::Message(format!("No response received: {}", e)),
            )
        })?;
        match response.status().as_u16() {
            200 => {
                let body = simple_hyper_client::to_bytes(response.body_mut())
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Failed to read response: {}", e)),
                        )
                    })?;
                str::from_utf8(&body)
                    .map_err(|e| {
                        ApiError::new(
                            SimpleErrorType::Temporary,
                            ApiErrorDetails::Message(format!("Response was not valid UTF8: {}", e)),
                        )
                    })
                    .and_then(|body| {
                        serde_json::from_str::<models::VersionResponse>(body).map_err(|e| e.into())
                    })
            }
            code => {
                let headers = response.headers().clone();
                let status = response.status();
                let result = simple_hyper_client::to_bytes(response).await;
                let err_type = match status.is_server_error() {
                    false => SimpleErrorType::Permanent,
                    true => SimpleErrorType::Temporary,
                };

                let response_body = result.ok().map(|bytes| bytes.to_vec());
                let response = ApiErrorDetails::ServerResponse(ServerErrorResponse::new(
                    code,
                    response_body,
                    headers,
                ));
                let api_err = ApiError::new(err_type, response);
                Err(api_err)
            } // error handler
        }
    } // end: fn get_agent_version
}

#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUri(UrlParseError),
    MissingHost,
}

impl From<UrlParseError> for ClientInitError {
    fn from(err: UrlParseError) -> ClientInitError {
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
