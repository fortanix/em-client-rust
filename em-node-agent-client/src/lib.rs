/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types,
    unused_qualifications
)]

extern crate base64;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(any(feature = "client"))]
#[macro_use]
extern crate hyper;
#[cfg(any(feature = "client"))]
#[macro_use]
extern crate url;

#[cfg(feature = "client")]
extern crate headers;
#[cfg(feature = "client")]
extern crate simple_hyper_client;

extern crate mime;
extern crate serde;
extern crate serde_json;

extern crate chrono;
extern crate futures;
extern crate uuid;

use async_trait::async_trait;
use futures::Stream;
use hyper::HeaderMap;
use std::error;
use std::fmt;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(feature = "client")]
mod mimetypes;

#[deprecated(note = "Import futures directly")]
pub use futures::Future;

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "1.0.0";

// Need to restore enum generation for multi-response request types
#[async_trait]
pub trait Api: Sync {
    type Error;

    /// Get result of the certificate issuance
    async fn get_issue_certificate_response(
        &self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    async fn issue_certificate(
        &self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Get Fortanix attestation for the application
    async fn get_fortanix_attestation(
        &self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    async fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error>;

    /// Get Agent Version
    async fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error>;
}

#[async_trait]
pub trait ApiMut: Send {
    type Error;

    /// Get result of the certificate issuance
    async fn get_issue_certificate_response(
        &mut self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    async fn issue_certificate(
        &mut self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Get Fortanix attestation for the application
    async fn get_fortanix_attestation(
        &mut self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    async fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error>;

    /// Get Agent Version
    async fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error>;
}

#[async_trait]
impl<T, E> Api for T
where
    T: CertificateApi<Error = E> + EnclaveApi<Error = E> + SystemApi<Error = E>,
{
    type Error = E;

    async fn get_issue_certificate_response(
        &self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.get_issue_certificate_response(task_id).await
    }

    async fn issue_certificate(
        &self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.issue_certificate(body).await
    }

    async fn get_fortanix_attestation(
        &self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        self.get_fortanix_attestation(body).await
    }

    async fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error> {
        self.get_target_info().await
    }

    async fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error> {
        self.get_agent_version().await
    }
}

#[async_trait]
impl<T, E> ApiMut for T
where
    T: CertificateApiMut<Error = E> + EnclaveApiMut<Error = E> + SystemApiMut<Error = E>,
{
    type Error = E;

    async fn get_issue_certificate_response(
        &mut self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.get_issue_certificate_response(task_id).await
    }

    async fn issue_certificate(
        &mut self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.issue_certificate(body).await
    }

    async fn get_fortanix_attestation(
        &mut self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        self.get_fortanix_attestation(body).await
    }

    async fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error> {
        self.get_target_info().await
    }

    async fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        self.get_agent_version().await
    }
}

#[async_trait]
pub trait CertificateApi: Sync {
    type Error;

    /// Get result of the certificate issuance
    async fn get_issue_certificate_response(
        &self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    async fn issue_certificate(
        &self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;
}

#[async_trait]
pub trait CertificateApiMut: Send {
    type Error;

    /// Get result of the certificate issuance
    async fn get_issue_certificate_response(
        &mut self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    async fn issue_certificate(
        &mut self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error>;
}

#[async_trait]
impl<T, E> CertificateApiMut for T
where
    T: CertificateApi<Error = E> + Send,
{
    type Error = E;

    async fn get_issue_certificate_response(
        &mut self,
        task_id: uuid::Uuid,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        <T as CertificateApi>::get_issue_certificate_response(self, task_id).await
    }

    async fn issue_certificate(
        &mut self,
        body: models::IssueCertificateRequest,
    ) -> Result<models::IssueCertificateResponse, Self::Error> {
        <T as CertificateApi>::issue_certificate(self, body).await
    }
}

#[async_trait]
pub trait EnclaveApi: Sync {
    type Error;

    /// Get Fortanix attestation for the application
    async fn get_fortanix_attestation(
        &self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    async fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error>;
}

#[async_trait]
pub trait EnclaveApiMut: Send {
    type Error;

    /// Get Fortanix attestation for the application
    async fn get_fortanix_attestation(
        &mut self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    async fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error>;
}

#[async_trait]
impl<T, E> EnclaveApiMut for T
where
    T: EnclaveApi<Error = E> + Send,
{
    type Error = E;

    async fn get_fortanix_attestation(
        &mut self,
        body: models::GetFortanixAttestationRequest,
    ) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        <T as EnclaveApi>::get_fortanix_attestation(self, body).await
    }

    async fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error> {
        <T as EnclaveApi>::get_target_info(self).await
    }
}

#[async_trait]
pub trait SystemApi: Sync {
    type Error;

    /// Get Agent Version
    async fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error>;
}

#[async_trait]
pub trait SystemApiMut: Send {
    type Error;

    /// Get Agent Version
    async fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error>;
}

#[async_trait]
impl<T, E> SystemApiMut for T
where
    T: SystemApi<Error = E> + Send,
{
    type Error = E;

    async fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        <T as SystemApi>::get_agent_version(self).await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

pub mod models;

pub mod base64_format {
    // This module from swagger-rs

    use base64::{decode, encode};
    use serde::de::{Deserialize, Deserializer, Error};
    use serde::ser::{Serialize, Serializer};
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    /// Base64-encoded byte array
    pub struct ByteArray(pub Vec<u8>);

    impl Serialize for ByteArray {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&encode(&self.0))
        }
    }

    impl<'de> Deserialize<'de> for ByteArray {
        fn deserialize<D>(deserializer: D) -> Result<ByteArray, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match decode(&s) {
                Ok(bin) => Ok(ByteArray(bin)),
                _ => Err(D::Error::custom("invalid base64")),
            }
        }
    }

    impl Deref for ByteArray {
        type Target = Vec<u8>;
        fn deref(&self) -> &Vec<u8> {
            &self.0
        }
    }

    impl DerefMut for ByteArray {
        fn deref_mut(&mut self) -> &mut Vec<u8> {
            &mut self.0
        }
    }

    impl AsRef<[u8]> for ByteArray {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }
}
pub use self::base64_format::ByteArray;


/// Very simple error type - just holds a description of the error. This is useful for human
/// diagnosis and troubleshooting, but not for applications to parse. The justification for this
/// is to deny applications visibility into the communication layer, forcing the application code
/// to act solely on the logical responses that the API provides, promoting abstraction in the
/// application code.
#[derive(Debug)]
pub struct ApiError {
    error_type: SimpleErrorType,
    details: ApiErrorDetails,
}

#[derive(Debug)]
pub enum ApiErrorDetails {
    Message(String),
    ServerResponse(ServerErrorResponse),
    ClientError(simple_hyper_client::Error)
}

impl From<simple_hyper_client::Error> for ApiError {
    fn from(e: simple_hyper_client::Error) -> ApiError {
        ApiError {
            error_type: SimpleErrorType::Temporary,
            details: ApiErrorDetails::ClientError(e),
        }
    }
}

pub struct ServerErrorResponse {
    pub status_code: u16,
    pub response_body: Option<Vec<u8>>,
    pub response_headers: HeaderMap,
}

impl fmt::Debug for ServerErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ServerErrorResponse")
         .field("status_code", &self.status_code)
         .field("response_body", &self.response_body.as_ref().map(|body| std::str::from_utf8(body)))
         .finish()
    }
}

impl ServerErrorResponse {
    pub fn new(status_code: u16, response_body: Option<Vec<u8>>, response_headers: HeaderMap) ->  ServerErrorResponse {
        ServerErrorResponse {
            status_code,
            response_body,
            response_headers,
        }
    }
}

impl ApiError {
    pub fn response(&self) -> Option<&ServerErrorResponse> {
        match &self.details {
            ApiErrorDetails::Message(_) => None,
            ApiErrorDetails::ServerResponse(response) => Some(response),
            ApiErrorDetails::ClientError(_) => None,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn error_type(&self) -> SimpleErrorType {
        self.error_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimpleErrorType {
    Temporary,
    Permanent,
}

impl ApiError {
    pub fn new(error_type: SimpleErrorType, details: ApiErrorDetails) -> ApiError {
        ApiError {
            error_type,
            details
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.details {
            ApiErrorDetails::Message(msg) => msg.fmt(f),
            ApiErrorDetails::ServerResponse(ServerErrorResponse { status_code, response_body, .. }) => {
                write!(f, "Unexpected response code {}", status_code)?;
                if let Some(body) = response_body.as_ref() {
                    f.write_str(":\n")?;
                    match std::str::from_utf8(body) {
                        Ok(body_str) => f.write_str(body_str)?,
                        Err(e) => write!(f, "<Body was not UTF8: {}>", e)?,
                    }
                }
                Ok(())
            }
            ApiErrorDetails::ClientError(inner) => {
                write!(f, "Client error: {inner}")
            },
        }
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        "Failed to produce a valid response."
    }
}

impl<'a> From<&'a str> for ApiError {
    fn from(e: &str) -> Self {
        ApiError::new(SimpleErrorType::Temporary, ApiErrorDetails::Message(e.to_string()))
    }
}

impl From<String> for ApiError {
    fn from(e: String) -> Self {
        ApiError::new(SimpleErrorType::Temporary, ApiErrorDetails::Message(e))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::new(SimpleErrorType::Temporary, ApiErrorDetails::Message(format!("Response body did not match the schema: {}", e)))
    }
}

#[derive(Debug)]
pub struct ServerError {
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    AccountIsLocked,
    BadRequest,
    Forbidden,
    Unauthorized,
    InvalidPathParameter,
    InvalidBodyParameter,
    InvalidQueryParameter,
    MissingParameter,
    NotFound,
    MethodNotAllowed,
    InvalidHeader,
}

impl ServerError {
    pub fn new(message: &str, error_type: ErrorType) -> ServerError {
        ServerError {
            message: message.to_owned(),
            error_type,
        }
    }
}

impl ::std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ErrorType::AccountIsLocked => write!(f, "AccountIsLocked"),
            ErrorType::BadRequest => write!(f, "BadRequest"),
            ErrorType::Forbidden => write!(f, "Forbidden"),
            ErrorType::Unauthorized => write!(f, "Unauthorized"),
            ErrorType::InvalidPathParameter => write!(f, "InvalidPathParameter"),
            ErrorType::InvalidBodyParameter => write!(f, "InvalidBodyParameter"),
            ErrorType::InvalidQueryParameter => write!(f, "InvalidQueryParameter"),
            ErrorType::MissingParameter => write!(f, "MissingParameter"),
            ErrorType::NotFound => write!(f, "NotFound"),
            ErrorType::MethodNotAllowed => write!(f, "MethodNotAllowed"),
            ErrorType::InvalidHeader => write!(f, "InvalidHeader"),
        }
    }
}
