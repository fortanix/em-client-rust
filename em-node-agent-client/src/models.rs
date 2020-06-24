/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;

use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use std::string::ParseError;
use uuid;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AgentManagerAuthRequest {
    /// Node IP
    #[serde(rename = "node_ip")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_ip: Option<String>,

    /// Node Name
    #[serde(rename = "node_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_name: Option<String>,

}

impl AgentManagerAuthRequest {
    pub fn new() -> AgentManagerAuthRequest {
        AgentManagerAuthRequest {
            node_ip: None,
            node_name: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AgentManagerAuthResponse {
    /// Access token for node
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}

impl AgentManagerAuthResponse {
    pub fn new() -> AgentManagerAuthResponse {
        AgentManagerAuthResponse {
            access_token: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppHeartbeatRequest {
    /// Application Heartbeat csr
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// Node Id for app
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<uuid::Uuid>,

}

impl AppHeartbeatRequest {
    pub fn new() -> AppHeartbeatRequest {
        AppHeartbeatRequest {
            csr: None,
            node_id: None,
        }
    }
}


/// App Heartbeat Response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppHeartbeatResponse {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::ResponseStatus>,

}

impl AppHeartbeatResponse {
    pub fn new() -> AppHeartbeatResponse {
        AppHeartbeatResponse {
            status: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetFortanixAttestationRequest {
    /// Enclave Report bytes
    #[serde(rename = "report")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub report: Option<String>,

    /// Fortanix attestation CSR
    #[serde(rename = "attestation_csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attestation_csr: Option<String>,

}

impl GetFortanixAttestationRequest {
    pub fn new() -> GetFortanixAttestationRequest {
        GetFortanixAttestationRequest {
            report: None,
            attestation_csr: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetFortanixAttestationResponse {
    /// Verified attestation certificate
    #[serde(rename = "attestation_certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attestation_certificate: Option<String>,

    /// Node Certificate
    #[serde(rename = "node_certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_certificate: Option<String>,

    /// FQPE Report bytes
    #[serde(rename = "fqpe_report")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fqpe_report: Option<String>,

}

impl GetFortanixAttestationResponse {
    pub fn new() -> GetFortanixAttestationResponse {
        GetFortanixAttestationResponse {
            attestation_certificate: None,
            node_certificate: None,
            fqpe_report: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct IssueCertificateRequest {
    /// Application CSR
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

}

impl IssueCertificateRequest {
    pub fn new() -> IssueCertificateRequest {
        IssueCertificateRequest {
            csr: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct IssueCertificateResponse {
    /// Task Id
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_id: Option<uuid::Uuid>,

    #[serde(rename = "task_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_status: Option<models::TaskStatusType>,

    /// App Certificate
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,

}

impl IssueCertificateResponse {
    pub fn new() -> IssueCertificateResponse {
        IssueCertificateResponse {
            task_id: None,
            task_status: None,
            certificate: None,
        }
    }
}


/// Node local data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeLocalData {
    /// Node id
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<uuid::Uuid>,

    /// node certificate
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,

}

impl NodeLocalData {
    pub fn new() -> NodeLocalData {
        NodeLocalData {
            node_id: None,
            certificate: None,
        }
    }
}


/// Status string for a response
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum ResponseStatus { 
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "NOT_OK")]
    NOT_OK,
}

impl ::std::fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            ResponseStatus::OK => write!(f, "{}", "OK"),
            ResponseStatus::NOT_OK => write!(f, "{}", "NOT_OK"),
        }
    }
}

impl ::std::str::FromStr for ResponseStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OK" => Ok(ResponseStatus::OK),
            "NOT_OK" => Ok(ResponseStatus::NOT_OK),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TargetInfo {
    /// Enclave Target Info
    #[serde(rename = "targetInfo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_info: Option<String>,

}

impl TargetInfo {
    pub fn new() -> TargetInfo {
        TargetInfo {
            target_info: None,
        }
    }
}


/// Status string for a task
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum TaskStatusType { 
    #[serde(rename = "INPROGRESS")]
    INPROGRESS,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "DENIED")]
    DENIED,
    #[serde(rename = "PENDING_WHITELIST")]
    PENDING_WHITELIST,
}

impl ::std::fmt::Display for TaskStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            TaskStatusType::INPROGRESS => write!(f, "{}", "INPROGRESS"),
            TaskStatusType::FAILED => write!(f, "{}", "FAILED"),
            TaskStatusType::SUCCESS => write!(f, "{}", "SUCCESS"),
            TaskStatusType::DENIED => write!(f, "{}", "DENIED"),
            TaskStatusType::PENDING_WHITELIST => write!(f, "{}", "PENDING_WHITELIST"),
        }
    }
}

impl ::std::str::FromStr for TaskStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INPROGRESS" => Ok(TaskStatusType::INPROGRESS),
            "FAILED" => Ok(TaskStatusType::FAILED),
            "SUCCESS" => Ok(TaskStatusType::SUCCESS),
            "DENIED" => Ok(TaskStatusType::DENIED),
            "PENDING_WHITELIST" => Ok(TaskStatusType::PENDING_WHITELIST),
            _ => Err(()),
        }
    }
}


/// Agent Version
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct VersionResponse {
    /// Agent Version
    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

}

impl VersionResponse {
    pub fn new() -> VersionResponse {
        VersionResponse {
            version: None,
        }
    }
}

