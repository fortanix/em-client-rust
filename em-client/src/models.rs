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






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum AccessRoles { 
    #[serde(rename = "READER")]
    READER,
    #[serde(rename = "WRITER")]
    WRITER,
    #[serde(rename = "MANAGER")]
    MANAGER,
}

impl ::std::fmt::Display for AccessRoles {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            AccessRoles::READER => write!(f, "{}", "READER"),
            AccessRoles::WRITER => write!(f, "{}", "WRITER"),
            AccessRoles::MANAGER => write!(f, "{}", "MANAGER"),
        }
    }
}

impl ::std::str::FromStr for AccessRoles {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "READER" => Ok(AccessRoles::READER),
            "WRITER" => Ok(AccessRoles::WRITER),
            "MANAGER" => Ok(AccessRoles::MANAGER),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Account {
    /// Name of the account. Account names must be unique within an EM instance.
    #[serde(rename = "name")]
    pub name: String,

    /// Account ID uniquely identifying this account.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// When this account was created.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// role of the current user in particular account
    #[serde(rename = "roles")]
    pub roles: Vec<models::AccessRoles>,

    /// logo of the particular account. Max size 128Kb, .jpg, .png, .svg file formats only
    #[serde(rename = "custom_logo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_logo: Option<crate::ByteArray>,

    #[serde(rename = "status")]
    pub status: models::UserAccountStatus,

}

impl Account {
    pub fn new(name: String, acct_id: uuid::Uuid, created_at: i64, roles: Vec<models::AccessRoles>, status: models::UserAccountStatus, ) -> Account {
        Account {
            name: name,
            acct_id: acct_id,
            created_at: created_at,
            roles: roles,
            custom_logo: None,
            status: status,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AccountListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<models::Account>>,

}

impl AccountListResponse {
    pub fn new() -> AccountListResponse {
        AccountListResponse {
            items: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AccountRequest {
    /// Name of the account. Accounts must be unique within an EM instance.
    #[serde(rename = "name")]
    pub name: String,

    /// logo for an account. Max size 128Kb, .jpg, .png, .svg file formats only
    #[serde(rename = "custom_logo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_logo: Option<crate::ByteArray>,

}

impl AccountRequest {
    pub fn new(name: String, ) -> AccountRequest {
        AccountRequest {
            name: name,
            custom_logo: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AccountUpdateRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "custom_logo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_logo: Option<crate::ByteArray>,

}

impl AccountUpdateRequest {
    pub fn new() -> AccountUpdateRequest {
        AccountUpdateRequest {
            name: None,
            custom_logo: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AddZoneRequest {
    /// Name for the new zone
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Description of the new zone
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl AddZoneRequest {
    pub fn new() -> AddZoneRequest {
        AddZoneRequest {
            name: None,
            description: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AdvancedSettings {
    /// Entrypoint for the container
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entrypoint: Option<Vec<String>>,

    /// Filesystem directories to encrypt using enclave sealing key
    #[serde(rename = "encryptedDirs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_dirs: Option<Vec<String>>,

    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<models::CertificateConfig>,

    #[serde(rename = "caCertificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_certificate: Option<models::CaCertificateConfig>,

    #[serde(rename = "java_runtime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub java_runtime: Option<models::JavaRuntime>,

    /// Filesystem directories to enable read write
    #[serde(rename = "rw_dirs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rw_dirs: Option<Vec<String>>,

    /// allow command line arguments converter flag for an image
    #[serde(rename = "allowCmdlineArgs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_cmdline_args: Option<bool>,

    /// Environment variables that will be passed to the manifest file when the container is converted
    #[serde(rename = "manifestEnv")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manifest_env: Option<Vec<String>>,

}

impl AdvancedSettings {
    pub fn new() -> AdvancedSettings {
        AdvancedSettings {
            entrypoint: None,
            encrypted_dirs: None,
            certificate: None,
            ca_certificate: None,
            java_runtime: None,
            rw_dirs: None,
            allow_cmdline_args: None,
            manifest_env: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct App {
    /// Timestamp of build addition to the system
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Timestamp of build updation to the system
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<i64>,

    /// Name of the app
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the app
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// UUID for the app
    #[serde(rename = "app_id")]
    pub app_id: uuid::Uuid,

    /// Input image name of builds for apps
    #[serde(rename = "input_image_name")]
    pub input_image_name: String,

    /// Output image name of builds for apps
    #[serde(rename = "output_image_name")]
    pub output_image_name: String,

    /// IsvProdId
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// Mem size required for the build
    #[serde(rename = "mem_size")]
    pub mem_size: i64,

    /// Threads req for the build
    #[serde(rename = "threads")]
    pub threads: i32,

    #[serde(rename = "allowed_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(rename = "whitelisted_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub whitelisted_domains: Option<Vec<String>>,

    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nodes: Option<Vec<models::AppNodeInfo>>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

    /// no of domain whitelist tasks pending for app
    #[serde(rename = "pending_domain_whitelist_tasks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_domain_whitelist_tasks: Option<i32>,

    #[serde(rename = "domains_added")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_added: Option<Vec<String>>,

    #[serde(rename = "domains_removed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_removed: Option<Vec<String>>,

}

impl App {
    pub fn new(name: String, app_id: uuid::Uuid, input_image_name: String, output_image_name: String, isvprodid: i32, isvsvn: i32, mem_size: i64, threads: i32, ) -> App {
        App {
            created_at: None,
            updated_at: None,
            name: name,
            description: None,
            app_id: app_id,
            input_image_name: input_image_name,
            output_image_name: output_image_name,
            isvprodid: isvprodid,
            isvsvn: isvsvn,
            mem_size: mem_size,
            threads: threads,
            allowed_domains: None,
            whitelisted_domains: None,
            nodes: None,
            advanced_settings: None,
            pending_domain_whitelist_tasks: None,
            domains_added: None,
            domains_removed: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppBodyUpdateRequest {
    /// Description of the app
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Input image name of builds for apps
    #[serde(rename = "input_image_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_image_name: Option<String>,

    /// Output image name of builds for apps
    #[serde(rename = "output_image_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_image_name: Option<String>,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub isvsvn: Option<i32>,

    /// Mem size required for the build
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads req for the build
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    #[serde(rename = "allowed_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

}

impl AppBodyUpdateRequest {
    pub fn new() -> AppBodyUpdateRequest {
        AppBodyUpdateRequest {
            description: None,
            input_image_name: None,
            output_image_name: None,
            isvsvn: None,
            mem_size: None,
            threads: None,
            allowed_domains: None,
            advanced_settings: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppHeartbeatRequest {
    /// App heartbeat csr
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// Node Id for the requesting host agent
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



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppNodeInfo {
    #[serde(rename = "certificate")]
    pub certificate: models::Certificate,

    /// App node creation time
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Node Id
    #[serde(rename = "node_id")]
    pub node_id: uuid::Uuid,

    /// Node name
    #[serde(rename = "node_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::AppStatus>,

    #[serde(rename = "build_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_info: Option<models::Build>,

    /// App heartbeat message count
    #[serde(rename = "message_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_count: Option<i32>,

    /// Key Id for app heartbeat
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    /// App running in debug mode or not
    #[serde(rename = "is_debug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_debug: Option<bool>,

}

impl AppNodeInfo {
    pub fn new(certificate: models::Certificate, created_at: i64, node_id: uuid::Uuid, ) -> AppNodeInfo {
        AppNodeInfo {
            certificate: certificate,
            created_at: created_at,
            node_id: node_id,
            node_name: None,
            status: None,
            build_info: None,
            message_count: None,
            key_id: None,
            is_debug: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppRequest {
    /// Name of the app
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the app
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Input image name of builds for apps
    #[serde(rename = "input_image_name")]
    pub input_image_name: String,

    /// Output image name of builds for apps
    #[serde(rename = "output_image_name")]
    pub output_image_name: String,

    /// IsvProdId
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// Mem size required for the build
    #[serde(rename = "mem_size")]
    pub mem_size: i64,

    /// Threads req for the build
    #[serde(rename = "threads")]
    pub threads: i32,

    #[serde(rename = "allowed_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

}

impl AppRequest {
    pub fn new(name: String, input_image_name: String, output_image_name: String, isvprodid: i32, isvsvn: i32, mem_size: i64, threads: i32, ) -> AppRequest {
        AppRequest {
            name: name,
            description: None,
            input_image_name: input_image_name,
            output_image_name: output_image_name,
            isvprodid: isvprodid,
            isvsvn: isvsvn,
            mem_size: mem_size,
            threads: threads,
            allowed_domains: None,
            advanced_settings: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppStatus {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::AppStatusType>,

    /// Time since the status change
    #[serde(rename = "status_updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_updated_at: Option<i64>,

    /// The app attestation date
    #[serde(rename = "attested_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attested_at: Option<i64>,

}

impl AppStatus {
    pub fn new() -> AppStatus {
        AppStatus {
            status: None,
            status_updated_at: None,
            attested_at: None,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum AppStatusType { 
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "STOPPED")]
    STOPPED,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

impl ::std::fmt::Display for AppStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            AppStatusType::RUNNING => write!(f, "{}", "RUNNING"),
            AppStatusType::STOPPED => write!(f, "{}", "STOPPED"),
            AppStatusType::UNKNOWN => write!(f, "{}", "UNKNOWN"),
        }
    }
}

impl ::std::str::FromStr for AppStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RUNNING" => Ok(AppStatusType::RUNNING),
            "STOPPED" => Ok(AppStatusType::STOPPED),
            "UNKNOWN" => Ok(AppStatusType::UNKNOWN),
            _ => Err(()),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppUpdateRequest {
    #[serde(rename = "status")]
    pub status: models::AppStatusType,

}

impl AppUpdateRequest {
    pub fn new(status: models::AppStatusType, ) -> AppUpdateRequest {
        AppUpdateRequest {
            status: status,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApprovalInfo {
    /// User Id
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,

    /// User Name
    #[serde(rename = "user_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::ApprovalStatus>,

}

impl ApprovalInfo {
    pub fn new(user_id: uuid::Uuid, ) -> ApprovalInfo {
        ApprovalInfo {
            user_id: user_id,
            user_name: None,
            status: None,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum ApprovalStatus { 
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "DENIED")]
    DENIED,
}

impl ::std::fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            ApprovalStatus::APPROVED => write!(f, "{}", "APPROVED"),
            ApprovalStatus::DENIED => write!(f, "{}", "DENIED"),
        }
    }
}

impl ::std::str::FromStr for ApprovalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "APPROVED" => Ok(ApprovalStatus::APPROVED),
            "DENIED" => Ok(ApprovalStatus::DENIED),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AttestationRequest {
    /// IAS Quote report bytes
    #[serde(rename = "ias_quote")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ias_quote: Option<crate::ByteArray>,

    /// Certificate Signing Request bytes
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

}

impl AttestationRequest {
    pub fn new() -> AttestationRequest {
        AttestationRequest {
            ias_quote: None,
            csr: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuditLog {
    /// Log Entry Id
    #[serde(rename = "log_id")]
    pub log_id: uuid::Uuid,

    /// Zone Id
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zone_id: Option<uuid::Uuid>,

    #[serde(rename = "severity")]
    pub severity: models::EventSeverity,

    /// Description of the event
    #[serde(rename = "description")]
    pub description: String,

    /// Event timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: i64,

    #[serde(rename = "action_type")]
    pub action_type: models::EventActionType,

    #[serde(rename = "actor_type")]
    pub actor_type: models::EventActorType,

    /// User Id, if actor is a user
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,

    /// User Name, if actor is a user
    #[serde(rename = "user_name")]
    pub user_name: String,

    /// App Id, if actor is an app
    #[serde(rename = "app_id")]
    pub app_id: uuid::Uuid,

    /// App Name, if actor is an app
    #[serde(rename = "app_name")]
    pub app_name: String,

    /// Node Id, if associated with the event
    #[serde(rename = "node_id")]
    pub node_id: uuid::Uuid,

    /// Node Name, if associated with the event
    #[serde(rename = "node_name")]
    pub node_name: String,

}

impl AuditLog {
    pub fn new(log_id: uuid::Uuid, severity: models::EventSeverity, description: String, timestamp: i64, action_type: models::EventActionType, actor_type: models::EventActorType, user_id: uuid::Uuid, user_name: String, app_id: uuid::Uuid, app_name: String, node_id: uuid::Uuid, node_name: String, ) -> AuditLog {
        AuditLog {
            log_id: log_id,
            zone_id: None,
            severity: severity,
            description: description,
            timestamp: timestamp,
            action_type: action_type,
            actor_type: actor_type,
            user_id: user_id,
            user_name: user_name,
            app_id: app_id,
            app_name: app_name,
            node_id: node_id,
            node_name: node_name,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthConfig {
    /// User name for docker registry authentication
    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    /// Password for docker registry authentication
    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

}

impl AuthConfig {
    pub fn new() -> AuthConfig {
        AuthConfig {
            username: None,
            password: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthResponse {
    /// Bearer token to be used to authenticate to other APIs
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}

impl AuthResponse {
    pub fn new() -> AuthResponse {
        AuthResponse {
            access_token: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct BackendNodeProvisionRequest {
    /// Url of the Fortanix IAS proxy. 'ftx-proxy' corresponds to the preset option. Otherwise any other url can be provided
    #[serde(rename = "ias_url")]
    pub ias_url: String,

    /// IAS SPID
    #[serde(rename = "ias_spid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ias_spid: Option<String>,

    /// IAS Proxy url
    #[serde(rename = "ias_proxy_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ias_proxy_url: Option<String>,

}

impl BackendNodeProvisionRequest {
    pub fn new(ias_url: String, ) -> BackendNodeProvisionRequest {
        BackendNodeProvisionRequest {
            ias_url: ias_url,
            ias_spid: None,
            ias_proxy_url: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Build {
    /// Build Id
    #[serde(rename = "build_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<uuid::Uuid>,

    #[serde(rename = "docker_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_info: Option<models::DockerInfo>,

    /// Timestamp of build addition to the system
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Timestamp of build updation to the system
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<i64>,

    /// App Id
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<uuid::Uuid>,

    /// App name
    #[serde(rename = "app_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_name: Option<String>,

    #[serde(rename = "status")]
    pub status: models::BuildStatus,

    #[serde(rename = "deployment_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_status: Option<models::BuildDeploymentStatus>,

    #[serde(rename = "enclave_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enclave_info: Option<models::EnclaveInfo>,

    /// App Description
    #[serde(rename = "app_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_description: Option<String>,

    /// Mem size required for the build
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads req for the build
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

    /// Build name if curated app
    #[serde(rename = "build_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_name: Option<String>,

}

impl Build {
    pub fn new(status: models::BuildStatus, ) -> Build {
        Build {
            build_id: None,
            docker_info: None,
            created_at: None,
            updated_at: None,
            app_id: None,
            app_name: None,
            status: status,
            deployment_status: None,
            enclave_info: None,
            app_description: None,
            mem_size: None,
            threads: None,
            advanced_settings: None,
            build_name: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct BuildDeploymentStatus {
    #[serde(rename = "status")]
    pub status: models::BuildDeploymentStatusType,

    /// when was the deployment status changed
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: i64,

}

impl BuildDeploymentStatus {
    pub fn new(status: models::BuildDeploymentStatusType, status_updated_at: i64, ) -> BuildDeploymentStatus {
        BuildDeploymentStatus {
            status: status,
            status_updated_at: status_updated_at,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum BuildDeploymentStatusType { 
    #[serde(rename = "DEPLOYED")]
    DEPLOYED,
    #[serde(rename = "UNDEPLOYED")]
    UNDEPLOYED,
}

impl ::std::fmt::Display for BuildDeploymentStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            BuildDeploymentStatusType::DEPLOYED => write!(f, "{}", "DEPLOYED"),
            BuildDeploymentStatusType::UNDEPLOYED => write!(f, "{}", "UNDEPLOYED"),
        }
    }
}

impl ::std::str::FromStr for BuildDeploymentStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPLOYED" => Ok(BuildDeploymentStatusType::DEPLOYED),
            "UNDEPLOYED" => Ok(BuildDeploymentStatusType::UNDEPLOYED),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct BuildStatus {
    #[serde(rename = "status")]
    pub status: models::BuildStatusType,

    /// Time since the status change
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: i64,

}

impl BuildStatus {
    pub fn new(status: models::BuildStatusType, status_updated_at: i64, ) -> BuildStatus {
        BuildStatus {
            status: status,
            status_updated_at: status_updated_at,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum BuildStatusType { 
    #[serde(rename = "REJECTED")]
    REJECTED,
    #[serde(rename = "WHITELISTED")]
    WHITELISTED,
    #[serde(rename = "PENDING")]
    PENDING,
}

impl ::std::fmt::Display for BuildStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            BuildStatusType::REJECTED => write!(f, "{}", "REJECTED"),
            BuildStatusType::WHITELISTED => write!(f, "{}", "WHITELISTED"),
            BuildStatusType::PENDING => write!(f, "{}", "PENDING"),
        }
    }
}

impl ::std::str::FromStr for BuildStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REJECTED" => Ok(BuildStatusType::REJECTED),
            "WHITELISTED" => Ok(BuildStatusType::WHITELISTED),
            "PENDING" => Ok(BuildStatusType::PENDING),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CaCertificateConfig {
    /// Path to expose the CA cert in the application filesystem
    #[serde(rename = "caPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_path: Option<String>,

    /// Base64-encoded CA certificate contents. Not required when converting applications via Enclave Manager. Required when calling the converter directly, or if you wish to override the Enclave Manager CA certificate. 
    #[serde(rename = "caCert")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_cert: Option<String>,

    /// Request to install CA cert in the system trust store
    #[serde(rename = "system")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub system: Option<String>,

}

impl CaCertificateConfig {
    pub fn new() -> CaCertificateConfig {
        CaCertificateConfig {
            ca_path: None,
            ca_cert: None,
            system: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Certificate {
    /// Certificate ID
    #[serde(rename = "certificate_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<uuid::Uuid>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::CertificateStatusType>,

    /// The certificate signing request
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// The certificate itself, if issued
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,

}

impl Certificate {
    pub fn new() -> Certificate {
        Certificate {
            certificate_id: None,
            status: None,
            csr: None,
            certificate: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CertificateConfig {
    /// Certificate issuance strategy
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issuer: Option<String>,

    /// Certificate subject common name, typically a DNS name
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subject: Option<String>,

    /// Subject alternate names to include in the certificate (e.g. DNS:example.com)
    #[serde(rename = "altNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alt_names: Option<Vec<String>>,

    /// Type of key to generate
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "keyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_type: Option<String>,

    /// Key parameters. Currently must be an instance of RsaKeyParam, but other types may be supported in the future. 
    #[serde(rename = "keyParam")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_param: Option<serde_json::Value>,

    /// Path to expose the key in the application filesystem
    #[serde(rename = "keyPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_path: Option<String>,

    /// Path to expose the certificate in the application filesystem
    #[serde(rename = "certPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cert_path: Option<String>,

    /// Path to expose the complete certificate chain in the application filesystem
    #[serde(rename = "chainPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub chain_path: Option<String>,

}

impl CertificateConfig {
    pub fn new() -> CertificateConfig {
        CertificateConfig {
            issuer: Some("MANAGER_CA".to_string()),
            subject: None,
            alt_names: None,
            key_type: Some("RSA".to_string()),
            key_param: None,
            key_path: None,
            cert_path: None,
            chain_path: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CertificateDetails {
    #[serde(rename = "enclave_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enclave_info: Option<models::EnclaveInfo>,

    /// name of the subject
    #[serde(rename = "subject_name")]
    pub subject_name: String,

    /// name of the issuer
    #[serde(rename = "issuer_name")]
    pub issuer_name: String,

    /// certificate expiry date
    #[serde(rename = "valid_until")]
    pub valid_until: i64,

    /// certificate valid from
    #[serde(rename = "valid_from")]
    pub valid_from: i64,

    /// cpusvn, as a hex string
    #[serde(rename = "cpusvn")]
    pub cpusvn: String,

    /// ias quote status
    #[serde(rename = "ias_quote_status")]
    pub ias_quote_status: String,

}

impl CertificateDetails {
    pub fn new(subject_name: String, issuer_name: String, valid_until: i64, valid_from: i64, cpusvn: String, ias_quote_status: String, ) -> CertificateDetails {
        CertificateDetails {
            enclave_info: None,
            subject_name: subject_name,
            issuer_name: issuer_name,
            valid_until: valid_until,
            valid_from: valid_from,
            cpusvn: cpusvn,
            ias_quote_status: ias_quote_status,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum CertificateStatusType { 
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "REJECTED")]
    REJECTED,
    #[serde(rename = "ISSUED")]
    ISSUED,
    #[serde(rename = "REVOKED")]
    REVOKED,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
}

impl ::std::fmt::Display for CertificateStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            CertificateStatusType::PENDING => write!(f, "{}", "PENDING"),
            CertificateStatusType::REJECTED => write!(f, "{}", "REJECTED"),
            CertificateStatusType::ISSUED => write!(f, "{}", "ISSUED"),
            CertificateStatusType::REVOKED => write!(f, "{}", "REVOKED"),
            CertificateStatusType::EXPIRED => write!(f, "{}", "EXPIRED"),
        }
    }
}

impl ::std::str::FromStr for CertificateStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING" => Ok(CertificateStatusType::PENDING),
            "REJECTED" => Ok(CertificateStatusType::REJECTED),
            "ISSUED" => Ok(CertificateStatusType::ISSUED),
            "REVOKED" => Ok(CertificateStatusType::REVOKED),
            "EXPIRED" => Ok(CertificateStatusType::EXPIRED),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ClusterCsrRequest {
    /// Dictionary of (OID, value) items, describing the subject of the CSR
    #[serde(rename = "subject")]
    pub subject: HashMap<String, String>,

    /// List of subjectAltName values for the CSR
    #[serde(rename = "subject_alternative_dns_names")]
    pub subject_alternative_dns_names: Vec<String>,

}

impl ClusterCsrRequest {
    pub fn new(subject: HashMap<String, String>, subject_alternative_dns_names: Vec<String>, ) -> ClusterCsrRequest {
        ClusterCsrRequest {
            subject: subject,
            subject_alternative_dns_names: subject_alternative_dns_names,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ClusterCsrResponse {
    /// DER-encoded certificate signing request
    #[serde(rename = "csr")]
    pub csr: crate::ByteArray,

}

impl ClusterCsrResponse {
    pub fn new(csr: crate::ByteArray, ) -> ClusterCsrResponse {
        ClusterCsrResponse {
            csr: csr,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum ClusterState { 
    #[serde(rename = "NOCLUSTER")]
    NOCLUSTER,
    #[serde(rename = "ZONENODE")]
    ZONENODE,
    #[serde(rename = "NOZONENODE")]
    NOZONENODE,
    #[serde(rename = "ERROR")]
    ERROR,
}

impl ::std::fmt::Display for ClusterState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            ClusterState::NOCLUSTER => write!(f, "{}", "NOCLUSTER"),
            ClusterState::ZONENODE => write!(f, "{}", "ZONENODE"),
            ClusterState::NOZONENODE => write!(f, "{}", "NOZONENODE"),
            ClusterState::ERROR => write!(f, "{}", "ERROR"),
        }
    }
}

impl ::std::str::FromStr for ClusterState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NOCLUSTER" => Ok(ClusterState::NOCLUSTER),
            "ZONENODE" => Ok(ClusterState::ZONENODE),
            "NOZONENODE" => Ok(ClusterState::NOZONENODE),
            "ERROR" => Ok(ClusterState::ERROR),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConfigureClusterRequest {
    /// DER-encoded certificate chain for the cluster's public client interface
    #[serde(rename = "public_if_cert_chain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_if_cert_chain: Option<Vec<crate::ByteArray>>,

}

impl ConfigureClusterRequest {
    pub fn new() -> ConfigureClusterRequest {
        ConfigureClusterRequest {
            public_if_cert_chain: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConfirmEmailRequest {
    #[serde(rename = "confirm_token")]
    pub confirm_token: String,

}

impl ConfirmEmailRequest {
    pub fn new(confirm_token: String, ) -> ConfirmEmailRequest {
        ConfirmEmailRequest {
            confirm_token: confirm_token,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConfirmEmailResponse {
    #[serde(rename = "user_email")]
    pub user_email: String,

}

impl ConfirmEmailResponse {
    pub fn new(user_email: String, ) -> ConfirmEmailResponse {
        ConfirmEmailResponse {
            user_email: user_email,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConversionRequest {
    /// Registry and image name for the input container, e.g. my-registry/sample-app:latest
    #[serde(rename = "inputImageName")]
    pub input_image_name: String,

    /// Registry and image name for the output container, e.g. my-registry/sample-app-enclaveos:latest
    #[serde(rename = "outputImageName")]
    pub output_image_name: String,

    #[serde(rename = "inputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "outputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "authConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_config: Option<models::AuthConfig>,

    /// Override the enclave size, e.g. 2048M. Suffixes K, M, and G are supported.
    #[serde(rename = "memSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<String>,

    /// Number of enclave threads
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    /// Enables debug logging from EnclaveOS
    #[serde(rename = "debug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub debug: Option<bool>,

    /// Override the entrypoint of the original container
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entrypoint: Option<Vec<String>>,

    /// Override additional arguments to the container entrypoint
    #[serde(rename = "entrypointArgs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entrypoint_args: Option<Vec<String>>,

    /// Filesystem directories to encrypt using enclave sealing key
    #[serde(rename = "encryptedDirs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_dirs: Option<Vec<String>>,

    /// Add additional options to EnclaveOS manifest file
    #[serde(rename = "manifestOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manifest_options: Option<serde_json::Value>,

    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificates: Option<Vec<models::CertificateConfig>>,

    #[serde(rename = "caCertificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_certificates: Option<Vec<models::CaCertificateConfig>>,

    #[serde(rename = "signingKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signing_key: Option<models::SigningKeyConfig>,

    /// Fortanix external packages mount point in the toolserver container
    #[serde(rename = "externalPackages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_packages: Option<String>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<serde_json::Value>,

    /// Template for generating debug core dump file paths
    #[serde(rename = "coreDumpPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub core_dump_pattern: Option<String>,

    /// Path for EnclaveOS log file
    #[serde(rename = "logFilePath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_file_path: Option<String>,

    /// Type of the Java JVM used
    #[serde(rename = "javaMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub java_mode: Option<String>,

    /// List of read write directories
    #[serde(rename = "rwDirs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rw_dirs: Option<Vec<String>>,

    /// Allow command line arguments to EnclaveOS application
    #[serde(rename = "allowCmdlineArgs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_cmdline_args: Option<bool>,

    /// List of manifest environment variables
    #[serde(rename = "manifestEnv")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub manifest_env: Option<Vec<String>>,

}

impl ConversionRequest {
    pub fn new(input_image_name: String, output_image_name: String, ) -> ConversionRequest {
        ConversionRequest {
            input_image_name: input_image_name,
            output_image_name: output_image_name,
            input_auth_config: None,
            output_auth_config: None,
            auth_config: None,
            mem_size: None,
            threads: None,
            debug: None,
            entrypoint: None,
            entrypoint_args: None,
            encrypted_dirs: None,
            manifest_options: None,
            certificates: None,
            ca_certificates: None,
            signing_key: None,
            external_packages: None,
            app: None,
            core_dump_pattern: None,
            log_file_path: None,
            java_mode: None,
            rw_dirs: None,
            allow_cmdline_args: None,
            manifest_env: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConversionResponse {
    /// Registry and image name for the output container (same as outputImageName in the request)
    #[serde(rename = "newImage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_image: Option<String>,

    /// Shortened SHA256 Hash of the output image (This is the id of the image)
    #[serde(rename = "imageSHA")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_sha: Option<String>,

    /// The output image size in bytes
    #[serde(rename = "imageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_size: Option<isize>,

    /// This is the enclave productId which is same as the isvprodid in input request, if set. Default value is 0
    #[serde(rename = "isvprodid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub isvprodid: Option<isize>,

    /// This is the enclave security version which is same as the isvsvn in input request, if set. Default value is 0
    #[serde(rename = "isvsvn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub isvsvn: Option<isize>,

    /// This is the measurement of the enclave which uniquely identifies the shielded application. This is in hex format.
    #[serde(rename = "mrenclave")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mrenclave: Option<String>,

    /// This is the hash of the signing key which uniquely identifies the signing key. This is in hex format.
    #[serde(rename = "mrsigner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mrsigner: Option<String>,

}

impl ConversionResponse {
    pub fn new() -> ConversionResponse {
        ConversionResponse {
            new_image: None,
            image_sha: None,
            image_size: None,
            isvprodid: None,
            isvsvn: None,
            mrenclave: None,
            mrsigner: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ConvertAppBuildRequest {
    /// app id of the build
    #[serde(rename = "app_id")]
    pub app_id: uuid::Uuid,

    /// Build docker version
    #[serde(rename = "docker_version")]
    pub docker_version: String,

    #[serde(rename = "inputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "outputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "authConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_config: Option<models::AuthConfig>,

    /// Enables debug logging from EnclaveOS
    #[serde(rename = "debug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub debug: Option<bool>,

    /// Mem size required for the build
    #[serde(rename = "memSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads req for the build
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

}

impl ConvertAppBuildRequest {
    pub fn new(app_id: uuid::Uuid, docker_version: String, ) -> ConvertAppBuildRequest {
        ConvertAppBuildRequest {
            app_id: app_id,
            docker_version: docker_version,
            input_auth_config: None,
            output_auth_config: None,
            auth_config: None,
            debug: None,
            mem_size: None,
            threads: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CreateBuildRequest {
    #[serde(rename = "docker_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_info: Option<models::DockerInfo>,

    /// mrenclave of the build
    #[serde(rename = "mrenclave")]
    pub mrenclave: String,

    /// mrsigner of the build
    #[serde(rename = "mrsigner")]
    pub mrsigner: String,

    /// isvprodid of the build
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// isvsvn of the build
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// app id of the build
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<uuid::Uuid>,

    /// app name of the build
    #[serde(rename = "app_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_name: Option<String>,

    /// Mem size required for the build
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads req for the build
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

}

impl CreateBuildRequest {
    pub fn new(mrenclave: String, mrsigner: String, isvprodid: i32, isvsvn: i32, ) -> CreateBuildRequest {
        CreateBuildRequest {
            docker_info: None,
            mrenclave: mrenclave,
            mrsigner: mrsigner,
            isvprodid: isvprodid,
            isvsvn: isvsvn,
            app_id: None,
            app_name: None,
            mem_size: None,
            threads: None,
            advanced_settings: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CreateClusterRequest {
    /// Name of the cluster to create
    #[serde(rename = "name")]
    pub name: String,

}

impl CreateClusterRequest {
    pub fn new(name: String, ) -> CreateClusterRequest {
        CreateClusterRequest {
            name: name,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DockerInfo {
    /// Build docker image name
    #[serde(rename = "docker_image_name")]
    pub docker_image_name: String,

    /// Build docker version
    #[serde(rename = "docker_version")]
    pub docker_version: String,

    /// Build docker image sha
    #[serde(rename = "docker_image_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_image_sha: Option<String>,

    /// Docker image size in kb
    #[serde(rename = "docker_image_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_image_size: Option<i64>,

}

impl DockerInfo {
    pub fn new(docker_image_name: String, docker_version: String, ) -> DockerInfo {
        DockerInfo {
            docker_image_name: docker_image_name,
            docker_version: docker_version,
            docker_image_sha: None,
            docker_image_size: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct EnclaveInfo {
    /// mrenclave of a build, as a hex string
    #[serde(rename = "mrenclave")]
    pub mrenclave: String,

    /// mr signer of a build, as a hex string
    #[serde(rename = "mrsigner")]
    pub mrsigner: String,

    /// IsvProdId
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

}

impl EnclaveInfo {
    pub fn new(mrenclave: String, mrsigner: String, isvprodid: i32, isvsvn: i32, ) -> EnclaveInfo {
        EnclaveInfo {
            mrenclave: mrenclave,
            mrsigner: mrsigner,
            isvprodid: isvprodid,
            isvsvn: isvsvn,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum EventActionType { 
    #[serde(rename = "NODE_STATUS")]
    NODE_STATUS,
    #[serde(rename = "APP_STATUS")]
    APP_STATUS,
    #[serde(rename = "USER_APPROVAL")]
    USER_APPROVAL,
    #[serde(rename = "NODE_ATTESTATION")]
    NODE_ATTESTATION,
    #[serde(rename = "CERTIFICATE")]
    CERTIFICATE,
    #[serde(rename = "ADMIN")]
    ADMIN,
    #[serde(rename = "APP_HEARTBEAT")]
    APP_HEARTBEAT,
    #[serde(rename = "USER_AUTH")]
    USER_AUTH,
}

impl ::std::fmt::Display for EventActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            EventActionType::NODE_STATUS => write!(f, "{}", "NODE_STATUS"),
            EventActionType::APP_STATUS => write!(f, "{}", "APP_STATUS"),
            EventActionType::USER_APPROVAL => write!(f, "{}", "USER_APPROVAL"),
            EventActionType::NODE_ATTESTATION => write!(f, "{}", "NODE_ATTESTATION"),
            EventActionType::CERTIFICATE => write!(f, "{}", "CERTIFICATE"),
            EventActionType::ADMIN => write!(f, "{}", "ADMIN"),
            EventActionType::APP_HEARTBEAT => write!(f, "{}", "APP_HEARTBEAT"),
            EventActionType::USER_AUTH => write!(f, "{}", "USER_AUTH"),
        }
    }
}

impl ::std::str::FromStr for EventActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NODE_STATUS" => Ok(EventActionType::NODE_STATUS),
            "APP_STATUS" => Ok(EventActionType::APP_STATUS),
            "USER_APPROVAL" => Ok(EventActionType::USER_APPROVAL),
            "NODE_ATTESTATION" => Ok(EventActionType::NODE_ATTESTATION),
            "CERTIFICATE" => Ok(EventActionType::CERTIFICATE),
            "ADMIN" => Ok(EventActionType::ADMIN),
            "APP_HEARTBEAT" => Ok(EventActionType::APP_HEARTBEAT),
            "USER_AUTH" => Ok(EventActionType::USER_AUTH),
            _ => Err(()),
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum EventActorType { 
    #[serde(rename = "APP")]
    APP,
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "SYSTEM")]
    SYSTEM,
}

impl ::std::fmt::Display for EventActorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            EventActorType::APP => write!(f, "{}", "APP"),
            EventActorType::USER => write!(f, "{}", "USER"),
            EventActorType::SYSTEM => write!(f, "{}", "SYSTEM"),
        }
    }
}

impl ::std::str::FromStr for EventActorType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "APP" => Ok(EventActorType::APP),
            "USER" => Ok(EventActorType::USER),
            "SYSTEM" => Ok(EventActorType::SYSTEM),
            _ => Err(()),
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum EventSeverity { 
    #[serde(rename = "INFO")]
    INFO,
    #[serde(rename = "WARNING")]
    WARNING,
    #[serde(rename = "ERROR")]
    ERROR,
    #[serde(rename = "CRITICAL")]
    CRITICAL,
}

impl ::std::fmt::Display for EventSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            EventSeverity::INFO => write!(f, "{}", "INFO"),
            EventSeverity::WARNING => write!(f, "{}", "WARNING"),
            EventSeverity::ERROR => write!(f, "{}", "ERROR"),
            EventSeverity::CRITICAL => write!(f, "{}", "CRITICAL"),
        }
    }
}

impl ::std::str::FromStr for EventSeverity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INFO" => Ok(EventSeverity::INFO),
            "WARNING" => Ok(EventSeverity::WARNING),
            "ERROR" => Ok(EventSeverity::ERROR),
            "CRITICAL" => Ok(EventSeverity::CRITICAL),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ForgotPasswordRequest {
    #[serde(rename = "user_email")]
    pub user_email: String,

}

impl ForgotPasswordRequest {
    pub fn new(user_email: String, ) -> ForgotPasswordRequest {
        ForgotPasswordRequest {
            user_email: user_email,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllAppsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::App>,

}

impl GetAllAppsResponse {
    pub fn new(items: Vec<models::App>, ) -> GetAllAppsResponse {
        GetAllAppsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllBuildDeploymentsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::AppNodeInfo>,

}

impl GetAllBuildDeploymentsResponse {
    pub fn new(items: Vec<models::AppNodeInfo>, ) -> GetAllBuildDeploymentsResponse {
        GetAllBuildDeploymentsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllBuildsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::Build>,

}

impl GetAllBuildsResponse {
    pub fn new(items: Vec<models::Build>, ) -> GetAllBuildsResponse {
        GetAllBuildsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllNodesResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::Node>,

}

impl GetAllNodesResponse {
    pub fn new(items: Vec<models::Node>, ) -> GetAllNodesResponse {
        GetAllNodesResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllTasksResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::Task>,

}

impl GetAllTasksResponse {
    pub fn new(items: Vec<models::Task>, ) -> GetAllTasksResponse {
        GetAllTasksResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllUsersResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::User>,

}

impl GetAllUsersResponse {
    pub fn new(items: Vec<models::User>, ) -> GetAllUsersResponse {
        GetAllUsersResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAuditLogsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::AuditLog>,

}

impl GetAuditLogsResponse {
    pub fn new(items: Vec<models::AuditLog>, ) -> GetAuditLogsResponse {
        GetAuditLogsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct InviteUserRequest {
    /// User's email address.
    #[serde(rename = "user_email")]
    pub user_email: String,

    #[serde(rename = "roles")]
    pub roles: Vec<models::AccessRoles>,

    #[serde(rename = "first_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "last_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

}

impl InviteUserRequest {
    pub fn new(user_email: String, roles: Vec<models::AccessRoles>, ) -> InviteUserRequest {
        InviteUserRequest {
            user_email: user_email,
            roles: roles,
            first_name: None,
            last_name: None,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum JavaRuntime { 
    #[serde(rename = "JAVA-ORACLE")]
    JAVA_ORACLE,
    #[serde(rename = "OPENJDK")]
    OPENJDK,
    #[serde(rename = "OPENJ9")]
    OPENJ9,
    #[serde(rename = "LIBERTY-JRE")]
    LIBERTY_JRE,
}

impl ::std::fmt::Display for JavaRuntime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            JavaRuntime::JAVA_ORACLE => write!(f, "{}", "JAVA-ORACLE"),
            JavaRuntime::OPENJDK => write!(f, "{}", "OPENJDK"),
            JavaRuntime::OPENJ9 => write!(f, "{}", "OPENJ9"),
            JavaRuntime::LIBERTY_JRE => write!(f, "{}", "LIBERTY-JRE"),
        }
    }
}

impl ::std::str::FromStr for JavaRuntime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JAVA-ORACLE" => Ok(JavaRuntime::JAVA_ORACLE),
            "OPENJDK" => Ok(JavaRuntime::OPENJDK),
            "OPENJ9" => Ok(JavaRuntime::OPENJ9),
            "LIBERTY-JRE" => Ok(JavaRuntime::LIBERTY_JRE),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct JoinClusterRequest {
    /// Hostname or IP address of the admin interface of the cluster to join
    #[serde(rename = "target")]
    pub target: String,

}

impl JoinClusterRequest {
    pub fn new(target: String, ) -> JoinClusterRequest {
        JoinClusterRequest {
            target: target,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NewCertificateRequest {
    /// Certificate signing request
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// Node Id for the requesting host agent
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<uuid::Uuid>,

}

impl NewCertificateRequest {
    pub fn new() -> NewCertificateRequest {
        NewCertificateRequest {
            csr: None,
            node_id: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Node {
    /// Name of the node
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the node
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The account ID of the account that this node belongs to.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// IP Address of the node
    #[serde(rename = "ipaddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ipaddress: Option<String>,

    /// UUID for the node
    #[serde(rename = "node_id")]
    pub node_id: uuid::Uuid,

    /// No longer used
    #[serde(rename = "host_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_id: Option<String>,

    /// Zone ID of the zone this node belongs to
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zone_id: Option<uuid::Uuid>,

    #[serde(rename = "status")]
    pub status: models::NodeStatus,

    /// The node attestation date
    #[serde(rename = "attested_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attested_at: Option<i64>,

    /// The node attestation certificate
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,

    /// Apps associated with the node
    #[serde(rename = "apps")]
    pub apps: Vec<models::AppNodeInfo>,

    #[serde(rename = "sgx_info")]
    pub sgx_info: models::SgxInfo,

}

impl Node {
    pub fn new(name: String, acct_id: uuid::Uuid, node_id: uuid::Uuid, status: models::NodeStatus, apps: Vec<models::AppNodeInfo>, sgx_info: models::SgxInfo, ) -> Node {
        Node {
            name: name,
            description: None,
            acct_id: acct_id,
            ipaddress: None,
            node_id: node_id,
            host_id: None,
            zone_id: None,
            status: status,
            attested_at: None,
            certificate: None,
            apps: apps,
            sgx_info: sgx_info,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeProvisionRequest {
    /// Name of the node
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the node
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// IP Address of the node
    #[serde(rename = "ipaddress")]
    pub ipaddress: String,

    /// No longer used
    #[serde(rename = "host_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_id: Option<String>,

    /// Intel SGX version running on the node
    #[serde(rename = "sgx_version")]
    pub sgx_version: String,

    #[serde(rename = "attestation_request")]
    pub attestation_request: models::AttestationRequest,

}

impl NodeProvisionRequest {
    pub fn new(name: String, ipaddress: String, sgx_version: String, attestation_request: models::AttestationRequest, ) -> NodeProvisionRequest {
        NodeProvisionRequest {
            name: name,
            description: None,
            ipaddress: ipaddress,
            host_id: None,
            sgx_version: sgx_version,
            attestation_request: attestation_request,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeStatus {
    #[serde(rename = "status")]
    pub status: models::NodeStatusType,

    /// Node created at
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Time since the status change
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: i64,

}

impl NodeStatus {
    pub fn new(status: models::NodeStatusType, created_at: i64, status_updated_at: i64, ) -> NodeStatus {
        NodeStatus {
            status: status,
            created_at: created_at,
            status_updated_at: status_updated_at,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum NodeStatusType { 
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "STOPPED")]
    STOPPED,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "DEACTIVATED")]
    DEACTIVATED,
    #[serde(rename = "INPROGRESS")]
    INPROGRESS,
}

impl ::std::fmt::Display for NodeStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            NodeStatusType::RUNNING => write!(f, "{}", "RUNNING"),
            NodeStatusType::STOPPED => write!(f, "{}", "STOPPED"),
            NodeStatusType::FAILED => write!(f, "{}", "FAILED"),
            NodeStatusType::DEACTIVATED => write!(f, "{}", "DEACTIVATED"),
            NodeStatusType::INPROGRESS => write!(f, "{}", "INPROGRESS"),
        }
    }
}

impl ::std::str::FromStr for NodeStatusType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RUNNING" => Ok(NodeStatusType::RUNNING),
            "STOPPED" => Ok(NodeStatusType::STOPPED),
            "FAILED" => Ok(NodeStatusType::FAILED),
            "DEACTIVATED" => Ok(NodeStatusType::DEACTIVATED),
            "INPROGRESS" => Ok(NodeStatusType::INPROGRESS),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeUpdateRequest {
    /// Name of the node
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the node
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// IP Address of the node
    #[serde(rename = "ipaddress")]
    pub ipaddress: String,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::NodeStatus>,

    /// Intel SGX version running on the node
    #[serde(rename = "sgx_version")]
    pub sgx_version: String,

}

impl NodeUpdateRequest {
    pub fn new(name: String, ipaddress: String, sgx_version: String, ) -> NodeUpdateRequest {
        NodeUpdateRequest {
            name: name,
            description: None,
            ipaddress: ipaddress,
            status: None,
            sgx_version: sgx_version,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OAuthInitiationRequest {
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub provider: Option<models::OAuthProviderType>,

    /// URI where Oauth provider will redirect to post authentication. If not given then redirects to /auth/callback
    #[serde(rename = "callback_uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub callback_uri: Option<String>,

}

impl OAuthInitiationRequest {
    pub fn new() -> OAuthInitiationRequest {
        OAuthInitiationRequest {
            provider: None,
            callback_uri: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OAuthInitiationResponse {
    /// OAuth redirection URL
    #[serde(rename = "redirect_uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub redirect_uri: Option<String>,

    /// OAuth state to be cached by client
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

}

impl OAuthInitiationResponse {
    pub fn new() -> OAuthInitiationResponse {
        OAuthInitiationResponse {
            redirect_uri: None,
            state: None,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum OAuthProviderType { 
    #[serde(rename = "IBM-APPID")]
    IBM_APPID,
}

impl ::std::fmt::Display for OAuthProviderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            OAuthProviderType::IBM_APPID => write!(f, "{}", "IBM-APPID"),
        }
    }
}

impl ::std::str::FromStr for OAuthProviderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IBM-APPID" => Ok(OAuthProviderType::IBM_APPID),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct PasswordChangeRequest {
    #[serde(rename = "current_password")]
    pub current_password: String,

    #[serde(rename = "new_password")]
    pub new_password: String,

}

impl PasswordChangeRequest {
    pub fn new(current_password: String, new_password: String, ) -> PasswordChangeRequest {
        PasswordChangeRequest {
            current_password: current_password,
            new_password: new_password,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct PasswordResetRequest {
    #[serde(rename = "reset_token")]
    pub reset_token: String,

    #[serde(rename = "new_password")]
    pub new_password: String,

}

impl PasswordResetRequest {
    pub fn new(reset_token: String, new_password: String, ) -> PasswordResetRequest {
        PasswordResetRequest {
            reset_token: reset_token,
            new_password: new_password,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ProcessInviteRequest {
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accepts: Option<Vec<uuid::Uuid>>,

    #[serde(rename = "rejects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rejects: Option<Vec<uuid::Uuid>>,

}

impl ProcessInviteRequest {
    pub fn new() -> ProcessInviteRequest {
        ProcessInviteRequest {
            accepts: None,
            rejects: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct RequesterInfo {
    /// User Id
    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id: Option<uuid::Uuid>,

    /// User Name
    #[serde(rename = "user_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    /// App Id
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<uuid::Uuid>,

    /// App Name
    #[serde(rename = "app_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_name: Option<String>,

    #[serde(rename = "requester_type")]
    pub requester_type: models::RequesterType,

}

impl RequesterInfo {
    pub fn new(requester_type: models::RequesterType, ) -> RequesterInfo {
        RequesterInfo {
            user_id: None,
            user_name: None,
            app_id: None,
            app_name: None,
            requester_type: requester_type,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum RequesterType { 
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "APP")]
    APP,
    #[serde(rename = "SYSTEM")]
    SYSTEM,
}

impl ::std::fmt::Display for RequesterType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            RequesterType::USER => write!(f, "{}", "USER"),
            RequesterType::APP => write!(f, "{}", "APP"),
            RequesterType::SYSTEM => write!(f, "{}", "SYSTEM"),
        }
    }
}

impl ::std::str::FromStr for RequesterType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USER" => Ok(RequesterType::USER),
            "APP" => Ok(RequesterType::APP),
            "SYSTEM" => Ok(RequesterType::SYSTEM),
            _ => Err(()),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SdkmsSigningKeyConfig {
    /// name of the signing key in SDKMS
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// API key to authenticate with SDKMS
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api_key: Option<String>,

}

impl SdkmsSigningKeyConfig {
    pub fn new() -> SdkmsSigningKeyConfig {
        SdkmsSigningKeyConfig {
            name: None,
            api_key: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SearchMetadata {
    /// Current page number
    #[serde(rename = "page")]
    pub page: isize,

    /// Total pages as per the item counts and page limit
    #[serde(rename = "pages")]
    pub pages: isize,

    /// Number of items to limit in a page
    #[serde(rename = "limit")]
    pub limit: isize,

    /// Total number of unfiltered items
    #[serde(rename = "total_count")]
    pub total_count: isize,

    /// Total number of items as per the current filter
    #[serde(rename = "filtered_count")]
    pub filtered_count: isize,

}

impl SearchMetadata {
    pub fn new(page: isize, pages: isize, limit: isize, total_count: isize, filtered_count: isize, ) -> SearchMetadata {
        SearchMetadata {
            page: page,
            pages: pages,
            limit: limit,
            total_count: total_count,
            filtered_count: filtered_count,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SgxInfo {
    /// Intel SGX version
    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

}

impl SgxInfo {
    pub fn new() -> SgxInfo {
        SgxInfo {
            version: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SigningKeyConfig {
    /// Requests signing the converted image with a default key
    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(rename = "sdkms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sdkms: Option<models::SdkmsSigningKeyConfig>,

}

impl SigningKeyConfig {
    pub fn new() -> SigningKeyConfig {
        SigningKeyConfig {
            default: None,
            sdkms: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SignupRequest {
    /// User's email address.
    #[serde(rename = "user_email")]
    pub user_email: String,

    /// The password to assign to this user in Enclave Manager.
    #[serde(rename = "user_password")]
    pub user_password: String,

    #[serde(rename = "first_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "last_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

}

impl SignupRequest {
    pub fn new(user_email: String, user_password: String, ) -> SignupRequest {
        SignupRequest {
            user_email: user_email,
            user_password: user_password,
            first_name: None,
            last_name: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Task {
    /// Task Id
    #[serde(rename = "task_id")]
    pub task_id: uuid::Uuid,

    #[serde(rename = "requester_info")]
    pub requester_info: models::RequesterInfo,

    /// app_id, build_id, node_id, cert_id are entity_id for app_domain_whitelisting, build_whitelisting, node_attestation and certificate_issuance respectively.
    #[serde(rename = "entity_id")]
    pub entity_id: uuid::Uuid,

    #[serde(rename = "task_type")]
    pub task_type: models::TaskType,

    #[serde(rename = "status")]
    pub status: models::TaskStatus,

    /// Task details
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "approvals")]
    pub approvals: Vec<models::ApprovalInfo>,

    #[serde(rename = "domains_added")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_added: Option<Vec<String>>,

    #[serde(rename = "domains_removed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_removed: Option<Vec<String>>,

}

impl Task {
    pub fn new(task_id: uuid::Uuid, requester_info: models::RequesterInfo, entity_id: uuid::Uuid, task_type: models::TaskType, status: models::TaskStatus, approvals: Vec<models::ApprovalInfo>, ) -> Task {
        Task {
            task_id: task_id,
            requester_info: requester_info,
            entity_id: entity_id,
            task_type: task_type,
            status: status,
            description: None,
            approvals: approvals,
            domains_added: None,
            domains_removed: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TaskResult {
    /// Task Id
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_id: Option<uuid::Uuid>,

    /// Certificate Id in case of certificate issuance task
    #[serde(rename = "certificate_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<uuid::Uuid>,

    /// Node Id
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<uuid::Uuid>,

    #[serde(rename = "task_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_type: Option<models::TaskType>,

    #[serde(rename = "task_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_status: Option<models::TaskStatus>,

    /// build Id
    #[serde(rename = "build_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<uuid::Uuid>,

}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            task_id: None,
            certificate_id: None,
            node_id: None,
            task_type: None,
            task_status: None,
            build_id: None,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TaskStatus {
    /// Task creation time
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Time since the status change
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: i64,

    #[serde(rename = "status")]
    pub status: models::TaskStatusType,

}

impl TaskStatus {
    pub fn new(created_at: i64, status_updated_at: i64, status: models::TaskStatusType, ) -> TaskStatus {
        TaskStatus {
            created_at: created_at,
            status_updated_at: status_updated_at,
            status: status,
        }
    }
}






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
}

impl ::std::fmt::Display for TaskStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            TaskStatusType::INPROGRESS => write!(f, "{}", "INPROGRESS"),
            TaskStatusType::FAILED => write!(f, "{}", "FAILED"),
            TaskStatusType::SUCCESS => write!(f, "{}", "SUCCESS"),
            TaskStatusType::DENIED => write!(f, "{}", "DENIED"),
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
            _ => Err(()),
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum TaskType { 
    #[serde(rename = "NODE_ATTESTATION")]
    NODE_ATTESTATION,
    #[serde(rename = "CERTIFICATE_ISSUANCE")]
    CERTIFICATE_ISSUANCE,
    #[serde(rename = "BUILD_WHITELIST")]
    BUILD_WHITELIST,
    #[serde(rename = "DOMAIN_WHITELIST")]
    DOMAIN_WHITELIST,
}

impl ::std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            TaskType::NODE_ATTESTATION => write!(f, "{}", "NODE_ATTESTATION"),
            TaskType::CERTIFICATE_ISSUANCE => write!(f, "{}", "CERTIFICATE_ISSUANCE"),
            TaskType::BUILD_WHITELIST => write!(f, "{}", "BUILD_WHITELIST"),
            TaskType::DOMAIN_WHITELIST => write!(f, "{}", "DOMAIN_WHITELIST"),
        }
    }
}

impl ::std::str::FromStr for TaskType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NODE_ATTESTATION" => Ok(TaskType::NODE_ATTESTATION),
            "CERTIFICATE_ISSUANCE" => Ok(TaskType::CERTIFICATE_ISSUANCE),
            "BUILD_WHITELIST" => Ok(TaskType::BUILD_WHITELIST),
            "DOMAIN_WHITELIST" => Ok(TaskType::DOMAIN_WHITELIST),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TaskUpdateRequest {
    #[serde(rename = "status")]
    pub status: models::ApprovalStatus,

}

impl TaskUpdateRequest {
    pub fn new(status: models::ApprovalStatus, ) -> TaskUpdateRequest {
        TaskUpdateRequest {
            status: status,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct UpdateUserRequest {
    /// first name
    #[serde(rename = "first_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    /// last name
    #[serde(rename = "last_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

    #[serde(rename = "roles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub roles: Option<Vec<models::AccessRoles>>,

}

impl UpdateUserRequest {
    pub fn new() -> UpdateUserRequest {
        UpdateUserRequest {
            first_name: None,
            last_name: None,
            roles: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct User {
    /// User Id
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,

    /// First Name
    #[serde(rename = "first_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    /// Last Name
    #[serde(rename = "last_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

    /// User Email
    #[serde(rename = "user_email")]
    pub user_email: String,

    /// Last login time of user
    #[serde(rename = "last_logged_in_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_logged_in_at: Option<i64>,

    /// Creation time of user
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Whether this user's email has been verified.
    #[serde(rename = "email_verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email_verified: Option<bool>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::UserStatus>,

    #[serde(rename = "roles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub roles: Option<Vec<models::AccessRoles>>,

    #[serde(rename = "user_account_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_account_status: Option<models::UserAccountStatus>,

}

impl User {
    pub fn new(user_id: uuid::Uuid, user_email: String, ) -> User {
        User {
            user_id: user_id,
            first_name: None,
            last_name: None,
            user_email: user_email,
            last_logged_in_at: None,
            created_at: None,
            email_verified: None,
            status: None,
            roles: None,
            user_account_status: None,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum UserAccountStatus { 
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "DISABLED")]
    DISABLED,
}

impl ::std::fmt::Display for UserAccountStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            UserAccountStatus::ACTIVE => write!(f, "{}", "ACTIVE"),
            UserAccountStatus::PENDING => write!(f, "{}", "PENDING"),
            UserAccountStatus::DISABLED => write!(f, "{}", "DISABLED"),
        }
    }
}

impl ::std::str::FromStr for UserAccountStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(UserAccountStatus::ACTIVE),
            "PENDING" => Ok(UserAccountStatus::PENDING),
            "DISABLED" => Ok(UserAccountStatus::DISABLED),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct UserBlacklistRequest {
    /// email of a user to be blacklisted
    #[serde(rename = "email")]
    pub email: String,

}

impl UserBlacklistRequest {
    pub fn new(email: String, ) -> UserBlacklistRequest {
        UserBlacklistRequest {
            email: email,
        }
    }
}






#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum UserStatus { 
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "DISABLED")]
    DISABLED,
}

impl ::std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            UserStatus::ACTIVE => write!(f, "{}", "ACTIVE"),
            UserStatus::PENDING => write!(f, "{}", "PENDING"),
            UserStatus::DISABLED => write!(f, "{}", "DISABLED"),
        }
    }
}

impl ::std::str::FromStr for UserStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(UserStatus::ACTIVE),
            "PENDING" => Ok(UserStatus::PENDING),
            "DISABLED" => Ok(UserStatus::DISABLED),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ValidateTokenRequest {
    #[serde(rename = "reset_token")]
    pub reset_token: String,

}

impl ValidateTokenRequest {
    pub fn new(reset_token: String, ) -> ValidateTokenRequest {
        ValidateTokenRequest {
            reset_token: reset_token,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ValidateTokenResponse {
    #[serde(rename = "user_email")]
    pub user_email: String,

}

impl ValidateTokenResponse {
    pub fn new(user_email: String, ) -> ValidateTokenResponse {
        ValidateTokenResponse {
            user_email: user_email,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct VersionResponse {
    /// Manager Version
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



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Zone {
    /// The account ID of the account that this zone belongs to.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// Zone Certificate
    #[serde(rename = "certificate")]
    pub certificate: String,

    /// Zone Id
    #[serde(rename = "zone_id")]
    pub zone_id: uuid::Uuid,

    /// zone name
    #[serde(rename = "name")]
    pub name: String,

    /// zone description
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl Zone {
    pub fn new(acct_id: uuid::Uuid, certificate: String, zone_id: uuid::Uuid, name: String, ) -> Zone {
        Zone {
            acct_id: acct_id,
            certificate: certificate,
            zone_id: zone_id,
            name: name,
            description: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ZoneJoinToken {
    /// Bearer token used to enroll compute nodes
    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

}

impl ZoneJoinToken {
    pub fn new() -> ZoneJoinToken {
        ZoneJoinToken {
            token: None,
        }
    }
}

