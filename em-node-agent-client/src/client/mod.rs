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

use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{future, stream};
use futures::{Future, Stream};
use hyper;
use hyper::client::{Request, Response};
use hyper::header::{ContentType, Header, HeaderFormat, Headers};
use hyper::method::Method;
use hyper::Url;
use mimetypes;
use serde_json;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Arc;

#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};

use crate::ApiError;

use {CertificateApi, EnclaveApi, SystemApi};

use models;

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
        Self::try_new_with_connector(base_path, Some("http"), hyper::net::HttpConnector)
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
        S: hyper::net::NetworkStream,
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
        base_path: &str,
    ) -> Result<Client, ClientInitError> {
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

impl CertificateApi for Client {
    type Error = ApiError;

    fn get_issue_certificate_response(
        &self,
        param_task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, ApiError> {
        let mut url = format!(
            "{}/v1/certificate/result/{task_id}",
            self.base_path,
            task_id = utf8_percent_encode(&param_task_id.to_string(), ID_ENCODE_SET)
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

        request
            .send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| match response.status.to_u16() {
                200 => {
                    let mut body = Vec::new();
                    response
                        .read_to_end(&mut body)
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                    str::from_utf8(&body)
                        .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                        .and_then(|body| {
                            serde_json::from_str::<models::IssueCertificateResponse>(body)
                                .map_err(|e| e.into())
                        })
                }
                code => {
                    let headers = response.headers.clone();
                    let mut body = Vec::new();
                    let result = response.read_to_end(&mut body);
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code,
                        headers,
                        match result {
                            Ok(_) => match str::from_utf8(&body) {
                                Ok(body) => Cow::from(body),
                                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                            },
                            Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                        }
                    )))
                }
            })
    }

    fn issue_certificate(
        &self,
        param_body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, ApiError> {
        let mut url = format!("{}/v1/certificate/issue", self.base_path);

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

        request = request.header(ContentType(mimetypes::requests::ISSUE_CERTIFICATE.clone()));

        request
            .send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| match response.status.to_u16() {
                200 => {
                    let mut body = Vec::new();
                    response
                        .read_to_end(&mut body)
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                    str::from_utf8(&body)
                        .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                        .and_then(|body| {
                            serde_json::from_str::<models::IssueCertificateResponse>(body)
                                .map_err(|e| e.into())
                        })
                }
                code => {
                    let headers = response.headers.clone();
                    let mut body = Vec::new();
                    let result = response.read_to_end(&mut body);
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code,
                        headers,
                        match result {
                            Ok(_) => match str::from_utf8(&body) {
                                Ok(body) => Cow::from(body),
                                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                            },
                            Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                        }
                    )))
                }
            })
    }
}

impl EnclaveApi for Client {
    type Error = ApiError;

    fn get_fortanix_attestation(
        &self,
        param_body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, ApiError> {
        let mut url = format!("{}/v1/enclave/attest", self.base_path);

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

        request = request.header(ContentType(
            mimetypes::requests::GET_FORTANIX_ATTESTATION.clone(),
        ));

        request
            .send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| match response.status.to_u16() {
                200 => {
                    let mut body = Vec::new();
                    response
                        .read_to_end(&mut body)
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                    str::from_utf8(&body)
                        .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                        .and_then(|body| {
                            serde_json::from_str::<models::GetFortanixAttestationResponse>(body)
                                .map_err(|e| e.into())
                        })
                }
                code => {
                    let headers = response.headers.clone();
                    let mut body = Vec::new();
                    let result = response.read_to_end(&mut body);
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code,
                        headers,
                        match result {
                            Ok(_) => match str::from_utf8(&body) {
                                Ok(body) => Cow::from(body),
                                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                            },
                            Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                        }
                    )))
                }
            })
    }

    fn get_target_info(&self) -> Result<models::TargetInfo, ApiError> {
        let mut url = format!("{}/v1/enclave/target-info", self.base_path);

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

        request
            .send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| match response.status.to_u16() {
                200 => {
                    let mut body = Vec::new();
                    response
                        .read_to_end(&mut body)
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                    str::from_utf8(&body)
                        .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                        .and_then(|body| {
                            serde_json::from_str::<models::TargetInfo>(body).map_err(|e| e.into())
                        })
                }
                code => {
                    let headers = response.headers.clone();
                    let mut body = Vec::new();
                    let result = response.read_to_end(&mut body);
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code,
                        headers,
                        match result {
                            Ok(_) => match str::from_utf8(&body) {
                                Ok(body) => Cow::from(body),
                                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                            },
                            Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                        }
                    )))
                }
            })
    }
}

impl SystemApi for Client {
    type Error = ApiError;

    fn get_agent_version(&self) -> Result<models::VersionResponse, ApiError> {
        let mut url = format!("{}/v1/sys/version", self.base_path);

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

        request
            .send()
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|mut response| match response.status.to_u16() {
                200 => {
                    let mut body = Vec::new();
                    response
                        .read_to_end(&mut body)
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e)))?;

                    str::from_utf8(&body)
                        .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))
                        .and_then(|body| {
                            serde_json::from_str::<models::VersionResponse>(body)
                                .map_err(|e| e.into())
                        })
                }
                code => {
                    let headers = response.headers.clone();
                    let mut body = Vec::new();
                    let result = response.read_to_end(&mut body);
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code,
                        headers,
                        match result {
                            Ok(_) => match str::from_utf8(&body) {
                                Ok(body) => Cow::from(body),
                                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                            },
                            Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                        }
                    )))
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
