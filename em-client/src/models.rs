/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;

use serde::ser::Serializer;

use std::collections::BTreeMap as SortedHashMap;
use std::collections::BTreeSet as SortedVec;
use std::collections::HashMap;
use models;
use std::string::ParseError;
use uuid;


/// Roles of a user.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Account ID uniquely identifying this account.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// When this account was created.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Role of the current user in a particular account.
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub roles: Option<Vec<models::AccessRoles>>,

    /// logo of the particular account. Max size 128Kb, .jpg, .png, .svg file formats only.
    #[serde(rename = "custom_logo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_logo: Option<crate::ByteArray>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::UserAccountStatus>,

    #[serde(rename = "features")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub features: Option<Vec<String>>,

    #[serde(rename = "auth_configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_configs: Option<HashMap<String, models::AuthenticationConfig>>,

    #[serde(rename = "approval_state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub approval_state: Option<models::AccountApprovalState>,

}

impl Account {
    pub fn new(acct_id: uuid::Uuid, ) -> Account {
        Account {
            name: None,
            acct_id: acct_id,
            created_at: None,
            roles: None,
            custom_logo: None,
            status: None,
            features: None,
            auth_configs: None,
            approval_state: None,
        }
    }
}


/// Approval state of an Account
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum AccountApprovalState { 
    #[serde(rename = "PENDING_CONFIRMATION")]
    PENDING_CONFIRMATION,
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "DISAPPROVED")]
    DISAPPROVED,
}

impl ::std::fmt::Display for AccountApprovalState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            AccountApprovalState::PENDING_CONFIRMATION => write!(f, "{}", "PENDING_CONFIRMATION"),
            AccountApprovalState::APPROVED => write!(f, "{}", "APPROVED"),
            AccountApprovalState::DISAPPROVED => write!(f, "{}", "DISAPPROVED"),
        }
    }
}

impl ::std::str::FromStr for AccountApprovalState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING_CONFIRMATION" => Ok(AccountApprovalState::PENDING_CONFIRMATION),
            "APPROVED" => Ok(AccountApprovalState::APPROVED),
            "DISAPPROVED" => Ok(AccountApprovalState::DISAPPROVED),
            _ => Err(()),
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

    /// logo for an account. Max size 128Kb, .jpg, .png, .svg file formats only.
    #[serde(rename = "custom_logo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_logo: Option<crate::ByteArray>,

    #[serde(rename = "auth_configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_configs: Option<Vec<models::AuthenticationConfig>>,

}

impl AccountRequest {
    pub fn new(name: String, ) -> AccountRequest {
        AccountRequest {
            name: name,
            custom_logo: None,
            auth_configs: None,
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

    #[serde(rename = "features")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub features: Option<Vec<String>>,

    #[serde(rename = "add_auth_configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_auth_configs: Option<Vec<models::AuthenticationConfig>>,

    #[serde(rename = "mod_auth_configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mod_auth_configs: Option<HashMap<String, models::AuthenticationConfig>>,

    #[serde(rename = "del_auth_configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub del_auth_configs: Option<Vec<uuid::Uuid>>,

}

impl AccountUpdateRequest {
    pub fn new() -> AccountUpdateRequest {
        AccountUpdateRequest {
            name: None,
            custom_logo: None,
            features: None,
            add_auth_configs: None,
            mod_auth_configs: None,
            del_auth_configs: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AddZoneRequest {
    /// Name of the new zone.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Description of the new zone.
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


/// Advanced settings for apps and images.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AdvancedSettings {
    /// Entrypoint for the container.
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entrypoint: Option<Vec<String>>,

    /// Filesystem directories to encrypt using enclave sealing key.
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

    /// Filesystem directories to enable read write.
    #[serde(rename = "rw_dirs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rw_dirs: Option<Vec<String>>,

    /// Allow command line arguments converter flag for an image.
    #[serde(rename = "allowCmdlineArgs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_cmdline_args: Option<bool>,

    /// Environment variables that will be passed to the manifest file when the container is converted.
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
    /// Timestamp of image addition to the system.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Timestamp of image updation to the system.
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<i64>,

    /// Name of the app.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the app.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// UUID for the app
    #[serde(rename = "app_id")]
    pub app_id: uuid::Uuid,

    /// Input image name of images for apps.
    #[serde(rename = "input_image_name")]
    pub input_image_name: String,

    /// Output image name of images for apps.
    #[serde(rename = "output_image_name")]
    pub output_image_name: String,

    /// IsvProdId
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// Mem size required for the image.
    #[serde(rename = "mem_size")]
    pub mem_size: i64,

    /// Threads req for the image.
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

    /// UUID of pending domain whitelist task for the app.
    #[serde(rename = "pending_task_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_task_id: Option<uuid::Uuid>,

    #[serde(rename = "domains_added")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_added: Option<Vec<String>>,

    #[serde(rename = "domains_removed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domains_removed: Option<Vec<String>>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

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
            pending_task_id: None,
            domains_added: None,
            domains_removed: None,
            labels: None,
        }
    }
}


/// Request to update an app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppBodyUpdateRequest {
    /// Description of the app.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Input image name of images for apps.
    #[serde(rename = "input_image_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_image_name: Option<String>,

    /// Output image name of images for apps.
    #[serde(rename = "output_image_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_image_name: Option<String>,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub isvsvn: Option<i32>,

    /// ISVPRODID
    #[serde(rename = "isvprodid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub isvprodid: Option<i32>,

    /// Mem size required for the image.
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads required for the image.
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    #[serde(rename = "allowed_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<models::PatchDocument>>,

}

impl AppBodyUpdateRequest {
    pub fn new() -> AppBodyUpdateRequest {
        AppBodyUpdateRequest {
            description: None,
            input_image_name: None,
            output_image_name: None,
            isvsvn: None,
            isvprodid: None,
            mem_size: None,
            threads: None,
            allowed_domains: None,
            advanced_settings: None,
            labels: None,
        }
    }
}


/// Detailed info of an app running on a compute node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppNodeInfo {
    #[serde(rename = "certificate")]
    pub certificate: models::Certificate,

    /// App compute node creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Compute Node Id
    #[serde(rename = "node_id")]
    pub node_id: uuid::Uuid,

    /// Compute Node Name.
    #[serde(rename = "node_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::AppStatus>,

    #[serde(rename = "build_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_info: Option<models::Build>,

    /// App heartbeat message count.
    #[serde(rename = "message_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_count: Option<i32>,

    /// Key Id for app heartbeat.
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    /// App running in debug mode or not.
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
pub struct AppRegistryResponse {
    #[serde(rename = "input_image_registry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_image_registry: Option<models::Registry>,

    #[serde(rename = "output_image_registry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_image_registry: Option<models::Registry>,

}

impl AppRegistryResponse {
    pub fn new() -> AppRegistryResponse {
        AppRegistryResponse {
            input_image_registry: None,
            output_image_registry: None,
        }
    }
}


/// Request to create an app.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppRequest {
    /// Name of the app.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the app.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Input image name of images for apps.
    #[serde(rename = "input_image_name")]
    pub input_image_name: String,

    /// Output image name of images for apps.
    #[serde(rename = "output_image_name")]
    pub output_image_name: String,

    /// IsvProdId
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// Mem size required for the image.
    #[serde(rename = "mem_size")]
    pub mem_size: i64,

    /// Threads req for the image.
    #[serde(rename = "threads")]
    pub threads: i32,

    #[serde(rename = "allowed_domains")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

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
            labels: None,
        }
    }
}


/// Run status info of an app for a compute node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AppStatus {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::AppStatusType>,

    /// Time since the status change.
    #[serde(rename = "status_updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_updated_at: Option<i64>,

    /// The app attestation date.
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


/// Status string for the app on a compute node.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// Request to update an app.
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


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfig {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "app_config")]
    pub app_config: SortedHashMap<String, models::ApplicationConfigContents>,

    #[serde(rename = "labels")]
    pub labels: SortedHashMap<String, String>,

    #[serde(rename = "ports")]
    pub ports: SortedVec<String>,

    #[serde(rename = "zone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zone: Option<models::VersionedZoneId>,

}

impl ApplicationConfig {
    pub fn new(name: String, description: String, app_config: SortedHashMap<String, models::ApplicationConfigContents>, labels: SortedHashMap<String, String>, ports: SortedVec<String>, ) -> ApplicationConfig {
        ApplicationConfig {
            name: name,
            description: description,
            app_config: app_config,
            labels: labels,
            ports: ports,
            zone: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigConnection {
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset: Option<models::ApplicationConfigConnectionDataset>,

    #[serde(rename = "application")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application: Option<models::ApplicationConfigConnectionApplication>,

}

impl ApplicationConfigConnection {
    pub fn new() -> ApplicationConfigConnection {
        ApplicationConfigConnection {
            dataset: None,
            application: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigConnectionApplication {
    #[serde(rename = "workflow_domain")]
    pub workflow_domain: String,

}

impl ApplicationConfigConnectionApplication {
    pub fn new(workflow_domain: String, ) -> ApplicationConfigConnectionApplication {
        ApplicationConfigConnectionApplication {
            workflow_domain: workflow_domain,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigConnectionDataset {
    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "credentials")]
    pub credentials: models::ApplicationConfigDatasetCredentials,

}

impl ApplicationConfigConnectionDataset {
    pub fn new(location: String, credentials: models::ApplicationConfigDatasetCredentials, ) -> ApplicationConfigConnectionDataset {
        ApplicationConfigConnectionDataset {
            location: location,
            credentials: credentials,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigContents {
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

}

impl ApplicationConfigContents {
    pub fn new() -> ApplicationConfigContents {
        ApplicationConfigContents {
            contents: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigDatasetCredentials {
    #[serde(rename = "sdkms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sdkms: Option<models::ApplicationConfigSdkmsCredentials>,

}

impl ApplicationConfigDatasetCredentials {
    pub fn new() -> ApplicationConfigDatasetCredentials {
        ApplicationConfigDatasetCredentials {
            sdkms: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigExtra {
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<SortedHashMap<String, SortedHashMap<String, models::ApplicationConfigConnection>>>,

}

impl ApplicationConfigExtra {
    pub fn new() -> ApplicationConfigExtra {
        ApplicationConfigExtra {
            connections: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigPort {
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset: Option<models::ApplicationConfigPortDataset>,

    /// 
    #[serde(rename = "application")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application: Option<serde_json::Value>,

}

impl ApplicationConfigPort {
    pub fn new() -> ApplicationConfigPort {
        ApplicationConfigPort {
            dataset: None,
            application: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigPortDataset {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

}

impl ApplicationConfigPortDataset {
    pub fn new(id: uuid::Uuid, ) -> ApplicationConfigPortDataset {
        ApplicationConfigPortDataset {
            id: id,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigResponse {
    #[serde(rename = "config_id")]
    pub config_id: String,

    #[serde(rename = "created_at")]
    pub created_at: i64,

    #[serde(rename = "updated_at")]
    pub updated_at: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "app_config")]
    pub app_config: SortedHashMap<String, models::ApplicationConfigContents>,

    #[serde(rename = "labels")]
    pub labels: SortedHashMap<String, String>,

    #[serde(rename = "ports")]
    pub ports: SortedVec<String>,

    #[serde(rename = "zone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zone: Option<models::VersionedZoneId>,

}

impl ApplicationConfigResponse {
    pub fn new(config_id: String, created_at: i64, updated_at: i64, name: String, description: String, app_config: SortedHashMap<String, models::ApplicationConfigContents>, labels: SortedHashMap<String, String>, ports: SortedVec<String>, ) -> ApplicationConfigResponse {
        ApplicationConfigResponse {
            config_id: config_id,
            created_at: created_at,
            updated_at: updated_at,
            name: name,
            description: description,
            app_config: app_config,
            labels: labels,
            ports: ports,
            zone: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigSdkmsCredentials {
    #[serde(rename = "credentials_url")]
    pub credentials_url: String,

    #[serde(rename = "credentials_key_name")]
    pub credentials_key_name: String,

    #[serde(rename = "sdkms_app_id")]
    pub sdkms_app_id: uuid::Uuid,

}

impl ApplicationConfigSdkmsCredentials {
    pub fn new(credentials_url: String, credentials_key_name: String, sdkms_app_id: uuid::Uuid, ) -> ApplicationConfigSdkmsCredentials {
        ApplicationConfigSdkmsCredentials {
            credentials_url: credentials_url,
            credentials_key_name: credentials_key_name,
            sdkms_app_id: sdkms_app_id,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApplicationConfigWorkflow {
    #[serde(rename = "workflow_id")]
    pub workflow_id: uuid::Uuid,

    #[serde(rename = "app_name")]
    pub app_name: String,

    #[serde(rename = "port_map")]
    pub port_map: SortedHashMap<String, SortedHashMap<String, models::ApplicationConfigPort>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_acct_id: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_group_id: Option<uuid::Uuid>,
}

impl ApplicationConfigWorkflow {
    pub fn new(workflow_id: uuid::Uuid, app_name: String, port_map: SortedHashMap<String, SortedHashMap<String, models::ApplicationConfigPort>>, app_acct_id: Option<uuid::Uuid>, app_group_id: Option<uuid::Uuid>) -> ApplicationConfigWorkflow {
        ApplicationConfigWorkflow {
            workflow_id: workflow_id,
            app_name: app_name,
            port_map: port_map,
            app_acct_id,
            app_group_id
        }
    }
}


/// Result of an approval request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApprovableResult {
    /// The HTTP status code for this partial request.
    #[serde(rename = "status")]
    pub status: isize,

    #[serde(rename = "body")]
    pub body: serde_json::Value,

}

impl ApprovableResult {
    pub fn new(status: isize, body: serde_json::Value, ) -> ApprovableResult {
        ApprovableResult {
            status: status,
            body: body,
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


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApprovalRequest {
    /// UUID uniquely identifying this approval request.
    #[serde(rename = "request_id")]
    pub request_id: uuid::Uuid,

    #[serde(rename = "requester")]
    pub requester: models::Entity,

    /// When this approval request was created.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// The account ID of the account that this approval request belongs to.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// Operation URL path, e.g. `/crypto/v1/keys`, `/crypto/v1/groups/<id>`.
    #[serde(rename = "operation")]
    pub operation: String,

    /// Method for the operation: POST, PATCH, PUT, DELETE, or GET. Default is POST.
    #[serde(rename = "method")]
    pub method: String,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<serde_json::Value>,

    #[serde(rename = "approvers")]
    pub approvers: Vec<models::Entity>,

    #[serde(rename = "denier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub denier: Option<models::Entity>,

    /// Reason given by denier.
    #[serde(rename = "denial_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub denial_reason: Option<String>,

    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<models::Entity>>,

    #[serde(rename = "status")]
    pub status: models::ApprovalRequestStatus,

    #[serde(rename = "subjects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subjects: Option<Vec<models::ApprovalSubject>>,

    /// Optional comment about the approval request for the reviewer.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// When this approval request expires.
    #[serde(rename = "expiry")]
    pub expiry: i64,

}

impl ApprovalRequest {
    pub fn new(request_id: uuid::Uuid, requester: models::Entity, created_at: i64, acct_id: uuid::Uuid, operation: String, method: String, approvers: Vec<models::Entity>, status: models::ApprovalRequestStatus, expiry: i64, ) -> ApprovalRequest {
        ApprovalRequest {
            request_id: request_id,
            requester: requester,
            created_at: created_at,
            acct_id: acct_id,
            operation: operation,
            method: method,
            body: None,
            approvers: approvers,
            denier: None,
            denial_reason: None,
            reviewers: None,
            status: status,
            subjects: None,
            description: None,
            expiry: expiry,
        }
    }
}


/// Request to create an approval request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApprovalRequestRequest {
    /// Operation URL path, e.g. `/crypto/v1/keys`, `/crypto/v1/groups/<id>`.
    #[serde(rename = "operation")]
    pub operation: String,

    /// Method for the operation: POST, PATCH, PUT, DELETE, or GET. Default is POST.
    #[serde(rename = "method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub method: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<serde_json::Value>,

    /// Optional comment about the approval request for the reviewer.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl ApprovalRequestRequest {
    pub fn new(operation: String, ) -> ApprovalRequestRequest {
        ApprovalRequestRequest {
            operation: operation,
            method: None,
            body: None,
            description: None,
        }
    }
}


/// Approval request status.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum ApprovalRequestStatus { 
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "DENIED")]
    DENIED,
    #[serde(rename = "FAILED")]
    FAILED,
}

impl ::std::fmt::Display for ApprovalRequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            ApprovalRequestStatus::PENDING => write!(f, "{}", "PENDING"),
            ApprovalRequestStatus::APPROVED => write!(f, "{}", "APPROVED"),
            ApprovalRequestStatus::DENIED => write!(f, "{}", "DENIED"),
            ApprovalRequestStatus::FAILED => write!(f, "{}", "FAILED"),
        }
    }
}

impl ::std::str::FromStr for ApprovalRequestStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING" => Ok(ApprovalRequestStatus::PENDING),
            "APPROVED" => Ok(ApprovalRequestStatus::APPROVED),
            "DENIED" => Ok(ApprovalRequestStatus::DENIED),
            "FAILED" => Ok(ApprovalRequestStatus::FAILED),
            _ => Err(()),
        }
    }
}


/// Approval status
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// Identifies an object acted upon by an approval request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApprovalSubject {
    /// The ID of the workflow being acted upon.
    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<uuid::Uuid>,

}

impl ApprovalSubject {
    pub fn new() -> ApprovalSubject {
        ApprovalSubject {
            workflow: None,
        }
    }
}


/// Optional parameters for approve request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ApproveRequest {
    /// Password is required if the approval policy requires password authentication.
    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    /// U2F is required if the approval policy requires two factor authentication.
    #[serde(rename = "u2f")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub u2f: Option<String>,

    /// Data associated with the approval
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<serde_json::Value>,

}

impl ApproveRequest {
    pub fn new() -> ApproveRequest {
        ApproveRequest {
            password: None,
            u2f: None,
            body: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AttestationRequest {
    /// IAS Quote report bytes.
    #[serde(rename = "ias_quote")]
    pub ias_quote: crate::ByteArray,

    /// Certificate Signing Request bytes.
    #[serde(rename = "csr")]
    pub csr: String,

    /// Node Attestation type (DCAP or EPID)
    #[serde(rename = "attestation_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attestation_type: Option<String>,

}

impl AttestationRequest {
    pub fn new(ias_quote: crate::ByteArray, csr: String, ) -> AttestationRequest {
        AttestationRequest {
            ias_quote: ias_quote,
            csr: csr,
            attestation_type: None,
        }
    }
}


/// Credentials for authenticating to a docker registry
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
pub struct AuthConfigOauth {
    #[serde(rename = "idp_name")]
    pub idp_name: String,

    #[serde(rename = "idp_icon_url")]
    pub idp_icon_url: String,

    #[serde(rename = "idp_authorization_endpoint")]
    pub idp_authorization_endpoint: String,

    #[serde(rename = "idp_token_endpoint")]
    pub idp_token_endpoint: String,

    #[serde(rename = "idp_requires_basic_auth")]
    pub idp_requires_basic_auth: bool,

    #[serde(rename = "idp_userinfo_endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub idp_userinfo_endpoint: Option<String>,

    #[serde(rename = "tls")]
    pub tls: models::TlsConfig,

    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "client_secret")]
    pub client_secret: String,

}

impl AuthConfigOauth {
    pub fn new(idp_name: String, idp_icon_url: String, idp_authorization_endpoint: String, idp_token_endpoint: String, idp_requires_basic_auth: bool, tls: models::TlsConfig, client_id: String, client_secret: String, ) -> AuthConfigOauth {
        AuthConfigOauth {
            idp_name: idp_name,
            idp_icon_url: idp_icon_url,
            idp_authorization_endpoint: idp_authorization_endpoint,
            idp_token_endpoint: idp_token_endpoint,
            idp_requires_basic_auth: idp_requires_basic_auth,
            idp_userinfo_endpoint: None,
            tls: tls,
            client_id: client_id,
            client_secret: client_secret,
        }
    }
}


/// Configuration for password-based authentication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthConfigPassword {
    #[serde(rename = "require_2fa")]
    pub require_2fa: bool,

    #[serde(rename = "administrators_only")]
    pub administrators_only: bool,

}

impl AuthConfigPassword {
    pub fn new(require_2fa: bool, administrators_only: bool, ) -> AuthConfigPassword {
        AuthConfigPassword {
            require_2fa: require_2fa,
            administrators_only: administrators_only,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthConfigRef {
    #[serde(rename = "target_id")]
    pub target_id: String,

}

impl AuthConfigRef {
    pub fn new(target_id: String, ) -> AuthConfigRef {
        AuthConfigRef {
            target_id: target_id,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthDiscoverRequest {
    /// User email
    #[serde(rename = "user_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_email: Option<String>,

}

impl AuthDiscoverRequest {
    pub fn new() -> AuthDiscoverRequest {
        AuthDiscoverRequest {
            user_email: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthDiscoverResponse {
    #[serde(rename = "auth_methods")]
    pub auth_methods: Vec<models::AuthMethod>,

}

impl AuthDiscoverResponse {
    pub fn new(auth_methods: Vec<models::AuthMethod>, ) -> AuthDiscoverResponse {
        AuthDiscoverResponse {
            auth_methods: auth_methods,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthMethod {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<serde_json::Value>,

    #[serde(rename = "oauth_code_grant")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oauth_code_grant: Option<models::OauthAuthCodeGrant>,

}

impl AuthMethod {
    pub fn new() -> AuthMethod {
        AuthMethod {
            password: None,
            oauth_code_grant: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthRequest {
    #[serde(rename = "oauth_auth_code")]
    pub oauth_auth_code: models::OauthCodeData,

}

impl AuthRequest {
    pub fn new(oauth_auth_code: models::OauthCodeData, ) -> AuthRequest {
        AuthRequest {
            oauth_auth_code: oauth_auth_code,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthResponse {
    /// Bearer token to be used to authenticate to other APIs.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

    #[serde(rename = "session_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_info: Option<models::SessionInfo>,

}

impl AuthResponse {
    pub fn new() -> AuthResponse {
        AuthResponse {
            access_token: None,
            session_info: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct AuthenticationConfig {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<models::AuthConfigPassword>,

    #[serde(rename = "oauth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oauth: Option<models::AuthConfigOauth>,

    #[serde(rename = "cluster_auth_ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_auth_ref: Option<models::AuthConfigRef>,

}

impl AuthenticationConfig {
    pub fn new() -> AuthenticationConfig {
        AuthenticationConfig {
            password: None,
            oauth: None,
            cluster_auth_ref: None,
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


/// Detailed info of an application image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Build {
    /// Image Id
    #[serde(rename = "build_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_id: Option<uuid::Uuid>,

    #[serde(rename = "docker_info")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_info: Option<models::DockerInfo>,

    /// Timestamp of image addition to the system.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    /// Timestamp of image updation to the system.
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

    /// Mem size required for the image.
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads required for the image.
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

    #[serde(rename = "advanced_settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub advanced_settings: Option<models::AdvancedSettings>,

    /// image name if curated app.
    #[serde(rename = "build_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub build_name: Option<String>,

    /// UUID of pending build whitelist task for the build
    #[serde(rename = "pending_task_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_task_id: Option<uuid::Uuid>,

    /// Application configurations attached to the image.
    #[serde(rename = "configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configs: Option<HashMap<String, serde_json::Value>>,

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
            pending_task_id: None,
            configs: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct BuildDeploymentStatus {
    #[serde(rename = "status")]
    pub status: models::BuildDeploymentStatusType,

    /// The time when the deployment status changed.
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


/// Status string for the image deployment.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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

    /// Time since the status change.
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


/// Status string for the image.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
pub struct BuildUpdateRequest {
    #[serde(rename = "configs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub configs: Option<HashMap<String, serde_json::Value>>,

}

impl BuildUpdateRequest {
    pub fn new() -> BuildUpdateRequest {
        BuildUpdateRequest {
            configs: None,
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
pub struct CaConfig {
    #[serde(rename = "ca_set")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_set: Option<models::CaSet>,

    #[serde(rename = "pinned")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pinned: Option<Vec<crate::ByteArray>>,

}

impl CaConfig {
    pub fn new() -> CaConfig {
        CaConfig {
            ca_set: None,
            pinned: None,
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum CaSet { 
    #[serde(rename = "GLOBAL_ROOTS")]
    GLOBAL_ROOTS,
}

impl ::std::fmt::Display for CaSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            CaSet::GLOBAL_ROOTS => write!(f, "{}", "GLOBAL_ROOTS"),
        }
    }
}

impl ::std::str::FromStr for CaSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GLOBAL_ROOTS" => Ok(CaSet::GLOBAL_ROOTS),
            _ => Err(()),
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

    /// The certificate signing request.
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// The certificate itself, if issued.
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


/// Certificate status
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
    /// App id of the image.
    #[serde(rename = "app_id")]
    pub app_id: uuid::Uuid,

    /// Common Image docker version for both input and output.
    #[serde(rename = "docker_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_version: Option<String>,

    /// Input Image docker version.
    #[serde(rename = "input_docker_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_docker_version: Option<String>,

    /// Output Image docker version.
    #[serde(rename = "output_docker_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_docker_version: Option<String>,

    #[serde(rename = "inputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "outputAuthConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output_auth_config: Option<models::AuthConfig>,

    #[serde(rename = "authConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_config: Option<models::AuthConfig>,

    /// Enables debug logging from EnclaveOS.
    #[serde(rename = "debug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub debug: Option<bool>,

    /// Mem size required for the image.
    #[serde(rename = "memSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads required for the image.
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub threads: Option<i32>,

}

impl ConvertAppBuildRequest {
    pub fn new(app_id: uuid::Uuid, ) -> ConvertAppBuildRequest {
        ConvertAppBuildRequest {
            app_id: app_id,
            docker_version: None,
            input_docker_version: None,
            output_docker_version: None,
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

    /// mrenclave of the image.
    #[serde(rename = "mrenclave")]
    pub mrenclave: String,

    /// mrsigner of the image.
    #[serde(rename = "mrsigner")]
    pub mrsigner: String,

    /// IsvProdId of the image.
    #[serde(rename = "isvprodid")]
    pub isvprodid: i32,

    /// ISVSVN of the image.
    #[serde(rename = "isvsvn")]
    pub isvsvn: i32,

    /// App id of the image.
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<uuid::Uuid>,

    /// App name of the image.
    #[serde(rename = "app_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_name: Option<String>,

    /// Mem size required for the image.
    #[serde(rename = "mem_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mem_size: Option<i64>,

    /// Threads required for the image.
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
pub struct CreateDatasetRequest {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "credentials")]
    pub credentials: models::DatasetCredentialsRequest,

}

impl CreateDatasetRequest {
    pub fn new(name: String, description: String, labels: HashMap<String, String>, location: String, credentials: models::DatasetCredentialsRequest, ) -> CreateDatasetRequest {
        CreateDatasetRequest {
            name: name,
            description: description,
            labels: labels,
            location: location,
            credentials: credentials,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CreateFinalWorkflowGraph {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "contents")]
    pub contents: models::CreateWorkflowVersionRequest,

}

impl CreateFinalWorkflowGraph {
    pub fn new(name: String, description: String, contents: models::CreateWorkflowVersionRequest, ) -> CreateFinalWorkflowGraph {
        CreateFinalWorkflowGraph {
            name: name,
            description: description,
            contents: contents,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CreateWorkflowGraph {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "objects")]
    pub objects: SortedHashMap<String, models::WorkflowObject>,

    #[serde(rename = "edges")]
    pub edges: SortedHashMap<String, models::WorkflowEdge>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::WorkflowMetadata>,

}

impl CreateWorkflowGraph {
    pub fn new(name: String, description: String, objects: SortedHashMap<String, models::WorkflowObject>, edges: SortedHashMap<String, models::WorkflowEdge>, ) -> CreateWorkflowGraph {
        CreateWorkflowGraph {
            name: name,
            description: description,
            objects: objects,
            edges: edges,
            metadata: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CreateWorkflowVersionRequest {
    #[serde(rename = "objects")]
    pub objects: SortedHashMap<String, models::WorkflowObject>,

    #[serde(rename = "edges")]
    pub edges: SortedHashMap<String, models::WorkflowEdge>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::WorkflowMetadata>,

}

impl CreateWorkflowVersionRequest {
    pub fn new(objects: SortedHashMap<String, models::WorkflowObject>, edges: SortedHashMap<String, models::WorkflowEdge>, ) -> CreateWorkflowVersionRequest {
        CreateWorkflowVersionRequest {
            objects: objects,
            edges: edges,
            metadata: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct CredentialType {
    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<models::AuthConfig>,

}

impl CredentialType {
    pub fn new() -> CredentialType {
        CredentialType {
            default: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Dataset {
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "owner")]
    pub owner: uuid::Uuid,

    /// Dataset creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Last update timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "labels")]
    pub labels: HashMap<String, String>,

    #[serde(rename = "credentials")]
    pub credentials: models::DatasetCredentials,

}

impl Dataset {
    pub fn new(dataset_id: uuid::Uuid, name: String, owner: uuid::Uuid, created_at: i64, updated_at: i64, description: String, location: String, labels: HashMap<String, String>, credentials: models::DatasetCredentials, ) -> Dataset {
        Dataset {
            dataset_id: dataset_id,
            name: name,
            owner: owner,
            created_at: created_at,
            updated_at: updated_at,
            description: description,
            location: location,
            labels: labels,
            credentials: credentials,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DatasetCredentials {
    #[serde(rename = "sdkms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sdkms: Option<models::SdkmsCredentials>,

}

impl DatasetCredentials {
    pub fn new() -> DatasetCredentials {
        DatasetCredentials {
            sdkms: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DatasetCredentialsRequest {
    #[serde(rename = "contents")]
    pub contents: String,

}

impl DatasetCredentialsRequest {
    pub fn new(contents: String, ) -> DatasetCredentialsRequest {
        DatasetCredentialsRequest {
            contents: contents,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DatasetUpdateRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credentials: Option<models::DatasetCredentialsRequest>,

}

impl DatasetUpdateRequest {
    pub fn new() -> DatasetUpdateRequest {
        DatasetUpdateRequest {
            name: None,
            description: None,
            labels: None,
            location: None,
            credentials: None,
        }
    }
}


/// Optional parameters for deny request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DenyRequest {
    /// Reason associated with the denial
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

}

impl DenyRequest {
    pub fn new() -> DenyRequest {
        DenyRequest {
            reason: None,
        }
    }
}


/// Docker info of an image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct DockerInfo {
    /// Image docker image name.
    #[serde(rename = "docker_image_name")]
    pub docker_image_name: String,

    /// image docker version.
    #[serde(rename = "docker_version")]
    pub docker_version: String,

    /// Build docker image sha.
    #[serde(rename = "docker_image_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker_image_sha: Option<String>,

    /// Docker image size in kb.
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


/// Info on a application enclave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct EnclaveInfo {
    /// mrenclave of an image, as a hex string.
    #[serde(rename = "mrenclave")]
    pub mrenclave: String,

    /// mr signer of an image, as a hex string.
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


/// An app, user, or plugin ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Entity {
    /// The user ID of the user who created this entity, if this entity was created by a user.
    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<uuid::Uuid>,

}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            user: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Event {
    /// Event Message
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "code")]
    pub code: models::EventType,

    #[serde(rename = "severity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub severity: Option<models::EventSeverity>,

}

impl Event {
    pub fn new(message: String, code: models::EventType, ) -> Event {
        Event {
            message: message,
            code: code,
            severity: None,
        }
    }
}


/// Event action type.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// Event actor type.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// Event severity
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// String enumeration identifying the event
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum EventType { 
    #[serde(rename = "BAD_REQUEST")]
    BAD_REQUEST,
    #[serde(rename = "NODE_NOT_ENROLLED")]
    NODE_NOT_ENROLLED,
    #[serde(rename = "INVALID_NAME")]
    INVALID_NAME,
    #[serde(rename = "INVALID_VALUE")]
    INVALID_VALUE,
    #[serde(rename = "UN_AUTHORIZED")]
    UN_AUTHORIZED,
    #[serde(rename = "NO_ACCOUNT_SELECTED")]
    NO_ACCOUNT_SELECTED,
    #[serde(rename = "NO_ZONE_SELECTED")]
    NO_ZONE_SELECTED,
    #[serde(rename = "ATTESTATION_REQUIRED")]
    ATTESTATION_REQUIRED,
    #[serde(rename = "NOT_FOUND")]
    NOT_FOUND,
    #[serde(rename = "UNIQUE_VIOLATION")]
    UNIQUE_VIOLATION,
    #[serde(rename = "KEY_UNIQUE_VIOLATION")]
    KEY_UNIQUE_VIOLATION,
    #[serde(rename = "INVALID_STATE")]
    INVALID_STATE,
    #[serde(rename = "USER_ALREADY_EXISTS")]
    USER_ALREADY_EXISTS,
    #[serde(rename = "FORBIDDEN")]
    FORBIDDEN,
    #[serde(rename = "AUTH_FAILED")]
    AUTH_FAILED,
    #[serde(rename = "INVALID_SESSION")]
    INVALID_SESSION,
    #[serde(rename = "SESSION_EXPIRED")]
    SESSION_EXPIRED,
    #[serde(rename = "CERT_PARSE_ERROR")]
    CERT_PARSE_ERROR,
    #[serde(rename = "QUOTA_EXCEEDED")]
    QUOTA_EXCEEDED,
    #[serde(rename = "USER_ACCOUNT_PENDING")]
    USER_ACCOUNT_PENDING,
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    INTERNAL_SERVER_ERROR,
    #[serde(rename = "MISSING_REQUIRED_PARAMETER")]
    MISSING_REQUIRED_PARAMETER,
    #[serde(rename = "INVALID_PATH_PARAMETER")]
    INVALID_PATH_PARAMETER,
    #[serde(rename = "INVALID_HEADER")]
    INVALID_HEADER,
    #[serde(rename = "INVALID_QUERY_PARAMETER")]
    INVALID_QUERY_PARAMETER,
    #[serde(rename = "INVALID_BODY_PARAMETER")]
    INVALID_BODY_PARAMETER,
    #[serde(rename = "METHOD_NOT_ALLOWED")]
    METHOD_NOT_ALLOWED,
    #[serde(rename = "LATEST_EULA_NOT_ACCEPTED")]
    LATEST_EULA_NOT_ACCEPTED,
    #[serde(rename = "CONFLICT")]
    CONFLICT,
    #[serde(rename = "DCAP_ARTIFACT_RETRIEVAL_ERROR")]
    DCAP_ARTIFACT_RETRIEVAL_ERROR,
    #[serde(rename = "DCAP_ERROR")]
    DCAP_ERROR,
    #[serde(rename = "DCAP_ARTIFACT_SERIALIZATION_ERROR")]
    DCAP_ARTIFACT_SERIALIZATION_ERROR,
    #[serde(rename = "DCAP_ARTIFACT_DESERIALIZATION_ERROR")]
    DCAP_ARTIFACT_DESERIALIZATION_ERROR,
    #[serde(rename = "LOCKED")]
    LOCKED,
    #[serde(rename = "UNDERGOING_MAINTENANCE")]
    UNDERGOING_MAINTENANCE,
}

impl ::std::fmt::Display for EventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            EventType::BAD_REQUEST => write!(f, "{}", "BAD_REQUEST"),
            EventType::NODE_NOT_ENROLLED => write!(f, "{}", "NODE_NOT_ENROLLED"),
            EventType::INVALID_NAME => write!(f, "{}", "INVALID_NAME"),
            EventType::INVALID_VALUE => write!(f, "{}", "INVALID_VALUE"),
            EventType::UN_AUTHORIZED => write!(f, "{}", "UN_AUTHORIZED"),
            EventType::NO_ACCOUNT_SELECTED => write!(f, "{}", "NO_ACCOUNT_SELECTED"),
            EventType::NO_ZONE_SELECTED => write!(f, "{}", "NO_ZONE_SELECTED"),
            EventType::ATTESTATION_REQUIRED => write!(f, "{}", "ATTESTATION_REQUIRED"),
            EventType::NOT_FOUND => write!(f, "{}", "NOT_FOUND"),
            EventType::UNIQUE_VIOLATION => write!(f, "{}", "UNIQUE_VIOLATION"),
            EventType::KEY_UNIQUE_VIOLATION => write!(f, "{}", "KEY_UNIQUE_VIOLATION"),
            EventType::INVALID_STATE => write!(f, "{}", "INVALID_STATE"),
            EventType::USER_ALREADY_EXISTS => write!(f, "{}", "USER_ALREADY_EXISTS"),
            EventType::FORBIDDEN => write!(f, "{}", "FORBIDDEN"),
            EventType::AUTH_FAILED => write!(f, "{}", "AUTH_FAILED"),
            EventType::INVALID_SESSION => write!(f, "{}", "INVALID_SESSION"),
            EventType::SESSION_EXPIRED => write!(f, "{}", "SESSION_EXPIRED"),
            EventType::CERT_PARSE_ERROR => write!(f, "{}", "CERT_PARSE_ERROR"),
            EventType::QUOTA_EXCEEDED => write!(f, "{}", "QUOTA_EXCEEDED"),
            EventType::USER_ACCOUNT_PENDING => write!(f, "{}", "USER_ACCOUNT_PENDING"),
            EventType::INTERNAL_SERVER_ERROR => write!(f, "{}", "INTERNAL_SERVER_ERROR"),
            EventType::MISSING_REQUIRED_PARAMETER => write!(f, "{}", "MISSING_REQUIRED_PARAMETER"),
            EventType::INVALID_PATH_PARAMETER => write!(f, "{}", "INVALID_PATH_PARAMETER"),
            EventType::INVALID_HEADER => write!(f, "{}", "INVALID_HEADER"),
            EventType::INVALID_QUERY_PARAMETER => write!(f, "{}", "INVALID_QUERY_PARAMETER"),
            EventType::INVALID_BODY_PARAMETER => write!(f, "{}", "INVALID_BODY_PARAMETER"),
            EventType::METHOD_NOT_ALLOWED => write!(f, "{}", "METHOD_NOT_ALLOWED"),
            EventType::LATEST_EULA_NOT_ACCEPTED => write!(f, "{}", "LATEST_EULA_NOT_ACCEPTED"),
            EventType::CONFLICT => write!(f, "{}", "CONFLICT"),
            EventType::DCAP_ARTIFACT_RETRIEVAL_ERROR => write!(f, "{}", "DCAP_ARTIFACT_RETRIEVAL_ERROR"),
            EventType::DCAP_ERROR => write!(f, "{}", "DCAP_ERROR"),
            EventType::DCAP_ARTIFACT_SERIALIZATION_ERROR => write!(f, "{}", "DCAP_ARTIFACT_SERIALIZATION_ERROR"),
            EventType::DCAP_ARTIFACT_DESERIALIZATION_ERROR => write!(f, "{}", "DCAP_ARTIFACT_DESERIALIZATION_ERROR"),
            EventType::LOCKED => write!(f, "{}", "LOCKED"),
            EventType::UNDERGOING_MAINTENANCE => write!(f, "{}", "UNDERGOING_MAINTENANCE"),
        }
    }
}

impl ::std::str::FromStr for EventType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BAD_REQUEST" => Ok(EventType::BAD_REQUEST),
            "NODE_NOT_ENROLLED" => Ok(EventType::NODE_NOT_ENROLLED),
            "INVALID_NAME" => Ok(EventType::INVALID_NAME),
            "INVALID_VALUE" => Ok(EventType::INVALID_VALUE),
            "UN_AUTHORIZED" => Ok(EventType::UN_AUTHORIZED),
            "NO_ACCOUNT_SELECTED" => Ok(EventType::NO_ACCOUNT_SELECTED),
            "NO_ZONE_SELECTED" => Ok(EventType::NO_ZONE_SELECTED),
            "ATTESTATION_REQUIRED" => Ok(EventType::ATTESTATION_REQUIRED),
            "NOT_FOUND" => Ok(EventType::NOT_FOUND),
            "UNIQUE_VIOLATION" => Ok(EventType::UNIQUE_VIOLATION),
            "KEY_UNIQUE_VIOLATION" => Ok(EventType::KEY_UNIQUE_VIOLATION),
            "INVALID_STATE" => Ok(EventType::INVALID_STATE),
            "USER_ALREADY_EXISTS" => Ok(EventType::USER_ALREADY_EXISTS),
            "FORBIDDEN" => Ok(EventType::FORBIDDEN),
            "AUTH_FAILED" => Ok(EventType::AUTH_FAILED),
            "INVALID_SESSION" => Ok(EventType::INVALID_SESSION),
            "SESSION_EXPIRED" => Ok(EventType::SESSION_EXPIRED),
            "CERT_PARSE_ERROR" => Ok(EventType::CERT_PARSE_ERROR),
            "QUOTA_EXCEEDED" => Ok(EventType::QUOTA_EXCEEDED),
            "USER_ACCOUNT_PENDING" => Ok(EventType::USER_ACCOUNT_PENDING),
            "INTERNAL_SERVER_ERROR" => Ok(EventType::INTERNAL_SERVER_ERROR),
            "MISSING_REQUIRED_PARAMETER" => Ok(EventType::MISSING_REQUIRED_PARAMETER),
            "INVALID_PATH_PARAMETER" => Ok(EventType::INVALID_PATH_PARAMETER),
            "INVALID_HEADER" => Ok(EventType::INVALID_HEADER),
            "INVALID_QUERY_PARAMETER" => Ok(EventType::INVALID_QUERY_PARAMETER),
            "INVALID_BODY_PARAMETER" => Ok(EventType::INVALID_BODY_PARAMETER),
            "METHOD_NOT_ALLOWED" => Ok(EventType::METHOD_NOT_ALLOWED),
            "LATEST_EULA_NOT_ACCEPTED" => Ok(EventType::LATEST_EULA_NOT_ACCEPTED),
            "CONFLICT" => Ok(EventType::CONFLICT),
            "DCAP_ARTIFACT_RETRIEVAL_ERROR" => Ok(EventType::DCAP_ARTIFACT_RETRIEVAL_ERROR),
            "DCAP_ERROR" => Ok(EventType::DCAP_ERROR),
            "DCAP_ARTIFACT_SERIALIZATION_ERROR" => Ok(EventType::DCAP_ARTIFACT_SERIALIZATION_ERROR),
            "DCAP_ARTIFACT_DESERIALIZATION_ERROR" => Ok(EventType::DCAP_ARTIFACT_DESERIALIZATION_ERROR),
            "LOCKED" => Ok(EventType::LOCKED),
            "UNDERGOING_MAINTENANCE" => Ok(EventType::UNDERGOING_MAINTENANCE),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct FinalWorkflow {
    #[serde(rename = "graph_id")]
    pub graph_id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    /// Dataset creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Last update timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "versions")]
    pub versions: HashMap<String, models::FinalWorkflowGraph>,

}

impl FinalWorkflow {
    pub fn new(graph_id: uuid::Uuid, name: String, created_at: i64, updated_at: i64, description: String, versions: HashMap<String, models::FinalWorkflowGraph>, ) -> FinalWorkflow {
        FinalWorkflow {
            graph_id: graph_id,
            name: name,
            created_at: created_at,
            updated_at: updated_at,
            description: description,
            versions: versions,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct FinalWorkflowGraph {
    /// Dataset creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    #[serde(rename = "objects")]
    pub objects: SortedHashMap<String, models::WorkflowObject>,

    #[serde(rename = "edges")]
    pub edges: SortedHashMap<String, models::WorkflowEdge>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::WorkflowMetadata>,

    #[serde(rename = "runtime_configs")]
    pub runtime_configs: SortedHashMap<String, models::WorkflowObjectRefApp>,

}

impl FinalWorkflowGraph {
    pub fn new(created_at: i64, objects: SortedHashMap<String, models::WorkflowObject>, edges: SortedHashMap<String, models::WorkflowEdge>, runtime_configs: SortedHashMap<String, models::WorkflowObjectRefApp>, ) -> FinalWorkflowGraph {
        FinalWorkflowGraph {
            created_at: created_at,
            objects: objects,
            edges: edges,
            metadata: None,
            runtime_configs: runtime_configs,
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
pub struct GetAllApplicationConfigsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::ApplicationConfigResponse>,

}

impl GetAllApplicationConfigsResponse {
    pub fn new(items: Vec<models::ApplicationConfigResponse>, ) -> GetAllApplicationConfigsResponse {
        GetAllApplicationConfigsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllApprovalRequests {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::ApprovalRequest>,

}

impl GetAllApprovalRequests {
    pub fn new(items: Vec<models::ApprovalRequest>, ) -> GetAllApprovalRequests {
        GetAllApprovalRequests {
            metadata: None,
            items: items,
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
pub struct GetAllDatasetsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::Dataset>,

}

impl GetAllDatasetsResponse {
    pub fn new(items: Vec<models::Dataset>, ) -> GetAllDatasetsResponse {
        GetAllDatasetsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetAllFinalWorkflowGraphsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::FinalWorkflow>,

}

impl GetAllFinalWorkflowGraphsResponse {
    pub fn new(items: Vec<models::FinalWorkflow>, ) -> GetAllFinalWorkflowGraphsResponse {
        GetAllFinalWorkflowGraphsResponse {
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
pub struct GetAllWorkflowGraphsResponse {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::SearchMetadata>,

    #[serde(rename = "items")]
    pub items: Vec<models::WorkflowGraph>,

}

impl GetAllWorkflowGraphsResponse {
    pub fn new(items: Vec<models::WorkflowGraph>, ) -> GetAllWorkflowGraphsResponse {
        GetAllWorkflowGraphsResponse {
            metadata: None,
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct GetPckCertResponse {
    /// Pck Certificate(PEM-encoded)
    #[serde(rename = "pck_cert")]
    pub pck_cert: crate::ByteArray,

}

impl GetPckCertResponse {
    pub fn new(pck_cert: crate::ByteArray, ) -> GetPckCertResponse {
        GetPckCertResponse {
            pck_cert: pck_cert,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct HashedConfig {
    #[serde(rename = "app_config")]
    pub app_config: SortedHashMap<String, models::ApplicationConfigContents>,

    #[serde(rename = "labels")]
    pub labels: SortedHashMap<String, String>,

    #[serde(rename = "zone_ca")]
    pub zone_ca: SortedVec<String>,

    #[serde(rename = "workflow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow: Option<models::ApplicationConfigWorkflow>,

}

impl HashedConfig {
    pub fn new(app_config: SortedHashMap<String, models::ApplicationConfigContents>, labels: SortedHashMap<String, String>, zone_ca: SortedVec<String>, ) -> HashedConfig {
        HashedConfig {
            app_config: app_config,
            labels: labels,
            zone_ca: zone_ca,
            workflow: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ImageRegistryResponse {
    #[serde(rename = "registry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registry: Option<models::Registry>,

}

impl ImageRegistryResponse {
    pub fn new() -> ImageRegistryResponse {
        ImageRegistryResponse {
            registry: None,
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


/// Java runtime mode for conversion.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
pub struct LabelCount {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "count")]
    pub count: i32,

}

impl LabelCount {
    pub fn new(key: String, value: String, count: i32, ) -> LabelCount {
        LabelCount {
            key: key,
            value: value,
            count: count,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct LabelsCount {
    #[serde(rename = "items")]
    pub items: Vec<models::LabelCount>,

}

impl LabelsCount {
    pub fn new(items: Vec<models::LabelCount>, ) -> LabelsCount {
        LabelsCount {
            items: items,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NewCertificateRequest {
    /// Certificate signing request.
    #[serde(rename = "csr")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csr: Option<String>,

    /// Compute Node Id for the requesting host agent
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
    /// Name of the compute node.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the compute node.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The account ID of the account that this compute node belongs to.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// IP Address of the compute node.
    #[serde(rename = "ipaddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ipaddress: Option<String>,

    /// UUID for the compute node.
    #[serde(rename = "node_id")]
    pub node_id: uuid::Uuid,

    /// No longer used.
    #[serde(rename = "host_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_id: Option<String>,

    /// Zone ID of the zone this compute node belongs to.
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zone_id: Option<uuid::Uuid>,

    #[serde(rename = "status")]
    pub status: models::NodeStatus,

    /// The compute node attestation date
    #[serde(rename = "attested_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attested_at: Option<i64>,

    /// The compute node attestation certificate
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<String>,

    /// Apps associated with the compute node.
    #[serde(rename = "apps")]
    pub apps: Vec<models::AppNodeInfo>,

    #[serde(rename = "sgx_info")]
    pub sgx_info: models::SgxInfo,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    /// Platform information of the compute node.
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub platform: Option<String>,

    /// Node Attestation type (DCAP or EPID)
    #[serde(rename = "attestation_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attestation_type: Option<String>,

    #[serde(rename = "error_report")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_report: Option<models::NodeErrorReport>,

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
            labels: None,
            platform: None,
            attestation_type: None,
            error_report: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeErrorReport {
    /// Error message containing the reason why node agent failed
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "name")]
    pub name: models::NodeProvisionErrorType,

}

impl NodeErrorReport {
    pub fn new(message: String, name: models::NodeProvisionErrorType, ) -> NodeErrorReport {
        NodeErrorReport {
            message: message,
            name: name,
        }
    }
}


/// Node agent attestation error type
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum NodeProvisionErrorType { 
    #[serde(rename = "AESMD_FAILURE")]
    AESMD_FAILURE,
    #[serde(rename = "QUOTE_GENERATION_ERROR")]
    QUOTE_GENERATION_ERROR,
    #[serde(rename = "QUOTE_VERIFICATION_ERROR")]
    QUOTE_VERIFICATION_ERROR,
    #[serde(rename = "GROUP_OUT_OF_DATE")]
    GROUP_OUT_OF_DATE,
    #[serde(rename = "SIGRL_VERSION_MISMATCH")]
    SIGRL_VERSION_MISMATCH,
    #[serde(rename = "CONFIGURATION_NEEDED")]
    CONFIGURATION_NEEDED,
    #[serde(rename = "QUOTE_REVOKED")]
    QUOTE_REVOKED,
    #[serde(rename = "SIGNATURE_INVALID")]
    SIGNATURE_INVALID,
    #[serde(rename = "DCAP_ERROR")]
    DCAP_ERROR,
    #[serde(rename = "CPUSVN_OUT_OF_DATE")]
    CPUSVN_OUT_OF_DATE,
    #[serde(rename = "PSW_OUT_OF_DATE")]
    PSW_OUT_OF_DATE,
    #[serde(rename = "BAD_PSW")]
    BAD_PSW,
    #[serde(rename = "BAD DATA")]
    BAD_DATA,
}

impl ::std::fmt::Display for NodeProvisionErrorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            NodeProvisionErrorType::AESMD_FAILURE => write!(f, "{}", "AESMD_FAILURE"),
            NodeProvisionErrorType::QUOTE_GENERATION_ERROR => write!(f, "{}", "QUOTE_GENERATION_ERROR"),
            NodeProvisionErrorType::QUOTE_VERIFICATION_ERROR => write!(f, "{}", "QUOTE_VERIFICATION_ERROR"),
            NodeProvisionErrorType::GROUP_OUT_OF_DATE => write!(f, "{}", "GROUP_OUT_OF_DATE"),
            NodeProvisionErrorType::SIGRL_VERSION_MISMATCH => write!(f, "{}", "SIGRL_VERSION_MISMATCH"),
            NodeProvisionErrorType::CONFIGURATION_NEEDED => write!(f, "{}", "CONFIGURATION_NEEDED"),
            NodeProvisionErrorType::QUOTE_REVOKED => write!(f, "{}", "QUOTE_REVOKED"),
            NodeProvisionErrorType::SIGNATURE_INVALID => write!(f, "{}", "SIGNATURE_INVALID"),
            NodeProvisionErrorType::DCAP_ERROR => write!(f, "{}", "DCAP_ERROR"),
            NodeProvisionErrorType::CPUSVN_OUT_OF_DATE => write!(f, "{}", "CPUSVN_OUT_OF_DATE"),
            NodeProvisionErrorType::PSW_OUT_OF_DATE => write!(f, "{}", "PSW_OUT_OF_DATE"),
            NodeProvisionErrorType::BAD_PSW => write!(f, "{}", "BAD_PSW"),
            NodeProvisionErrorType::BAD_DATA => write!(f, "{}", "BAD DATA"),
        }
    }
}

impl ::std::str::FromStr for NodeProvisionErrorType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AESMD_FAILURE" => Ok(NodeProvisionErrorType::AESMD_FAILURE),
            "QUOTE_GENERATION_ERROR" => Ok(NodeProvisionErrorType::QUOTE_GENERATION_ERROR),
            "QUOTE_VERIFICATION_ERROR" => Ok(NodeProvisionErrorType::QUOTE_VERIFICATION_ERROR),
            "GROUP_OUT_OF_DATE" => Ok(NodeProvisionErrorType::GROUP_OUT_OF_DATE),
            "SIGRL_VERSION_MISMATCH" => Ok(NodeProvisionErrorType::SIGRL_VERSION_MISMATCH),
            "CONFIGURATION_NEEDED" => Ok(NodeProvisionErrorType::CONFIGURATION_NEEDED),
            "QUOTE_REVOKED" => Ok(NodeProvisionErrorType::QUOTE_REVOKED),
            "SIGNATURE_INVALID" => Ok(NodeProvisionErrorType::SIGNATURE_INVALID),
            "DCAP_ERROR" => Ok(NodeProvisionErrorType::DCAP_ERROR),
            "CPUSVN_OUT_OF_DATE" => Ok(NodeProvisionErrorType::CPUSVN_OUT_OF_DATE),
            "PSW_OUT_OF_DATE" => Ok(NodeProvisionErrorType::PSW_OUT_OF_DATE),
            "BAD_PSW" => Ok(NodeProvisionErrorType::BAD_PSW),
            "BAD DATA" => Ok(NodeProvisionErrorType::BAD_DATA),
            _ => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeProvisionRequest {
    /// Name of the compute node.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the compute node
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// IP Address of the compute node.
    #[serde(rename = "ipaddress")]
    pub ipaddress: String,

    /// No longer used.
    #[serde(rename = "host_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub host_id: Option<String>,

    /// Version of the Intel SGX Platform Software running on the compute node
    #[serde(rename = "sgx_version")]
    pub sgx_version: String,

    #[serde(rename = "attestation_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attestation_request: Option<models::AttestationRequest>,

    #[serde(rename = "error_report")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_report: Option<models::NodeErrorReport>,

}

impl NodeProvisionRequest {
    pub fn new(name: String, ipaddress: String, sgx_version: String, ) -> NodeProvisionRequest {
        NodeProvisionRequest {
            name: name,
            description: None,
            ipaddress: ipaddress,
            host_id: None,
            sgx_version: sgx_version,
            attestation_request: None,
            error_report: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeStatus {
    #[serde(rename = "status")]
    pub status: models::NodeStatusType,

    /// Compute node creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Time since the status changed.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: i64,

    /// Time the node was last seen.
    #[serde(rename = "last_seen_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_seen_at: Option<i64>,

    /// Version of the node when it was last seen.
    #[serde(rename = "last_seen_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_seen_version: Option<String>,

}

impl NodeStatus {
    pub fn new(status: models::NodeStatusType, created_at: i64, status_updated_at: i64, ) -> NodeStatus {
        NodeStatus {
            status: status,
            created_at: created_at,
            status_updated_at: status_updated_at,
            last_seen_at: None,
            last_seen_version: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct NodeStatusRequest {
    /// Hostname of the compute node.
    #[serde(rename = "name")]
    pub name: String,

    /// Description of the compute node.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// IP Address of the compute node.
    #[serde(rename = "ipaddress")]
    pub ipaddress: String,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<models::NodeStatus>,

    /// Version of the Intel SGX Platform Software running on the compute node
    #[serde(rename = "sgx_version")]
    pub sgx_version: String,

}

impl NodeStatusRequest {
    pub fn new(name: String, ipaddress: String, sgx_version: String, ) -> NodeStatusRequest {
        NodeStatusRequest {
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
pub struct NodeStatusResponse {
    /// Interval between node agent checkins with the backend, in seconds
    #[serde(rename = "node_refresh_interval")]
    pub node_refresh_interval: i64,

    /// The node agent requests certificate renewal when the certificate's remaining validity is less than this percentage of the original validity
    #[serde(rename = "node_renewal_threshold")]
    pub node_renewal_threshold: i32,

}

impl NodeStatusResponse {
    pub fn new(node_refresh_interval: i64, node_renewal_threshold: i32, ) -> NodeStatusResponse {
        NodeStatusResponse {
            node_refresh_interval: node_refresh_interval,
            node_renewal_threshold: node_renewal_threshold,
        }
    }
}


/// Status string for the compute node.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
    #[serde(rename = "patch")]
    pub patch: Vec<models::PatchDocument>,

}

impl NodeUpdateRequest {
    pub fn new(patch: Vec<models::PatchDocument>, ) -> NodeUpdateRequest {
        NodeUpdateRequest {
            patch: patch,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OauthAuthCodeGrant {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "icon_url")]
    pub icon_url: String,

    #[serde(rename = "authorization_url")]
    pub authorization_url: String,

    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,

    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "idp_id")]
    pub idp_id: crate::ByteArray,

}

impl OauthAuthCodeGrant {
    pub fn new(name: String, icon_url: String, authorization_url: String, client_id: String, redirect_uri: String, state: String, idp_id: crate::ByteArray, ) -> OauthAuthCodeGrant {
        OauthAuthCodeGrant {
            name: name,
            icon_url: icon_url,
            authorization_url: authorization_url,
            client_id: client_id,
            redirect_uri: redirect_uri,
            state: state,
            idp_id: idp_id,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct OauthCodeData {
    #[serde(rename = "idp_id")]
    pub idp_id: crate::ByteArray,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "email")]
    pub email: String,

}

impl OauthCodeData {
    pub fn new(idp_id: crate::ByteArray, code: String, email: String, ) -> OauthCodeData {
        OauthCodeData {
            idp_id: idp_id,
            code: code,
            email: email,
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


/// A JSONPatch document as defined by RFC 6902. The patch operation is subset of what is supported by RFC 6902.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct PatchDocument {
    #[serde(rename = "op")]
    pub op: models::PatchOperation,

    /// It is JSON pointer indicating a field to be updated
    #[serde(rename = "path")]
    pub path: String,

    /// It is the value to be used for the field as indicated by op and path
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<serde_json::Value>,

}

impl PatchDocument {
    pub fn new(op: models::PatchOperation, path: String, ) -> PatchDocument {
        PatchDocument {
            op: op,
            path: path,
            value: None,
        }
    }
}


/// The operation to be performed
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGenericEnum))]
pub enum PatchOperation { 
    #[serde(rename = "add")]
    ADD,
    #[serde(rename = "remove")]
    REMOVE,
    #[serde(rename = "replace")]
    REPLACE,
}

impl ::std::fmt::Display for PatchOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            PatchOperation::ADD => write!(f, "{}", "add"),
            PatchOperation::REMOVE => write!(f, "{}", "remove"),
            PatchOperation::REPLACE => write!(f, "{}", "replace"),
        }
    }
}

impl ::std::str::FromStr for PatchOperation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(PatchOperation::ADD),
            "remove" => Ok(PatchOperation::REMOVE),
            "replace" => Ok(PatchOperation::REPLACE),
            _ => Err(()),
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
pub struct RefreshResponse {
    #[serde(rename = "session_info")]
    pub session_info: models::SessionInfo,

}

impl RefreshResponse {
    pub fn new(session_info: models::SessionInfo, ) -> RefreshResponse {
        RefreshResponse {
            session_info: session_info,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Registry {
    /// URL of the registry
    #[serde(rename = "url")]
    pub url: String,

    /// UUID of the registry
    #[serde(rename = "registry_id")]
    pub registry_id: uuid::Uuid,

    /// Description of the registry
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Username of the registry
    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

}

impl Registry {
    pub fn new(url: String, registry_id: uuid::Uuid, ) -> Registry {
        Registry {
            url: url,
            registry_id: registry_id,
            description: None,
            username: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct RegistryRequest {
    /// URL of the registry
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "credential")]
    pub credential: models::CredentialType,

    /// Description of the registry
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}

impl RegistryRequest {
    pub fn new(url: String, credential: models::CredentialType, ) -> RegistryRequest {
        RegistryRequest {
            url: url,
            credential: credential,
            description: None,
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


/// Type of requester
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct RuntimeAppConfig {
    #[serde(rename = "config")]
    pub config: models::HashedConfig,

    #[serde(rename = "extra")]
    pub extra: models::ApplicationConfigExtra,

}

impl RuntimeAppConfig {
    pub fn new(config: models::HashedConfig, extra: models::ApplicationConfigExtra, ) -> RuntimeAppConfig {
        RuntimeAppConfig {
            config: config,
            extra: extra,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SdkmsCredentials {
    #[serde(rename = "credentials_url")]
    pub credentials_url: String,

    #[serde(rename = "credentials_key_name")]
    pub credentials_key_name: String,

}

impl SdkmsCredentials {
    pub fn new(credentials_url: String, credentials_key_name: String, ) -> SdkmsCredentials {
        SdkmsCredentials {
            credentials_url: credentials_url,
            credentials_key_name: credentials_key_name,
        }
    }
}


/// Configures an SDKMS signing key. The key must be an RSA key with public exponent 3. 
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

    /// Total pages as per the item counts and page limit.
    #[serde(rename = "pages")]
    pub pages: isize,

    /// Number of items to limit in a page.
    #[serde(rename = "limit")]
    pub limit: isize,

    /// Total number of unfiltered items.
    #[serde(rename = "total_count")]
    pub total_count: isize,

    /// Total number of items as per the current filter.
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
pub struct SelectAccountResponse {
    #[serde(rename = "session_info")]
    pub session_info: models::SessionInfo,

}

impl SelectAccountResponse {
    pub fn new(session_info: models::SessionInfo, ) -> SelectAccountResponse {
        SelectAccountResponse {
            session_info: session_info,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SessionInfo {
    #[serde(rename = "subject_id")]
    pub subject_id: uuid::Uuid,

    /// Timestamp of when session will expire
    #[serde(rename = "session_expires_at")]
    pub session_expires_at: i64,

    /// Timestamp of when session token will expire
    #[serde(rename = "session_token_expires_at")]
    pub session_token_expires_at: i64,

}

impl SessionInfo {
    pub fn new(subject_id: uuid::Uuid, session_expires_at: i64, session_token_expires_at: i64, ) -> SessionInfo {
        SessionInfo {
            subject_id: subject_id,
            session_expires_at: session_expires_at,
            session_token_expires_at: session_token_expires_at,
        }
    }
}


/// SGX Related details of a compute node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct SgxInfo {
    /// Version of the Intel SGX Platform Software running on the compute node
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


/// Configures a key to sign the converted image
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

    #[serde(rename = "recaptcha_response")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recaptcha_response: Option<String>,

}

impl SignupRequest {
    pub fn new(user_email: String, user_password: String, ) -> SignupRequest {
        SignupRequest {
            user_email: user_email,
            user_password: user_password,
            first_name: None,
            last_name: None,
            recaptcha_response: None,
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

    /// Certificate Id in case of certificate issuance task.
    #[serde(rename = "certificate_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<uuid::Uuid>,

    /// Compute Node Id
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


/// Status info for a task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TaskStatus {
    /// Task creation time
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Time since the status change.
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


/// Status string for a task.
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


/// The types of tasks supported.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
pub struct TlsConfig {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<serde_json::Value>,

    #[serde(rename = "opportunistic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub opportunistic: Option<serde_json::Value>,

    #[serde(rename = "required")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<models::TlsConfigRequired>,

}

impl TlsConfig {
    pub fn new() -> TlsConfig {
        TlsConfig {
            disabled: None,
            opportunistic: None,
            required: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct TlsConfigRequired {
    #[serde(rename = "validate_hostname")]
    pub validate_hostname: bool,

    #[serde(rename = "client_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_key: Option<crate::ByteArray>,

    #[serde(rename = "client_cert")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_cert: Option<crate::ByteArray>,

    #[serde(rename = "ca")]
    pub ca: models::CaConfig,

}

impl TlsConfigRequired {
    pub fn new(validate_hostname: bool, ca: models::CaConfig, ) -> TlsConfigRequired {
        TlsConfigRequired {
            validate_hostname: validate_hostname,
            client_key: None,
            client_cert: None,
            ca: ca,
        }
    }
}


/// Update an application config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct UpdateApplicationConfigRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "ports")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ports: Option<SortedVec<String>>,

}

impl UpdateApplicationConfigRequest {
    pub fn new() -> UpdateApplicationConfigRequest {
        UpdateApplicationConfigRequest {
            name: None,
            description: None,
            ports: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct UpdateRegistryRequest(Vec<PatchDocument>);

impl ::std::convert::From<Vec<PatchDocument>> for UpdateRegistryRequest {
    fn from(x: Vec<PatchDocument>) -> Self {
        UpdateRegistryRequest(x)
    }
}

impl ::std::convert::From<UpdateRegistryRequest> for Vec<PatchDocument> {
    fn from(x: UpdateRegistryRequest) -> Self {
        x.0
    }
}

impl ::std::iter::FromIterator<PatchDocument> for UpdateRegistryRequest {
    fn from_iter<U: IntoIterator<Item=PatchDocument>>(u: U) -> Self {
        UpdateRegistryRequest(Vec::<PatchDocument>::from_iter(u))
    }
}

impl ::std::iter::IntoIterator for UpdateRegistryRequest {
    type Item = PatchDocument;
    type IntoIter = ::std::vec::IntoIter<PatchDocument>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a UpdateRegistryRequest {
    type Item = &'a PatchDocument;
    type IntoIter = ::std::slice::Iter<'a, PatchDocument>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> ::std::iter::IntoIterator for &'a mut UpdateRegistryRequest {
    type Item = &'a mut PatchDocument;
    type IntoIter = ::std::slice::IterMut<'a, PatchDocument>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl ::std::ops::Deref for UpdateRegistryRequest {
    type Target = Vec<PatchDocument>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for UpdateRegistryRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct UpdateWorkflowGraph {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "version")]
    pub version: isize,

    #[serde(rename = "objects")]
    pub objects: SortedHashMap<String, models::WorkflowObject>,

    #[serde(rename = "edges")]
    pub edges: SortedHashMap<String, models::WorkflowEdge>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::WorkflowMetadata>,

}

impl UpdateWorkflowGraph {
    pub fn new(name: String, description: String, version: isize, objects: SortedHashMap<String, models::WorkflowObject>, edges: SortedHashMap<String, models::WorkflowEdge>, ) -> UpdateWorkflowGraph {
        UpdateWorkflowGraph {
            name: name,
            description: description,
            version: version,
            objects: objects,
            edges: edges,
            metadata: None,
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

    /// Last login time of user.
    #[serde(rename = "last_logged_in_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_logged_in_at: Option<i64>,

    /// Creation time of user.
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

    /// Whether this user has accepted latest terms and conditions or not
    #[serde(rename = "accepted_latest_terms_and_conditions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accepted_latest_terms_and_conditions: Option<bool>,

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
            accepted_latest_terms_and_conditions: None,
        }
    }
}


/// Status of an Account for a user.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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


/// Status of a user.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
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
pub struct VersionInFinalWorkflow {
    #[serde(rename = "graph_id")]
    pub graph_id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "contents")]
    pub contents: models::FinalWorkflowGraph,

}

impl VersionInFinalWorkflow {
    pub fn new(graph_id: uuid::Uuid, name: String, description: String, version: String, contents: models::FinalWorkflowGraph, ) -> VersionInFinalWorkflow {
        VersionInFinalWorkflow {
            graph_id: graph_id,
            name: name,
            description: description,
            version: version,
            contents: contents,
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
pub struct VersionedZoneId {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,

}

impl VersionedZoneId {
    pub fn new(id: uuid::Uuid, ) -> VersionedZoneId {
        VersionedZoneId {
            id: id,
            version: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowEdge {
    #[serde(rename = "source")]
    pub source: models::WorkflowEdgeLink,

    #[serde(rename = "target")]
    pub target: models::WorkflowEdgeLink,

}

impl WorkflowEdge {
    pub fn new(source: models::WorkflowEdgeLink, target: models::WorkflowEdgeLink, ) -> WorkflowEdge {
        WorkflowEdge {
            source: source,
            target: target,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowEdgeLink {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<String>,

}

impl WorkflowEdgeLink {
    pub fn new(id: String, ) -> WorkflowEdgeLink {
        WorkflowEdgeLink {
            id: id,
            port: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowGraph {
    #[serde(rename = "graph_id")]
    pub graph_id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "creator_id")]
    pub creator_id: uuid::Uuid,

    /// Dataset creation time.
    #[serde(rename = "created_at")]
    pub created_at: i64,

    /// Last update timestamp.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "version")]
    pub version: isize,

    #[serde(rename = "objects")]
    pub objects: SortedHashMap<String, models::WorkflowObject>,

    #[serde(rename = "edges")]
    pub edges: SortedHashMap<String, models::WorkflowEdge>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<models::WorkflowMetadata>,

}

impl WorkflowGraph {
    pub fn new(graph_id: uuid::Uuid, name: String, creator_id: uuid::Uuid, created_at: i64, updated_at: i64, description: String, version: isize, objects: SortedHashMap<String, models::WorkflowObject>, edges: SortedHashMap<String, models::WorkflowEdge>, ) -> WorkflowGraph {
        WorkflowGraph {
            graph_id: graph_id,
            name: name,
            creator_id: creator_id,
            created_at: created_at,
            updated_at: updated_at,
            description: description,
            version: version,
            objects: objects,
            edges: edges,
            metadata: None,
        }
    }
}


/// The final workflow from which this draft was derived. This field may point to a deleted final workflow in which you should treat it as if it's not present.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowLinkMetadata {
    #[serde(rename = "graph_id")]
    pub graph_id: uuid::Uuid,

    #[serde(rename = "source_version")]
    pub source_version: isize,

}

impl WorkflowLinkMetadata {
    pub fn new(graph_id: uuid::Uuid, source_version: isize, ) -> WorkflowLinkMetadata {
        WorkflowLinkMetadata {
            graph_id: graph_id,
            source_version: source_version,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowMetadata {
    #[serde(rename = "nodes")]
    pub nodes: HashMap<String, models::WorkflowNodeMetadata>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<models::WorkflowLinkMetadata>,

}

impl WorkflowMetadata {
    pub fn new(nodes: HashMap<String, models::WorkflowNodeMetadata>, ) -> WorkflowMetadata {
        WorkflowMetadata {
            nodes: nodes,
            parent: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowNodeMetadata {
    #[serde(rename = "position")]
    pub position: models::WorkflowNodePositionMetadata,

}

impl WorkflowNodeMetadata {
    pub fn new(position: models::WorkflowNodePositionMetadata, ) -> WorkflowNodeMetadata {
        WorkflowNodeMetadata {
            position: position,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowNodePositionMetadata {
    #[serde(rename = "x")]
    pub x: isize,

    #[serde(rename = "y")]
    pub y: isize,

}

impl WorkflowNodePositionMetadata {
    pub fn new(x: isize, y: isize, ) -> WorkflowNodePositionMetadata {
        WorkflowNodePositionMetadata {
            x: x,
            y: y,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowObject {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "ref")]
    pub _ref: models::WorkflowObjectRef,

}

impl WorkflowObject {
    pub fn new(name: String, user_id: uuid::Uuid, _ref: models::WorkflowObjectRef, ) -> WorkflowObject {
        WorkflowObject {
            name: name,
            user_id: user_id,
            description: None,
            _ref: _ref,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowObjectRef {
    #[serde(rename = "placeholder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placeholder: Option<models::WorkflowObjectRefPlaceholder>,

    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset: Option<models::WorkflowObjectRefDataset>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<models::WorkflowObjectRefApp>,

}

impl WorkflowObjectRef {
    pub fn new() -> WorkflowObjectRef {
        WorkflowObjectRef {
            placeholder: None,
            dataset: None,
            app: None,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowObjectRefApp {
    #[serde(rename = "image_id")]
    pub image_id: uuid::Uuid,

    #[serde(rename = "config_id")]
    pub config_id: String,

}

impl WorkflowObjectRefApp {
    pub fn new(image_id: uuid::Uuid, config_id: String, ) -> WorkflowObjectRefApp {
        WorkflowObjectRefApp {
            image_id: image_id,
            config_id: config_id,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowObjectRefDataset {
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,

}

impl WorkflowObjectRefDataset {
    pub fn new(dataset_id: uuid::Uuid, ) -> WorkflowObjectRefDataset {
        WorkflowObjectRefDataset {
            dataset_id: dataset_id,
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct WorkflowObjectRefPlaceholder {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "kind")]
    pub kind: String,

}

impl WorkflowObjectRefPlaceholder {
    pub fn new(kind: String, ) -> WorkflowObjectRefPlaceholder {
        WorkflowObjectRefPlaceholder {
            kind: kind,
        }
    }
}


/// Detailed info of a zone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct Zone {
    /// The account ID of the account that this zone belongs to.
    #[serde(rename = "acct_id")]
    pub acct_id: uuid::Uuid,

    /// Zone certificate (PEM format)
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

    /// Interval between node agent checkins with the backend, in seconds
    #[serde(rename = "node_refresh_interval")]
    pub node_refresh_interval: i64,

    /// The node agent requests certificate renewal when the certificate's remaining validity is less than this percentage of the original validity
    #[serde(rename = "node_renewal_threshold")]
    pub node_renewal_threshold: i32,

}

impl Zone {
    pub fn new(acct_id: uuid::Uuid, certificate: String, zone_id: uuid::Uuid, name: String, node_refresh_interval: i64, node_renewal_threshold: i32, ) -> Zone {
        Zone {
            acct_id: acct_id,
            certificate: certificate,
            zone_id: zone_id,
            name: name,
            description: None,
            node_refresh_interval: node_refresh_interval,
            node_renewal_threshold: node_renewal_threshold,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "conversion", derive(LabelledGeneric))]
pub struct ZoneJoinToken {
    /// Bearer token used to enroll compute nodes.
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

