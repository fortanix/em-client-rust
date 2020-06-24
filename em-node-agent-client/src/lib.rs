/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types, unused_qualifications)]

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


extern crate mime;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate chrono;
extern crate uuid;

use futures::Stream;
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

pub trait Api {
    type Error;


    /// Get result of the certificate issuance
    fn get_issue_certificate_response(&self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    fn issue_certificate(&self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error>;



    /// Get Fortanix attestation for the application
    fn get_fortanix_attestation(&self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error>;



    /// Get Agent Version
    fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error>;


}

pub trait ApiMut {
    type Error;


    /// Get result of the certificate issuance
    fn get_issue_certificate_response(&mut self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    fn issue_certificate(&mut self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error>;



    /// Get Fortanix attestation for the application
    fn get_fortanix_attestation(&mut self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error>;



    /// Get Agent Version
    fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error>;


}

impl<T, E> Api for T
where
T: CertificateApi<Error = E> + EnclaveApi<Error = E> + SystemApi<Error = E> + 
{
type Error = E;
    
        fn get_issue_certificate_response(&self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.get_issue_certificate_response(task_id, )
        }
    
        fn issue_certificate(&self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.issue_certificate(body, )
        }
    

    
        fn get_fortanix_attestation(&self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        self.get_fortanix_attestation(body, )
        }
    
        fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error> {
        self.get_target_info()
        }
    

    
        fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error> {
        self.get_agent_version()
        }
    

}

impl<T, E> ApiMut for T
where
    T: CertificateApiMut<Error = E> + EnclaveApiMut<Error = E> + SystemApiMut<Error = E> + 
{
    type Error = E;




    fn get_issue_certificate_response(&mut self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.get_issue_certificate_response(task_id, )
    }

    fn issue_certificate(&mut self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.issue_certificate(body, )
    }



    fn get_fortanix_attestation(&mut self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        self.get_fortanix_attestation(body, )
    }

    fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error> {
        self.get_target_info()
    }



    fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        self.get_agent_version()
    }


}

impl<T, E> Api for std::cell::RefCell<T>
where
    T: ApiMut<Error = E>,
{
    type Error = E;



    fn get_issue_certificate_response(&self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.borrow_mut().get_issue_certificate_response(task_id, )
    }

    fn issue_certificate(&self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error> {
        self.borrow_mut().issue_certificate(body, )
    }



    fn get_fortanix_attestation(&self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        self.borrow_mut().get_fortanix_attestation(body, )
    }

    fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error> {
        self.borrow_mut().get_target_info()
    }



    fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error> {
        self.borrow_mut().get_agent_version()
    }


}

pub trait CertificateApi {
    type Error;


    /// Get result of the certificate issuance
    fn get_issue_certificate_response(&self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    fn issue_certificate(&self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error>;

}

pub trait CertificateApiMut {
    type Error;


    /// Get result of the certificate issuance
    fn get_issue_certificate_response(&mut self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error>;

    /// Submit request for certificate issuance
    fn issue_certificate(&mut self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error>;

}

impl<T, E> CertificateApiMut for T
where
    T: CertificateApi<Error = E>,
{
    type Error = E;

    fn get_issue_certificate_response(&mut self, task_id: uuid::Uuid) -> Result<models::IssueCertificateResponse, Self::Error> {
        <T as CertificateApi>::get_issue_certificate_response(self, task_id, )
    }

    fn issue_certificate(&mut self, body: models::IssueCertificateRequest) -> Result<models::IssueCertificateResponse, Self::Error> {
        <T as CertificateApi>::issue_certificate(self, body, )
    }

}


pub trait EnclaveApi {
    type Error;


    /// Get Fortanix attestation for the application
    fn get_fortanix_attestation(&self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    fn get_target_info(&self) -> Result<models::TargetInfo, Self::Error>;

}

pub trait EnclaveApiMut {
    type Error;


    /// Get Fortanix attestation for the application
    fn get_fortanix_attestation(&mut self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error>;

    /// Get Target Info for node provisioning enclave
    fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error>;

}

impl<T, E> EnclaveApiMut for T
where
    T: EnclaveApi<Error = E>,
{
    type Error = E;

    fn get_fortanix_attestation(&mut self, body: models::GetFortanixAttestationRequest) -> Result<models::GetFortanixAttestationResponse, Self::Error> {
        <T as EnclaveApi>::get_fortanix_attestation(self, body, )
    }

    fn get_target_info(&mut self) -> Result<models::TargetInfo, Self::Error> {
        <T as EnclaveApi>::get_target_info(self, )
    }

}


pub trait SystemApi {
    type Error;


    /// Get Agent Version
    fn get_agent_version(&self) -> Result<models::VersionResponse, Self::Error>;

}

pub trait SystemApiMut {
    type Error;


    /// Get Agent Version
    fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error>;

}

impl<T, E> SystemApiMut for T
where
    T: SystemApi<Error = E>,
{
    type Error = E;

    fn get_agent_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        <T as SystemApi>::get_agent_version(self, )
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
pub use base64_format::ByteArray;


/// Very simple error type - just holds a description of the error. This is useful for human
/// diagnosis and troubleshooting, but not for applications to parse. The justification for this
/// is to deny applications visibility into the communication layer, forcing the application code
/// to act solely on the logical responses that the API provides, promoting abstraction in the
/// application code.
#[derive(Clone, Debug)]
pub struct ApiError(pub String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug: &dyn fmt::Debug = self;
        debug.fmt(f)
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        "Failed to produce a valid response."
    }
}

impl<'a> From<&'a str> for ApiError {
    fn from(e: &str) -> Self {
        ApiError(e.to_string())
    }
}

impl From<String> for ApiError {
    fn from(e: String) -> Self {
        ApiError(e)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError(format!("Response body did not match the schema: {}", e))
    }
}
