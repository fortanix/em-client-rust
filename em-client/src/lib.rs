/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types, unused_qualifications)]

extern crate base64;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate hyper;
#[macro_use]
extern crate url;

extern crate mime;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate chrono;
extern crate uuid;
extern crate mbedtls;

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
pub const API_VERSION: &'static str = "2.0.0";

// https://en.wikipedia.org/wiki/SHA-2
const SHA256_BYTE_LENGTH: usize = 32;

const SHA256_CHAR_LENGTH: usize = SHA256_BYTE_LENGTH * 2;

// Need to restore enum generation for multi-response request types

/// Trait for decorating an implementation of the API with common
/// pre/post-handling for all request types.
///
/// The ApiDecorator and ApiDispatch traits are similar. Only `ApiDispatch`
/// (defined in `mod server`) has the `req` argument to `dispatch`. That
/// aside, attmpting to combine them into a single trait would result in
/// circular trait implementations `impl ApiDispatch for A: Api` and
/// `impl Api for T: ApiDecorator`.

pub trait ApiDecorator {
    type Error;

    fn dispatch<F, T>(&self, f: F) -> Result<T, Self::Error>
    where
        F: FnOnce(&dyn Api<Error = Self::Error>) -> Result<T, Self::Error>;
}

/// API trait with immutable receivers i.e. `fn api_call(&self, ...)`
pub trait Api {
    type Error;



    /// Create a new account.
    fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete an account.
    fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account.
    fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts.
    fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on.
    fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update an account.
    fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;



    /// Add an application.
    fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Delete a particular app
    fn delete_app(&self, app_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all apps information.
    fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app.
    fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate.
    fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an app's certificate for a compute node.
    fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the applications within selected account
    fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, Self::Error>;

    /// Update details of a particular app.
    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;



    /// Add an app config.
    fn create_application_config(&self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Delete a particular app config
    fn delete_application_config(&self, config_id: String) -> Result<(), Self::Error>;

    /// Get all app configs
    fn get_all_application_configs(&self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error>;

    /// Get details of a particular app config.
    fn get_application_config(&self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Get app config
    fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Get details of a particular runtime app config.
    fn get_specific_runtime_application_config(&self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Update details of a particular app config.
    fn update_application_config(&self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error>;



    /// Approve a request.
    fn approve_approval_request(&self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Create approval request.
    fn create_approval_request(&self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error>;

    /// Delete an approval request.
    fn delete_approval_request(&self, request_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Deny a request.
    fn deny_approval_request(&self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get all approval requests
    fn get_all_approval_requests(&self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error>;

    /// Get an approval request.
    fn get_approval_request(&self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get the result for an approved or failed request.
    fn get_approval_request_result(&self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error>;



    /// User authentication
    fn authenticate_user(&self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error>;



    /// Convert a docker image and create a new image.
    fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new image.
    fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular image.
    fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all images information.
    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular image.
    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of an image.
    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

    /// Update details of a particular image.
    fn update_build(&self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error>;



    /// Retrieve a certificate.
    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Request a new certificate for an Enclave application
    fn new_certificate(&self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error>;




    fn create_dataset(&self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error>;


    fn delete_dataset(&self, dataset_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all datasets
    fn get_all_datasets(&self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error>;


    fn get_dataset(&self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error>;


    fn update_dataset(&self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error>;



    /// Deactivate a particular compute node.
    fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all compute nodes information.
    fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular compute node.
    fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested compute node's certificate.
    fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get a compute node's certificate.
    fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the nodes within selected account
    fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, Self::Error>;

    /// Provision a new compute node.
    fn provision_node(&self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error>;

    /// Update details of a particular compute node.
    fn update_node(&self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error>;

    /// Called periodically by a compute node.
    fn update_node_status(&self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error>;



    /// Add a new registry to an account
    fn create_registry(&self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error>;

    /// Delete registry
    fn delete_registry(&self, registry_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get details of all registry in the account
    fn get_all_registries(&self) -> Result<Vec<models::Registry>, Self::Error>;

    /// Get details of a particular registry
    fn get_registry(&self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error>;

    /// Get details of the registry that will be used for the particular app images
    fn get_registry_for_app(&self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error>;

    /// Get details of the registry that will be used for the particular image
    fn get_registry_for_image(&self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error>;

    /// Update a particular registry details
    fn update_registry(&self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error>;



    /// Get Manager Version.
    fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error>;



    /// Get all the tasks.
    fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task.
    fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task.
    fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task.
    fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;



    /// Convert an application to run in EnclaveOS.
    fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;



    /// Current user accepts latest terms and conditions.
    fn accept_terms_and_conditions(&self) -> Result<(), Self::Error>;

    /// Change user password.
    fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address.
    fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user.
    fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Completely delete a user profile from system
    fn delete_user_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Removed user's association with an account.
    fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user.
    fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all user's information.
    fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user.
    fn get_logged_in_user(&self) -> Result<models::User, Self::Error>;

    /// Get details of a particular user.
    fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user.
    fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations.
    fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address.
    fn resend_confirm_email(&self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account.
    fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password.
    fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, and the role of a user. User with MANAGER access role can only update another user.
    fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user.
    fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;




    fn create_workflow_graph(&self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

    /// Delete a particular draft workflow
    fn delete_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<(), Self::Error>;


    fn get_all_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular draft workflow
    fn get_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error>;


    fn update_workflow_graph(&self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;




    fn create_final_workflow_graph(&self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error>;

    /// Delete a particular final workflow
    fn delete_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error>;


    fn get_all_final_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular final workflow version
    fn get_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error>;

    /// Get details of a particular final workflow
    fn get_full_final_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error>;

    /// Create a new version for a particular final workflow
    fn update_final_workflow_graph(&self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error>;



    /// Get zone details.
    fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get the authentication token.
    fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones.
    fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error>;


}

/// API trait with mutable receivers i.e. `fn api_call(&mut self, ...)`
pub trait ApiMut {
    type Error;



    /// Create a new account.
    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete an account.
    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account.
    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts.
    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on.
    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update an account.
    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;



    /// Add an application.
    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Delete a particular app
    fn delete_app(&mut self, app_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all apps information.
    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app.
    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate.
    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an app's certificate for a compute node.
    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the applications within selected account
    fn get_apps_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error>;

    /// Update details of a particular app.
    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;



    /// Add an app config.
    fn create_application_config(&mut self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Delete a particular app config
    fn delete_application_config(&mut self, config_id: String) -> Result<(), Self::Error>;

    /// Get all app configs
    fn get_all_application_configs(&mut self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error>;

    /// Get details of a particular app config.
    fn get_application_config(&mut self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Get app config
    fn get_runtime_application_config(&mut self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Get details of a particular runtime app config.
    fn get_specific_runtime_application_config(&mut self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Update details of a particular app config.
    fn update_application_config(&mut self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error>;



    /// Approve a request.
    fn approve_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Create approval request.
    fn create_approval_request(&mut self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error>;

    /// Delete an approval request.
    fn delete_approval_request(&mut self, request_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Deny a request.
    fn deny_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get all approval requests
    fn get_all_approval_requests(&mut self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error>;

    /// Get an approval request.
    fn get_approval_request(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get the result for an approved or failed request.
    fn get_approval_request_result(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error>;



    /// User authentication
    fn authenticate_user(&mut self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error>;



    /// Convert a docker image and create a new image.
    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new image.
    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular image.
    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all images information.
    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular image.
    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of an image.
    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

    /// Update details of a particular image.
    fn update_build(&mut self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error>;



    /// Retrieve a certificate.
    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Request a new certificate for an Enclave application
    fn new_certificate(&mut self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error>;




    fn create_dataset(&mut self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error>;


    fn delete_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all datasets
    fn get_all_datasets(&mut self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error>;


    fn get_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error>;


    fn update_dataset(&mut self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error>;



    /// Deactivate a particular compute node.
    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all compute nodes information.
    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular compute node.
    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested compute node's certificate.
    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get a compute node's certificate.
    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the nodes within selected account
    fn get_nodes_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error>;

    /// Provision a new compute node.
    fn provision_node(&mut self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error>;

    /// Update details of a particular compute node.
    fn update_node(&mut self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error>;

    /// Called periodically by a compute node.
    fn update_node_status(&mut self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error>;



    /// Add a new registry to an account
    fn create_registry(&mut self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error>;

    /// Delete registry
    fn delete_registry(&mut self, registry_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get details of all registry in the account
    fn get_all_registries(&mut self) -> Result<Vec<models::Registry>, Self::Error>;

    /// Get details of a particular registry
    fn get_registry(&mut self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error>;

    /// Get details of the registry that will be used for the particular app images
    fn get_registry_for_app(&mut self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error>;

    /// Get details of the registry that will be used for the particular image
    fn get_registry_for_image(&mut self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error>;

    /// Update a particular registry details
    fn update_registry(&mut self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error>;



    /// Get Manager Version.
    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error>;



    /// Get all the tasks.
    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task.
    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task.
    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task.
    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;



    /// Convert an application to run in EnclaveOS.
    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;



    /// Current user accepts latest terms and conditions.
    fn accept_terms_and_conditions(&mut self) -> Result<(), Self::Error>;

    /// Change user password.
    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address.
    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user.
    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Completely delete a user profile from system
    fn delete_user_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Removed user's association with an account.
    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user.
    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all user's information.
    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user.
    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error>;

    /// Get details of a particular user.
    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user.
    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations.
    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address.
    fn resend_confirm_email(&mut self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account.
    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password.
    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, and the role of a user. User with MANAGER access role can only update another user.
    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user.
    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;




    fn create_workflow_graph(&mut self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

    /// Delete a particular draft workflow
    fn delete_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<(), Self::Error>;


    fn get_all_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular draft workflow
    fn get_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error>;


    fn update_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;




    fn create_final_workflow_graph(&mut self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error>;

    /// Delete a particular final workflow
    fn delete_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error>;


    fn get_all_final_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular final workflow version
    fn get_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error>;

    /// Get details of a particular final workflow
    fn get_full_final_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error>;

    /// Create a new version for a particular final workflow
    fn update_final_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error>;



    /// Get zone details.
    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get the authentication token.
    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones.
    fn get_zones(&mut self) -> Result<Vec<models::Zone>, Self::Error>;


}

/*
 * We want to implement Api for types that implement each of the API category
 * traits individually. We do this indirectly via ApiDecorator and a wrapper
 * type ApiCombiner, because the compiler won't allow two blanket `for T`
 * implementations of the same trait (the trait bounds are not sufficent to
 * ensure that the two implementations can never overlap).
 */

pub struct ApiCombiner<'a, T>(&'a T);

impl<T, E> ApiDecorator for T
where
    T: AccountsApi<Error = E> + AppApi<Error = E> + ApplicationConfigApi<Error = E> + ApprovalRequestsApi<Error = E> + AuthApi<Error = E> + BuildApi<Error = E> + CertificateApi<Error = E> + DatasetApi<Error = E> + NodeApi<Error = E> + RegistryApi<Error = E> + SystemApi<Error = E> + TaskApi<Error = E> + ToolsApi<Error = E> + UsersApi<Error = E> + WorkflowApi<Error = E> + WorkflowFinalApi<Error = E> + ZoneApi<Error = E> + 
{
    type Error = E;

    fn dispatch<F, U>(&self, f: F) -> Result<U, Self::Error>
    where
        F: FnOnce(&dyn Api<Error = E>) -> Result<U, Self::Error>
    {
        f(&ApiCombiner(self))
    }
}

/// Implements with functions of the form `fn api_call(self, ...) { CategoryApi::api_call(self.0, ...) }`
impl<'a, T, E> Api for ApiCombiner<'a, T>
where
    T: AccountsApi<Error = E> + AppApi<Error = E> + ApplicationConfigApi<Error = E> + ApprovalRequestsApi<Error = E> + AuthApi<Error = E> + BuildApi<Error = E> + CertificateApi<Error = E> + DatasetApi<Error = E> + NodeApi<Error = E> + RegistryApi<Error = E> + SystemApi<Error = E> + TaskApi<Error = E> + ToolsApi<Error = E> + UsersApi<Error = E> + WorkflowApi<Error = E> + WorkflowFinalApi<Error = E> + ZoneApi<Error = E> +  'a
{
    type Error = E;


    
        fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
            AccountsApi::create_account(self.0, body, )
        }
    
        fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
            AccountsApi::delete_account(self.0, account_id, )
        }
    
        fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
            AccountsApi::get_account(self.0, account_id, )
        }
    
        fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error> {
            AccountsApi::get_accounts(self.0, )
        }
    
        fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
            AccountsApi::select_account(self.0, account_id, )
        }
    
        fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
            AccountsApi::update_account(self.0, account_id, body, )
        }
    

    
        fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error> {
            AppApi::add_application(self.0, body, )
        }
    
        fn delete_app(&self, app_id: uuid::Uuid) -> Result<(), Self::Error> {
            AppApi::delete_app(self.0, app_id, )
        }
    
        fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
            AppApi::get_all_apps(self.0, name, description, all_search, limit, offset, sort_by, )
        }
    
        fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
            AppApi::get_app(self.0, app_id, )
        }
    
        fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            AppApi::get_app_certificate(self.0, node_id, app_id, )
        }
    
        fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
            AppApi::get_app_node_certificate_details(self.0, node_id, app_id, )
        }
    
        fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
            AppApi::get_apps_unique_labels(self.0, )
        }
    
        fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
            AppApi::update_app(self.0, app_id, body, )
        }
    

    
        fn create_application_config(&self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error> {
            ApplicationConfigApi::create_application_config(self.0, body, )
        }
    
        fn delete_application_config(&self, config_id: String) -> Result<(), Self::Error> {
            ApplicationConfigApi::delete_application_config(self.0, config_id, )
        }
    
        fn get_all_application_configs(&self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error> {
            ApplicationConfigApi::get_all_application_configs(self.0, name, description, image_id, limit, offset, )
        }
    
        fn get_application_config(&self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error> {
            ApplicationConfigApi::get_application_config(self.0, config_id, )
        }
    
        fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error> {
            ApplicationConfigApi::get_runtime_application_config(self.0, expected_hash)
        }
    
        fn get_specific_runtime_application_config(&self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error> {
            ApplicationConfigApi::get_specific_runtime_application_config(self.0, config_id, )
        }
    
        fn update_application_config(&self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error> {
            ApplicationConfigApi::update_application_config(self.0, config_id, body, )
        }
    

    
        fn approve_approval_request(&self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error> {
            ApprovalRequestsApi::approve_approval_request(self.0, request_id, body, )
        }
    
        fn create_approval_request(&self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error> {
            ApprovalRequestsApi::create_approval_request(self.0, body, )
        }
    
        fn delete_approval_request(&self, request_id: uuid::Uuid) -> Result<(), Self::Error> {
            ApprovalRequestsApi::delete_approval_request(self.0, request_id, )
        }
    
        fn deny_approval_request(&self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error> {
            ApprovalRequestsApi::deny_approval_request(self.0, request_id, body, )
        }
    
        fn get_all_approval_requests(&self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error> {
            ApprovalRequestsApi::get_all_approval_requests(self.0, requester, reviewer, subject, status, all_search, sort_by, limit, offset, )
        }
    
        fn get_approval_request(&self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error> {
            ApprovalRequestsApi::get_approval_request(self.0, request_id, )
        }
    
        fn get_approval_request_result(&self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error> {
            ApprovalRequestsApi::get_approval_request_result(self.0, request_id, )
        }
    

    
        fn authenticate_user(&self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error> {
            AuthApi::authenticate_user(self.0, body, )
        }
    

    
        fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
            BuildApi::convert_app_build(self.0, body, )
        }
    
        fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
            BuildApi::create_build(self.0, body, )
        }
    
        fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
            BuildApi::delete_build(self.0, build_id, )
        }
    
        fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
            BuildApi::get_all_builds(self.0, all_search, docker_image_name, config_id, deployed_status, status, limit, offset, sort_by, )
        }
    
        fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
            BuildApi::get_build(self.0, build_id, )
        }
    
        fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
            BuildApi::get_build_deployments(self.0, build_id, status, all_search, sort_by, limit, offset, )
        }
    
        fn update_build(&self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error> {
            BuildApi::update_build(self.0, build_id, body, )
        }
    

    
        fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            CertificateApi::get_certificate(self.0, cert_id, )
        }
    
        fn new_certificate(&self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error> {
            CertificateApi::new_certificate(self.0, body, )
        }
    

    
        fn create_dataset(&self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error> {
            DatasetApi::create_dataset(self.0, body, )
        }
    
        fn delete_dataset(&self, dataset_id: uuid::Uuid) -> Result<(), Self::Error> {
            DatasetApi::delete_dataset(self.0, dataset_id, )
        }
    
        fn get_all_datasets(&self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error> {
            DatasetApi::get_all_datasets(self.0, name, description, limit, offset, )
        }
    
        fn get_dataset(&self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error> {
            DatasetApi::get_dataset(self.0, dataset_id, )
        }
    
        fn update_dataset(&self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error> {
            DatasetApi::update_dataset(self.0, dataset_id, body, )
        }
    

    
        fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
            NodeApi::deactivate_node(self.0, node_id, )
        }
    
        fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
            NodeApi::get_all_nodes(self.0, name, description, sgx_version, all_search, status, limit, offset, sort_by, )
        }
    
        fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
            NodeApi::get_node(self.0, node_id, )
        }
    
        fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            NodeApi::get_node_certificate(self.0, node_id, )
        }
    
        fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
            NodeApi::get_node_certificate_details(self.0, node_id, )
        }
    
        fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
            NodeApi::get_nodes_unique_labels(self.0, )
        }
    
        fn provision_node(&self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error> {
            NodeApi::provision_node(self.0, body, )
        }
    
        fn update_node(&self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error> {
            NodeApi::update_node(self.0, node_id, body, )
        }
    
        fn update_node_status(&self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error> {
            NodeApi::update_node_status(self.0, body, )
        }
    

    
        fn create_registry(&self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error> {
            RegistryApi::create_registry(self.0, registry_request, )
        }
    
        fn delete_registry(&self, registry_id: uuid::Uuid) -> Result<(), Self::Error> {
            RegistryApi::delete_registry(self.0, registry_id, )
        }
    
        fn get_all_registries(&self) -> Result<Vec<models::Registry>, Self::Error> {
            RegistryApi::get_all_registries(self.0, )
        }
    
        fn get_registry(&self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error> {
            RegistryApi::get_registry(self.0, registry_id, )
        }
    
        fn get_registry_for_app(&self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error> {
            RegistryApi::get_registry_for_app(self.0, app_id, )
        }
    
        fn get_registry_for_image(&self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error> {
            RegistryApi::get_registry_for_image(self.0, image_name, )
        }
    
        fn update_registry(&self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error> {
            RegistryApi::update_registry(self.0, registry_id, body, )
        }
    

    
        fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error> {
            SystemApi::get_manager_version(self.0, )
        }
    

    
        fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
            TaskApi::get_all_tasks(self.0, task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, )
        }
    
        fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
            TaskApi::get_task(self.0, task_id, )
        }
    
        fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
            TaskApi::get_task_status(self.0, task_id, )
        }
    
        fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
            TaskApi::update_task(self.0, task_id, body, )
        }
    

    
        fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
            ToolsApi::convert_app(self.0, body, )
        }
    

    
        fn accept_terms_and_conditions(&self) -> Result<(), Self::Error> {
            UsersApi::accept_terms_and_conditions(self.0, )
        }
    
        fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
            UsersApi::change_password(self.0, body, )
        }
    
        fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
            UsersApi::confirm_email(self.0, body, )
        }
    
        fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
            UsersApi::create_user(self.0, body, )
        }
    
        fn delete_user_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            UsersApi::delete_user_account(self.0, user_id, )
        }
    
        fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            UsersApi::delete_user_from_account(self.0, user_id, )
        }
    
        fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
            UsersApi::forgot_password(self.0, body, )
        }
    
        fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
            UsersApi::get_all_users(self.0, all_search, limit, offset, sort_by, )
        }
    
        fn get_logged_in_user(&self) -> Result<models::User, Self::Error> {
            UsersApi::get_logged_in_user(self.0, )
        }
    
        fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
            UsersApi::get_user(self.0, user_id, )
        }
    
        fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
            UsersApi::invite_user(self.0, body, )
        }
    
        fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
            UsersApi::process_invitations(self.0, body, )
        }
    
        fn resend_confirm_email(&self) -> Result<(), Self::Error> {
            UsersApi::resend_confirm_email(self.0, )
        }
    
        fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            UsersApi::resend_invitation(self.0, user_id, )
        }
    
        fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
            UsersApi::reset_password(self.0, user_id, body, )
        }
    
        fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
            UsersApi::update_user(self.0, user_id, body, )
        }
    
        fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
            UsersApi::validate_password_reset_token(self.0, user_id, body, )
        }
    

    
        fn create_workflow_graph(&self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
            WorkflowApi::create_workflow_graph(self.0, body, )
        }
    
        fn delete_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<(), Self::Error> {
            WorkflowApi::delete_workflow_graph(self.0, graph_id, )
        }
    
        fn get_all_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error> {
            WorkflowApi::get_all_workflow_graphs(self.0, name, description, all_search, parent_graph_id, sort_by, limit, offset, )
        }
    
        fn get_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error> {
            WorkflowApi::get_workflow_graph(self.0, graph_id, )
        }
    
        fn update_workflow_graph(&self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
            WorkflowApi::update_workflow_graph(self.0, graph_id, body, )
        }
    

    
        fn create_final_workflow_graph(&self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error> {
            WorkflowFinalApi::create_final_workflow_graph(self.0, body, )
        }
    
        fn delete_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error> {
            WorkflowFinalApi::delete_final_workflow_graph(self.0, graph_id, version, )
        }
    
        fn get_all_final_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error> {
            WorkflowFinalApi::get_all_final_workflow_graphs(self.0, name, description, all_search, sort_by, limit, offset, )
        }
    
        fn get_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error> {
            WorkflowFinalApi::get_final_workflow_graph(self.0, graph_id, version, )
        }
    
        fn get_full_final_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error> {
            WorkflowFinalApi::get_full_final_workflow_graph(self.0, graph_id, )
        }
    
        fn update_final_workflow_graph(&self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error> {
            WorkflowFinalApi::update_final_workflow_graph(self.0, graph_id, body, )
        }
    

    
        fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
            ZoneApi::get_zone(self.0, zone_id, )
        }
    
        fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
            ZoneApi::get_zone_join_token(self.0, zone_id, )
        }
    
        fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error> {
            ZoneApi::get_zones(self.0, )
        }
    

}

/// Implements with functions of the form `fn api_call(&self, ...) { self.dispatch(|a| Api::api_call(a, ...)) }`
impl<T, E> Api for T
where
    T: ApiDecorator<Error = E>
{
    type Error = E;


    
        fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
            self.dispatch(|a| Api::create_account(a, body, ))
        }
    
        fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_account(a, account_id, ))
        }
    
        fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
            self.dispatch(|a| Api::get_account(a, account_id, ))
        }
    
        fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error> {
            self.dispatch(|a| Api::get_accounts(a, ))
        }
    
        fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::select_account(a, account_id, ))
        }
    
        fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
            self.dispatch(|a| Api::update_account(a, account_id, body, ))
        }
    

    
        fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error> {
            self.dispatch(|a| Api::add_application(a, body, ))
        }
    
        fn delete_app(&self, app_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_app(a, app_id, ))
        }
    
        fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_apps(a, name, description, all_search, limit, offset, sort_by, ))
        }
    
        fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
            self.dispatch(|a| Api::get_app(a, app_id, ))
        }
    
        fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            self.dispatch(|a| Api::get_app_certificate(a, node_id, app_id, ))
        }
    
        fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
            self.dispatch(|a| Api::get_app_node_certificate_details(a, node_id, app_id, ))
        }
    
        fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
            self.dispatch(|a| Api::get_apps_unique_labels(a, ))
        }
    
        fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
            self.dispatch(|a| Api::update_app(a, app_id, body, ))
        }
    

    
        fn create_application_config(&self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error> {
            self.dispatch(|a| Api::create_application_config(a, body, ))
        }
    
        fn delete_application_config(&self, config_id: String) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_application_config(a, config_id, ))
        }
    
        fn get_all_application_configs(&self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_application_configs(a, name, description, image_id, limit, offset, ))
        }
    
        fn get_application_config(&self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error> {
            self.dispatch(|a| Api::get_application_config(a, config_id, ))
        }
    
        fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error> {
            self.dispatch(|a| Api::get_runtime_application_config(a, expected_hash))
        }
    
        fn get_specific_runtime_application_config(&self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error> {
            self.dispatch(|a| Api::get_specific_runtime_application_config(a, config_id, ))
        }
    
        fn update_application_config(&self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error> {
            self.dispatch(|a| Api::update_application_config(a, config_id, body, ))
        }
    

    
        fn approve_approval_request(&self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error> {
            self.dispatch(|a| Api::approve_approval_request(a, request_id, body, ))
        }
    
        fn create_approval_request(&self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error> {
            self.dispatch(|a| Api::create_approval_request(a, body, ))
        }
    
        fn delete_approval_request(&self, request_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_approval_request(a, request_id, ))
        }
    
        fn deny_approval_request(&self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error> {
            self.dispatch(|a| Api::deny_approval_request(a, request_id, body, ))
        }
    
        fn get_all_approval_requests(&self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error> {
            self.dispatch(|a| Api::get_all_approval_requests(a, requester, reviewer, subject, status, all_search, sort_by, limit, offset, ))
        }
    
        fn get_approval_request(&self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error> {
            self.dispatch(|a| Api::get_approval_request(a, request_id, ))
        }
    
        fn get_approval_request_result(&self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error> {
            self.dispatch(|a| Api::get_approval_request_result(a, request_id, ))
        }
    

    
        fn authenticate_user(&self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error> {
            self.dispatch(|a| Api::authenticate_user(a, body, ))
        }
    

    
        fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
            self.dispatch(|a| Api::convert_app_build(a, body, ))
        }
    
        fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
            self.dispatch(|a| Api::create_build(a, body, ))
        }
    
        fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_build(a, build_id, ))
        }
    
        fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_builds(a, all_search, docker_image_name, config_id, deployed_status, status, limit, offset, sort_by, ))
        }
    
        fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
            self.dispatch(|a| Api::get_build(a, build_id, ))
        }
    
        fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
            self.dispatch(|a| Api::get_build_deployments(a, build_id, status, all_search, sort_by, limit, offset, ))
        }
    
        fn update_build(&self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error> {
            self.dispatch(|a| Api::update_build(a, build_id, body, ))
        }
    

    
        fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            self.dispatch(|a| Api::get_certificate(a, cert_id, ))
        }
    
        fn new_certificate(&self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error> {
            self.dispatch(|a| Api::new_certificate(a, body, ))
        }
    

    
        fn create_dataset(&self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error> {
            self.dispatch(|a| Api::create_dataset(a, body, ))
        }
    
        fn delete_dataset(&self, dataset_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_dataset(a, dataset_id, ))
        }
    
        fn get_all_datasets(&self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_datasets(a, name, description, limit, offset, ))
        }
    
        fn get_dataset(&self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error> {
            self.dispatch(|a| Api::get_dataset(a, dataset_id, ))
        }
    
        fn update_dataset(&self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error> {
            self.dispatch(|a| Api::update_dataset(a, dataset_id, body, ))
        }
    

    
        fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::deactivate_node(a, node_id, ))
        }
    
        fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_nodes(a, name, description, sgx_version, all_search, status, limit, offset, sort_by, ))
        }
    
        fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
            self.dispatch(|a| Api::get_node(a, node_id, ))
        }
    
        fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
            self.dispatch(|a| Api::get_node_certificate(a, node_id, ))
        }
    
        fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
            self.dispatch(|a| Api::get_node_certificate_details(a, node_id, ))
        }
    
        fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
            self.dispatch(|a| Api::get_nodes_unique_labels(a, ))
        }
    
        fn provision_node(&self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error> {
            self.dispatch(|a| Api::provision_node(a, body, ))
        }
    
        fn update_node(&self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error> {
            self.dispatch(|a| Api::update_node(a, node_id, body, ))
        }
    
        fn update_node_status(&self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error> {
            self.dispatch(|a| Api::update_node_status(a, body, ))
        }
    

    
        fn create_registry(&self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error> {
            self.dispatch(|a| Api::create_registry(a, registry_request, ))
        }
    
        fn delete_registry(&self, registry_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_registry(a, registry_id, ))
        }
    
        fn get_all_registries(&self) -> Result<Vec<models::Registry>, Self::Error> {
            self.dispatch(|a| Api::get_all_registries(a, ))
        }
    
        fn get_registry(&self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error> {
            self.dispatch(|a| Api::get_registry(a, registry_id, ))
        }
    
        fn get_registry_for_app(&self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error> {
            self.dispatch(|a| Api::get_registry_for_app(a, app_id, ))
        }
    
        fn get_registry_for_image(&self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error> {
            self.dispatch(|a| Api::get_registry_for_image(a, image_name, ))
        }
    
        fn update_registry(&self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error> {
            self.dispatch(|a| Api::update_registry(a, registry_id, body, ))
        }
    

    
        fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error> {
            self.dispatch(|a| Api::get_manager_version(a, ))
        }
    

    
        fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_tasks(a, task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, ))
        }
    
        fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
            self.dispatch(|a| Api::get_task(a, task_id, ))
        }
    
        fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
            self.dispatch(|a| Api::get_task_status(a, task_id, ))
        }
    
        fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
            self.dispatch(|a| Api::update_task(a, task_id, body, ))
        }
    

    
        fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
            self.dispatch(|a| Api::convert_app(a, body, ))
        }
    

    
        fn accept_terms_and_conditions(&self) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::accept_terms_and_conditions(a, ))
        }
    
        fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::change_password(a, body, ))
        }
    
        fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
            self.dispatch(|a| Api::confirm_email(a, body, ))
        }
    
        fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
            self.dispatch(|a| Api::create_user(a, body, ))
        }
    
        fn delete_user_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_user_account(a, user_id, ))
        }
    
        fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_user_from_account(a, user_id, ))
        }
    
        fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::forgot_password(a, body, ))
        }
    
        fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_users(a, all_search, limit, offset, sort_by, ))
        }
    
        fn get_logged_in_user(&self) -> Result<models::User, Self::Error> {
            self.dispatch(|a| Api::get_logged_in_user(a, ))
        }
    
        fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
            self.dispatch(|a| Api::get_user(a, user_id, ))
        }
    
        fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
            self.dispatch(|a| Api::invite_user(a, body, ))
        }
    
        fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::process_invitations(a, body, ))
        }
    
        fn resend_confirm_email(&self) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::resend_confirm_email(a, ))
        }
    
        fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::resend_invitation(a, user_id, ))
        }
    
        fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::reset_password(a, user_id, body, ))
        }
    
        fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
            self.dispatch(|a| Api::update_user(a, user_id, body, ))
        }
    
        fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
            self.dispatch(|a| Api::validate_password_reset_token(a, user_id, body, ))
        }
    

    
        fn create_workflow_graph(&self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
            self.dispatch(|a| Api::create_workflow_graph(a, body, ))
        }
    
        fn delete_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_workflow_graph(a, graph_id, ))
        }
    
        fn get_all_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_workflow_graphs(a, name, description, all_search, parent_graph_id, sort_by, limit, offset, ))
        }
    
        fn get_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error> {
            self.dispatch(|a| Api::get_workflow_graph(a, graph_id, ))
        }
    
        fn update_workflow_graph(&self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
            self.dispatch(|a| Api::update_workflow_graph(a, graph_id, body, ))
        }
    

    
        fn create_final_workflow_graph(&self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error> {
            self.dispatch(|a| Api::create_final_workflow_graph(a, body, ))
        }
    
        fn delete_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error> {
            self.dispatch(|a| Api::delete_final_workflow_graph(a, graph_id, version, ))
        }
    
        fn get_all_final_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error> {
            self.dispatch(|a| Api::get_all_final_workflow_graphs(a, name, description, all_search, sort_by, limit, offset, ))
        }
    
        fn get_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error> {
            self.dispatch(|a| Api::get_final_workflow_graph(a, graph_id, version, ))
        }
    
        fn get_full_final_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error> {
            self.dispatch(|a| Api::get_full_final_workflow_graph(a, graph_id, ))
        }
    
        fn update_final_workflow_graph(&self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error> {
            self.dispatch(|a| Api::update_final_workflow_graph(a, graph_id, body, ))
        }
    

    
        fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
            self.dispatch(|a| Api::get_zone(a, zone_id, ))
        }
    
        fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
            self.dispatch(|a| Api::get_zone_join_token(a, zone_id, ))
        }
    
        fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error> {
            self.dispatch(|a| Api::get_zones(a, ))
        }
    

}

/// Implements with functions of the form `fn api_call(&mut self, ...) { self.api_call(...) }`
impl<T, E> ApiMut for T
where
    T: AccountsApiMut<Error = E> + AppApiMut<Error = E> + ApplicationConfigApiMut<Error = E> + ApprovalRequestsApiMut<Error = E> + AuthApiMut<Error = E> + BuildApiMut<Error = E> + CertificateApiMut<Error = E> + DatasetApiMut<Error = E> + NodeApiMut<Error = E> + RegistryApiMut<Error = E> + SystemApiMut<Error = E> + TaskApiMut<Error = E> + ToolsApiMut<Error = E> + UsersApiMut<Error = E> + WorkflowApiMut<Error = E> + WorkflowFinalApiMut<Error = E> + ZoneApiMut<Error = E> + 
{
    type Error = E;



    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
        self.create_account(body, )
    }

    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_account(account_id, )
    }

    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
        self.get_account(account_id, )
    }

    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error> {
        self.get_accounts()
    }

    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.select_account(account_id, )
    }

    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
        self.update_account(account_id, body, )
    }



    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error> {
        self.add_application(body, )
    }

    fn delete_app(&mut self, app_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_app(app_id, )
    }

    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
        self.get_all_apps(name, description, all_search, limit, offset, sort_by, )
    }

    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
        self.get_app(app_id, )
    }

    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_app_certificate(node_id, app_id, )
    }

    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.get_app_node_certificate_details(node_id, app_id, )
    }

    fn get_apps_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error> {
        self.get_apps_unique_labels()
    }

    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        self.update_app(app_id, body, )
    }



    fn create_application_config(&mut self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.create_application_config(body, )
    }

    fn delete_application_config(&mut self, config_id: String) -> Result<(), Self::Error> {
        self.delete_application_config(config_id, )
    }

    fn get_all_application_configs(&mut self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error> {
        self.get_all_application_configs(name, description, image_id, limit, offset, )
    }

    fn get_application_config(&mut self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.get_application_config(config_id, )
    }

    fn get_runtime_application_config(&mut self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error> {
        self.get_runtime_application_config(expected_hash)
    }

    fn get_specific_runtime_application_config(&mut self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error> {
        self.get_specific_runtime_application_config(config_id, )
    }

    fn update_application_config(&mut self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.update_application_config(config_id, body, )
    }



    fn approve_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        self.approve_approval_request(request_id, body, )
    }

    fn create_approval_request(&mut self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error> {
        self.create_approval_request(body, )
    }

    fn delete_approval_request(&mut self, request_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_approval_request(request_id, )
    }

    fn deny_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        self.deny_approval_request(request_id, body, )
    }

    fn get_all_approval_requests(&mut self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error> {
        self.get_all_approval_requests(requester, reviewer, subject, status, all_search, sort_by, limit, offset, )
    }

    fn get_approval_request(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error> {
        self.get_approval_request(request_id, )
    }

    fn get_approval_request_result(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error> {
        self.get_approval_request_result(request_id, )
    }



    fn authenticate_user(&mut self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error> {
        self.authenticate_user(body, )
    }



    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
        self.convert_app_build(body, )
    }

    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
        self.create_build(body, )
    }

    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_build(build_id, )
    }

    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        self.get_all_builds(all_search, docker_image_name, config_id, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        self.get_build(build_id, )
    }

    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        self.get_build_deployments(build_id, status, all_search, sort_by, limit, offset, )
    }

    fn update_build(&mut self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error> {
        self.update_build(build_id, body, )
    }



    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_certificate(cert_id, )
    }

    fn new_certificate(&mut self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error> {
        self.new_certificate(body, )
    }



    fn create_dataset(&mut self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error> {
        self.create_dataset(body, )
    }

    fn delete_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_dataset(dataset_id, )
    }

    fn get_all_datasets(&mut self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error> {
        self.get_all_datasets(name, description, limit, offset, )
    }

    fn get_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error> {
        self.get_dataset(dataset_id, )
    }

    fn update_dataset(&mut self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error> {
        self.update_dataset(dataset_id, body, )
    }



    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.deactivate_node(node_id, )
    }

    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
        self.get_all_nodes(name, description, sgx_version, all_search, status, limit, offset, sort_by, )
    }

    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
        self.get_node(node_id, )
    }

    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_node_certificate(node_id, )
    }

    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.get_node_certificate_details(node_id, )
    }

    fn get_nodes_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error> {
        self.get_nodes_unique_labels()
    }

    fn provision_node(&mut self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error> {
        self.provision_node(body, )
    }

    fn update_node(&mut self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error> {
        self.update_node(node_id, body, )
    }

    fn update_node_status(&mut self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error> {
        self.update_node_status(body, )
    }



    fn create_registry(&mut self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error> {
        self.create_registry(registry_request, )
    }

    fn delete_registry(&mut self, registry_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_registry(registry_id, )
    }

    fn get_all_registries(&mut self) -> Result<Vec<models::Registry>, Self::Error> {
        self.get_all_registries()
    }

    fn get_registry(&mut self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error> {
        self.get_registry(registry_id, )
    }

    fn get_registry_for_app(&mut self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error> {
        self.get_registry_for_app(app_id, )
    }

    fn get_registry_for_image(&mut self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error> {
        self.get_registry_for_image(image_name, )
    }

    fn update_registry(&mut self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error> {
        self.update_registry(registry_id, body, )
    }



    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        self.get_manager_version()
    }



    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
        self.get_all_tasks(task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, )
    }

    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
        self.get_task(task_id, )
    }

    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
        self.get_task_status(task_id, )
    }

    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
        self.update_task(task_id, body, )
    }



    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
        self.convert_app(body, )
    }



    fn accept_terms_and_conditions(&mut self) -> Result<(), Self::Error> {
        self.accept_terms_and_conditions()
    }

    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
        self.change_password(body, )
    }

    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
        self.confirm_email(body, )
    }

    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
        self.create_user(body, )
    }

    fn delete_user_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_user_account(user_id, )
    }

    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_user_from_account(user_id, )
    }

    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
        self.forgot_password(body, )
    }

    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
        self.get_all_users(all_search, limit, offset, sort_by, )
    }

    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error> {
        self.get_logged_in_user()
    }

    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
        self.get_user(user_id, )
    }

    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
        self.invite_user(body, )
    }

    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
        self.process_invitations(body, )
    }

    fn resend_confirm_email(&mut self) -> Result<(), Self::Error> {
        self.resend_confirm_email()
    }

    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.resend_invitation(user_id, )
    }

    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
        self.reset_password(user_id, body, )
    }

    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
        self.update_user(user_id, body, )
    }

    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
        self.validate_password_reset_token(user_id, body, )
    }



    fn create_workflow_graph(&mut self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        self.create_workflow_graph(body, )
    }

    fn delete_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_workflow_graph(graph_id, )
    }

    fn get_all_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error> {
        self.get_all_workflow_graphs(name, description, all_search, parent_graph_id, sort_by, limit, offset, )
    }

    fn get_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error> {
        self.get_workflow_graph(graph_id, )
    }

    fn update_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        self.update_workflow_graph(graph_id, body, )
    }



    fn create_final_workflow_graph(&mut self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error> {
        self.create_final_workflow_graph(body, )
    }

    fn delete_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error> {
        self.delete_final_workflow_graph(graph_id, version, )
    }

    fn get_all_final_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error> {
        self.get_all_final_workflow_graphs(name, description, all_search, sort_by, limit, offset, )
    }

    fn get_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        self.get_final_workflow_graph(graph_id, version, )
    }

    fn get_full_final_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error> {
        self.get_full_final_workflow_graph(graph_id, )
    }

    fn update_final_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        self.update_final_workflow_graph(graph_id, body, )
    }



    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
        self.get_zone(zone_id, )
    }

    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
        self.get_zone_join_token(zone_id, )
    }

    fn get_zones(&mut self) -> Result<Vec<models::Zone>, Self::Error> {
        self.get_zones()
    }


}

impl<T, E> Api for std::cell::RefCell<T>
where
    T: ApiMut<Error = E>,
{
    type Error = E;


    fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
        self.borrow_mut().create_account(body, )
    }

    fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_account(account_id, )
    }

    fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
        self.borrow_mut().get_account(account_id, )
    }

    fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error> {
        self.borrow_mut().get_accounts()
    }

    fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().select_account(account_id, )
    }

    fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
        self.borrow_mut().update_account(account_id, body, )
    }



    fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error> {
        self.borrow_mut().add_application(body, )
    }

    fn delete_app(&self, app_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_app(app_id, )
    }

    fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
        self.borrow_mut().get_all_apps(name, description, all_search, limit, offset, sort_by, )
    }

    fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
        self.borrow_mut().get_app(app_id, )
    }

    fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.borrow_mut().get_app_certificate(node_id, app_id, )
    }

    fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.borrow_mut().get_app_node_certificate_details(node_id, app_id, )
    }

    fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
        self.borrow_mut().get_apps_unique_labels()
    }

    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        self.borrow_mut().update_app(app_id, body, )
    }



    fn create_application_config(&self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.borrow_mut().create_application_config(body, )
    }

    fn delete_application_config(&self, config_id: String) -> Result<(), Self::Error> {
        self.borrow_mut().delete_application_config(config_id, )
    }

    fn get_all_application_configs(&self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error> {
        self.borrow_mut().get_all_application_configs(name, description, image_id, limit, offset, )
    }

    fn get_application_config(&self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.borrow_mut().get_application_config(config_id, )
    }

    fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error> {
        self.borrow_mut().get_runtime_application_config(expected_hash)
    }

    fn get_specific_runtime_application_config(&self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error> {
        self.borrow_mut().get_specific_runtime_application_config(config_id, )
    }

    fn update_application_config(&self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error> {
        self.borrow_mut().update_application_config(config_id, body, )
    }



    fn approve_approval_request(&self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        self.borrow_mut().approve_approval_request(request_id, body, )
    }

    fn create_approval_request(&self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error> {
        self.borrow_mut().create_approval_request(body, )
    }

    fn delete_approval_request(&self, request_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_approval_request(request_id, )
    }

    fn deny_approval_request(&self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        self.borrow_mut().deny_approval_request(request_id, body, )
    }

    fn get_all_approval_requests(&self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error> {
        self.borrow_mut().get_all_approval_requests(requester, reviewer, subject, status, all_search, sort_by, limit, offset, )
    }

    fn get_approval_request(&self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error> {
        self.borrow_mut().get_approval_request(request_id, )
    }

    fn get_approval_request_result(&self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error> {
        self.borrow_mut().get_approval_request_result(request_id, )
    }



    fn authenticate_user(&self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error> {
        self.borrow_mut().authenticate_user(body, )
    }



    fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
        self.borrow_mut().convert_app_build(body, )
    }

    fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
        self.borrow_mut().create_build(body, )
    }

    fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_build(build_id, )
    }

    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        self.borrow_mut().get_all_builds(all_search, docker_image_name, config_id, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        self.borrow_mut().get_build(build_id, )
    }

    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        self.borrow_mut().get_build_deployments(build_id, status, all_search, sort_by, limit, offset, )
    }

    fn update_build(&self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error> {
        self.borrow_mut().update_build(build_id, body, )
    }



    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.borrow_mut().get_certificate(cert_id, )
    }

    fn new_certificate(&self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error> {
        self.borrow_mut().new_certificate(body, )
    }



    fn create_dataset(&self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error> {
        self.borrow_mut().create_dataset(body, )
    }

    fn delete_dataset(&self, dataset_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_dataset(dataset_id, )
    }

    fn get_all_datasets(&self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error> {
        self.borrow_mut().get_all_datasets(name, description, limit, offset, )
    }

    fn get_dataset(&self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error> {
        self.borrow_mut().get_dataset(dataset_id, )
    }

    fn update_dataset(&self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error> {
        self.borrow_mut().update_dataset(dataset_id, body, )
    }



    fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().deactivate_node(node_id, )
    }

    fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
        self.borrow_mut().get_all_nodes(name, description, sgx_version, all_search, status, limit, offset, sort_by, )
    }

    fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
        self.borrow_mut().get_node(node_id, )
    }

    fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.borrow_mut().get_node_certificate(node_id, )
    }

    fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.borrow_mut().get_node_certificate_details(node_id, )
    }

    fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, Self::Error> {
        self.borrow_mut().get_nodes_unique_labels()
    }

    fn provision_node(&self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error> {
        self.borrow_mut().provision_node(body, )
    }

    fn update_node(&self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error> {
        self.borrow_mut().update_node(node_id, body, )
    }

    fn update_node_status(&self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error> {
        self.borrow_mut().update_node_status(body, )
    }



    fn create_registry(&self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error> {
        self.borrow_mut().create_registry(registry_request, )
    }

    fn delete_registry(&self, registry_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_registry(registry_id, )
    }

    fn get_all_registries(&self) -> Result<Vec<models::Registry>, Self::Error> {
        self.borrow_mut().get_all_registries()
    }

    fn get_registry(&self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error> {
        self.borrow_mut().get_registry(registry_id, )
    }

    fn get_registry_for_app(&self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error> {
        self.borrow_mut().get_registry_for_app(app_id, )
    }

    fn get_registry_for_image(&self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error> {
        self.borrow_mut().get_registry_for_image(image_name, )
    }

    fn update_registry(&self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error> {
        self.borrow_mut().update_registry(registry_id, body, )
    }



    fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error> {
        self.borrow_mut().get_manager_version()
    }



    fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
        self.borrow_mut().get_all_tasks(task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, )
    }

    fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
        self.borrow_mut().get_task(task_id, )
    }

    fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
        self.borrow_mut().get_task_status(task_id, )
    }

    fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
        self.borrow_mut().update_task(task_id, body, )
    }



    fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
        self.borrow_mut().convert_app(body, )
    }



    fn accept_terms_and_conditions(&self) -> Result<(), Self::Error> {
        self.borrow_mut().accept_terms_and_conditions()
    }

    fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
        self.borrow_mut().change_password(body, )
    }

    fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
        self.borrow_mut().confirm_email(body, )
    }

    fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
        self.borrow_mut().create_user(body, )
    }

    fn delete_user_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_user_account(user_id, )
    }

    fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_user_from_account(user_id, )
    }

    fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
        self.borrow_mut().forgot_password(body, )
    }

    fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
        self.borrow_mut().get_all_users(all_search, limit, offset, sort_by, )
    }

    fn get_logged_in_user(&self) -> Result<models::User, Self::Error> {
        self.borrow_mut().get_logged_in_user()
    }

    fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
        self.borrow_mut().get_user(user_id, )
    }

    fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
        self.borrow_mut().invite_user(body, )
    }

    fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
        self.borrow_mut().process_invitations(body, )
    }

    fn resend_confirm_email(&self) -> Result<(), Self::Error> {
        self.borrow_mut().resend_confirm_email()
    }

    fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().resend_invitation(user_id, )
    }

    fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
        self.borrow_mut().reset_password(user_id, body, )
    }

    fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
        self.borrow_mut().update_user(user_id, body, )
    }

    fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
        self.borrow_mut().validate_password_reset_token(user_id, body, )
    }



    fn create_workflow_graph(&self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        self.borrow_mut().create_workflow_graph(body, )
    }

    fn delete_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.borrow_mut().delete_workflow_graph(graph_id, )
    }

    fn get_all_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error> {
        self.borrow_mut().get_all_workflow_graphs(name, description, all_search, parent_graph_id, sort_by, limit, offset, )
    }

    fn get_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error> {
        self.borrow_mut().get_workflow_graph(graph_id, )
    }

    fn update_workflow_graph(&self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        self.borrow_mut().update_workflow_graph(graph_id, body, )
    }



    fn create_final_workflow_graph(&self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error> {
        self.borrow_mut().create_final_workflow_graph(body, )
    }

    fn delete_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error> {
        self.borrow_mut().delete_final_workflow_graph(graph_id, version, )
    }

    fn get_all_final_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error> {
        self.borrow_mut().get_all_final_workflow_graphs(name, description, all_search, sort_by, limit, offset, )
    }

    fn get_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        self.borrow_mut().get_final_workflow_graph(graph_id, version, )
    }

    fn get_full_final_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error> {
        self.borrow_mut().get_full_final_workflow_graph(graph_id, )
    }

    fn update_final_workflow_graph(&self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        self.borrow_mut().update_final_workflow_graph(graph_id, body, )
    }



    fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
        self.borrow_mut().get_zone(zone_id, )
    }

    fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
        self.borrow_mut().get_zone_join_token(zone_id, )
    }

    fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error> {
        self.borrow_mut().get_zones()
    }


}


pub trait AccountsApi {
    type Error;


    /// Create a new account.
    fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete an account.
    fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account.
    fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts.
    fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on.
    fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update an account.
    fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;

}

pub trait AccountsApiMut {
    type Error;


    /// Create a new account.
    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete an account.
    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account.
    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts.
    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on.
    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update an account.
    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> AccountsApiMut for T
where
    T: AccountsApi<Error = E>,
{
    type Error = E;

    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
        <T as AccountsApi>::create_account(self, body, )
    }

    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as AccountsApi>::delete_account(self, account_id, )
    }

    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
        <T as AccountsApi>::get_account(self, account_id, )
    }

    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error> {
        <T as AccountsApi>::get_accounts(self, )
    }

    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as AccountsApi>::select_account(self, account_id, )
    }

    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
        <T as AccountsApi>::update_account(self, account_id, body, )
    }

}


pub trait AppApi {
    type Error;


    /// Add an application.
    fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Delete a particular app
    fn delete_app(&self, app_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all apps information.
    fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app.
    fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate.
    fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an app's certificate for a compute node.
    fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the applications within selected account
    fn get_apps_unique_labels(&self) -> Result<models::LabelsCount, Self::Error>;

    /// Update details of a particular app.
    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;

}

pub trait AppApiMut {
    type Error;


    /// Add an application.
    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Delete a particular app
    fn delete_app(&mut self, app_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all apps information.
    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app.
    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate.
    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an app's certificate for a compute node.
    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the applications within selected account
    fn get_apps_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error>;

    /// Update details of a particular app.
    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> AppApiMut for T
where
    T: AppApi<Error = E>,
{
    type Error = E;

    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error> {
        <T as AppApi>::add_application(self, body, )
    }

    fn delete_app(&mut self, app_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as AppApi>::delete_app(self, app_id, )
    }

    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
        <T as AppApi>::get_all_apps(self, name, description, all_search, limit, offset, sort_by, )
    }

    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
        <T as AppApi>::get_app(self, app_id, )
    }

    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        <T as AppApi>::get_app_certificate(self, node_id, app_id, )
    }

    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        <T as AppApi>::get_app_node_certificate_details(self, node_id, app_id, )
    }

    fn get_apps_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error> {
        <T as AppApi>::get_apps_unique_labels(self, )
    }

    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        <T as AppApi>::update_app(self, app_id, body, )
    }

}


pub trait ApplicationConfigApi {
    type Error;


    /// Add an app config.
    fn create_application_config(&self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Delete a particular app config
    fn delete_application_config(&self, config_id: String) -> Result<(), Self::Error>;

    /// Get all app configs
    fn get_all_application_configs(&self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error>;

    /// Get details of a particular app config.
    fn get_application_config(&self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Get app config
    fn get_runtime_application_config(&self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Get details of a particular runtime app config.
    fn get_specific_runtime_application_config(&self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Update details of a particular app config.
    fn update_application_config(&self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error>;

}

pub trait ApplicationConfigApiMut {
    type Error;


    /// Add an app config.
    fn create_application_config(&mut self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Delete a particular app config
    fn delete_application_config(&mut self, config_id: String) -> Result<(), Self::Error>;

    /// Get all app configs
    fn get_all_application_configs(&mut self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error>;

    /// Get details of a particular app config.
    fn get_application_config(&mut self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error>;

    /// Get app config
    fn get_runtime_application_config(&mut self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Get details of a particular runtime app config.
    fn get_specific_runtime_application_config(&mut self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error>;

    /// Update details of a particular app config.
    fn update_application_config(&mut self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> ApplicationConfigApiMut for T
where
    T: ApplicationConfigApi<Error = E>,
{
    type Error = E;

    fn create_application_config(&mut self, body: models::ApplicationConfig) -> Result<models::ApplicationConfigResponse, Self::Error> {
        <T as ApplicationConfigApi>::create_application_config(self, body, )
    }

    fn delete_application_config(&mut self, config_id: String) -> Result<(), Self::Error> {
        <T as ApplicationConfigApi>::delete_application_config(self, config_id, )
    }

    fn get_all_application_configs(&mut self, name: Option<String>, description: Option<String>, image_id: Option<uuid::Uuid>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApplicationConfigsResponse, Self::Error> {
        <T as ApplicationConfigApi>::get_all_application_configs(self, name, description, image_id, limit, offset, )
    }

    fn get_application_config(&mut self, config_id: String) -> Result<models::ApplicationConfigResponse, Self::Error> {
        <T as ApplicationConfigApi>::get_application_config(self, config_id, )
    }

    fn get_runtime_application_config(&mut self, expected_hash: &[u8; 32]) -> Result<models::RuntimeAppConfig, Self::Error> {
        <T as ApplicationConfigApi>::get_runtime_application_config(self, expected_hash)
    }

    fn get_specific_runtime_application_config(&mut self, config_id: String) -> Result<models::RuntimeAppConfig, Self::Error> {
        <T as ApplicationConfigApi>::get_specific_runtime_application_config(self, config_id, )
    }

    fn update_application_config(&mut self, config_id: String, body: models::UpdateApplicationConfigRequest) -> Result<models::ApplicationConfigResponse, Self::Error> {
        <T as ApplicationConfigApi>::update_application_config(self, config_id, body, )
    }

}


pub trait ApprovalRequestsApi {
    type Error;


    /// Approve a request.
    fn approve_approval_request(&self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Create approval request.
    fn create_approval_request(&self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error>;

    /// Delete an approval request.
    fn delete_approval_request(&self, request_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Deny a request.
    fn deny_approval_request(&self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get all approval requests
    fn get_all_approval_requests(&self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error>;

    /// Get an approval request.
    fn get_approval_request(&self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get the result for an approved or failed request.
    fn get_approval_request_result(&self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error>;

}

pub trait ApprovalRequestsApiMut {
    type Error;


    /// Approve a request.
    fn approve_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Create approval request.
    fn create_approval_request(&mut self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error>;

    /// Delete an approval request.
    fn delete_approval_request(&mut self, request_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Deny a request.
    fn deny_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get all approval requests
    fn get_all_approval_requests(&mut self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error>;

    /// Get an approval request.
    fn get_approval_request(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error>;

    /// Get the result for an approved or failed request.
    fn get_approval_request_result(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> ApprovalRequestsApiMut for T
where
    T: ApprovalRequestsApi<Error = E>,
{
    type Error = E;

    fn approve_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::ApproveRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        <T as ApprovalRequestsApi>::approve_approval_request(self, request_id, body, )
    }

    fn create_approval_request(&mut self, body: models::ApprovalRequestRequest) -> Result<models::ApprovalRequest, Self::Error> {
        <T as ApprovalRequestsApi>::create_approval_request(self, body, )
    }

    fn delete_approval_request(&mut self, request_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as ApprovalRequestsApi>::delete_approval_request(self, request_id, )
    }

    fn deny_approval_request(&mut self, request_id: uuid::Uuid, body: Option<models::DenyRequest>) -> Result<models::ApprovalRequest, Self::Error> {
        <T as ApprovalRequestsApi>::deny_approval_request(self, request_id, body, )
    }

    fn get_all_approval_requests(&mut self, requester: Option<uuid::Uuid>, reviewer: Option<uuid::Uuid>, subject: Option<uuid::Uuid>, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllApprovalRequests, Self::Error> {
        <T as ApprovalRequestsApi>::get_all_approval_requests(self, requester, reviewer, subject, status, all_search, sort_by, limit, offset, )
    }

    fn get_approval_request(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovalRequest, Self::Error> {
        <T as ApprovalRequestsApi>::get_approval_request(self, request_id, )
    }

    fn get_approval_request_result(&mut self, request_id: uuid::Uuid) -> Result<models::ApprovableResult, Self::Error> {
        <T as ApprovalRequestsApi>::get_approval_request_result(self, request_id, )
    }

}


pub trait AuthApi {
    type Error;


    /// User authentication
    fn authenticate_user(&self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error>;

}

pub trait AuthApiMut {
    type Error;


    /// User authentication
    fn authenticate_user(&mut self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> AuthApiMut for T
where
    T: AuthApi<Error = E>,
{
    type Error = E;

    fn authenticate_user(&mut self, body: Option<models::AuthRequest>) -> Result<models::AuthResponse, Self::Error> {
        <T as AuthApi>::authenticate_user(self, body, )
    }

}


pub trait BuildApi {
    type Error;


    /// Convert a docker image and create a new image.
    fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new image.
    fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular image.
    fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all images information.
    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular image.
    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of an image.
    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

    /// Update details of a particular image.
    fn update_build(&self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error>;

}

pub trait BuildApiMut {
    type Error;


    /// Convert a docker image and create a new image.
    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new image.
    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular image.
    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all images information.
    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular image.
    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of an image.
    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

    /// Update details of a particular image.
    fn update_build(&mut self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> BuildApiMut for T
where
    T: BuildApi<Error = E>,
{
    type Error = E;

    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
        <T as BuildApi>::convert_app_build(self, body, )
    }

    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
        <T as BuildApi>::create_build(self, body, )
    }

    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as BuildApi>::delete_build(self, build_id, )
    }

    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, config_id: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        <T as BuildApi>::get_all_builds(self, all_search, docker_image_name, config_id, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        <T as BuildApi>::get_build(self, build_id, )
    }

    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        <T as BuildApi>::get_build_deployments(self, build_id, status, all_search, sort_by, limit, offset, )
    }

    fn update_build(&mut self, build_id: uuid::Uuid, body: models::BuildUpdateRequest) -> Result<models::Build, Self::Error> {
        <T as BuildApi>::update_build(self, build_id, body, )
    }

}


pub trait CertificateApi {
    type Error;


    /// Retrieve a certificate.
    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Request a new certificate for an Enclave application
    fn new_certificate(&self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error>;

}

pub trait CertificateApiMut {
    type Error;


    /// Retrieve a certificate.
    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Request a new certificate for an Enclave application
    fn new_certificate(&mut self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> CertificateApiMut for T
where
    T: CertificateApi<Error = E>,
{
    type Error = E;

    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        <T as CertificateApi>::get_certificate(self, cert_id, )
    }

    fn new_certificate(&mut self, body: models::NewCertificateRequest) -> Result<models::TaskResult, Self::Error> {
        <T as CertificateApi>::new_certificate(self, body, )
    }

}


pub trait DatasetApi {
    type Error;



    fn create_dataset(&self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error>;


    fn delete_dataset(&self, dataset_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all datasets
    fn get_all_datasets(&self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error>;


    fn get_dataset(&self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error>;


    fn update_dataset(&self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error>;

}

pub trait DatasetApiMut {
    type Error;



    fn create_dataset(&mut self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error>;


    fn delete_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all datasets
    fn get_all_datasets(&mut self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error>;


    fn get_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error>;


    fn update_dataset(&mut self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> DatasetApiMut for T
where
    T: DatasetApi<Error = E>,
{
    type Error = E;

    fn create_dataset(&mut self, body: models::CreateDatasetRequest) -> Result<models::Dataset, Self::Error> {
        <T as DatasetApi>::create_dataset(self, body, )
    }

    fn delete_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as DatasetApi>::delete_dataset(self, dataset_id, )
    }

    fn get_all_datasets(&mut self, name: Option<String>, description: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllDatasetsResponse, Self::Error> {
        <T as DatasetApi>::get_all_datasets(self, name, description, limit, offset, )
    }

    fn get_dataset(&mut self, dataset_id: uuid::Uuid) -> Result<models::Dataset, Self::Error> {
        <T as DatasetApi>::get_dataset(self, dataset_id, )
    }

    fn update_dataset(&mut self, dataset_id: uuid::Uuid, body: models::DatasetUpdateRequest) -> Result<models::Dataset, Self::Error> {
        <T as DatasetApi>::update_dataset(self, dataset_id, body, )
    }

}


pub trait NodeApi {
    type Error;


    /// Deactivate a particular compute node.
    fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all compute nodes information.
    fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular compute node.
    fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested compute node's certificate.
    fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get a compute node's certificate.
    fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the nodes within selected account
    fn get_nodes_unique_labels(&self) -> Result<models::LabelsCount, Self::Error>;

    /// Provision a new compute node.
    fn provision_node(&self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error>;

    /// Update details of a particular compute node.
    fn update_node(&self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error>;

    /// Called periodically by a compute node.
    fn update_node_status(&self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error>;

}

pub trait NodeApiMut {
    type Error;


    /// Deactivate a particular compute node.
    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all compute nodes information.
    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular compute node.
    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested compute node's certificate.
    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get a compute node's certificate.
    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get all the unique labels across all the nodes within selected account
    fn get_nodes_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error>;

    /// Provision a new compute node.
    fn provision_node(&mut self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error>;

    /// Update details of a particular compute node.
    fn update_node(&mut self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error>;

    /// Called periodically by a compute node.
    fn update_node_status(&mut self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> NodeApiMut for T
where
    T: NodeApi<Error = E>,
{
    type Error = E;

    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as NodeApi>::deactivate_node(self, node_id, )
    }

    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
        <T as NodeApi>::get_all_nodes(self, name, description, sgx_version, all_search, status, limit, offset, sort_by, )
    }

    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
        <T as NodeApi>::get_node(self, node_id, )
    }

    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        <T as NodeApi>::get_node_certificate(self, node_id, )
    }

    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        <T as NodeApi>::get_node_certificate_details(self, node_id, )
    }

    fn get_nodes_unique_labels(&mut self) -> Result<models::LabelsCount, Self::Error> {
        <T as NodeApi>::get_nodes_unique_labels(self, )
    }

    fn provision_node(&mut self, body: models::NodeProvisionRequest) -> Result<models::TaskResult, Self::Error> {
        <T as NodeApi>::provision_node(self, body, )
    }

    fn update_node(&mut self, node_id: uuid::Uuid, body: models::NodeUpdateRequest) -> Result<models::Node, Self::Error> {
        <T as NodeApi>::update_node(self, node_id, body, )
    }

    fn update_node_status(&mut self, body: models::NodeStatusRequest) -> Result<models::NodeStatusResponse, Self::Error> {
        <T as NodeApi>::update_node_status(self, body, )
    }

}


pub trait RegistryApi {
    type Error;


    /// Add a new registry to an account
    fn create_registry(&self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error>;

    /// Delete registry
    fn delete_registry(&self, registry_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get details of all registry in the account
    fn get_all_registries(&self) -> Result<Vec<models::Registry>, Self::Error>;

    /// Get details of a particular registry
    fn get_registry(&self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error>;

    /// Get details of the registry that will be used for the particular app images
    fn get_registry_for_app(&self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error>;

    /// Get details of the registry that will be used for the particular image
    fn get_registry_for_image(&self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error>;

    /// Update a particular registry details
    fn update_registry(&self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error>;

}

pub trait RegistryApiMut {
    type Error;


    /// Add a new registry to an account
    fn create_registry(&mut self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error>;

    /// Delete registry
    fn delete_registry(&mut self, registry_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get details of all registry in the account
    fn get_all_registries(&mut self) -> Result<Vec<models::Registry>, Self::Error>;

    /// Get details of a particular registry
    fn get_registry(&mut self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error>;

    /// Get details of the registry that will be used for the particular app images
    fn get_registry_for_app(&mut self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error>;

    /// Get details of the registry that will be used for the particular image
    fn get_registry_for_image(&mut self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error>;

    /// Update a particular registry details
    fn update_registry(&mut self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> RegistryApiMut for T
where
    T: RegistryApi<Error = E>,
{
    type Error = E;

    fn create_registry(&mut self, registry_request: models::RegistryRequest) -> Result<models::Registry, Self::Error> {
        <T as RegistryApi>::create_registry(self, registry_request, )
    }

    fn delete_registry(&mut self, registry_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as RegistryApi>::delete_registry(self, registry_id, )
    }

    fn get_all_registries(&mut self) -> Result<Vec<models::Registry>, Self::Error> {
        <T as RegistryApi>::get_all_registries(self, )
    }

    fn get_registry(&mut self, registry_id: uuid::Uuid) -> Result<models::Registry, Self::Error> {
        <T as RegistryApi>::get_registry(self, registry_id, )
    }

    fn get_registry_for_app(&mut self, app_id: uuid::Uuid) -> Result<models::AppRegistryResponse, Self::Error> {
        <T as RegistryApi>::get_registry_for_app(self, app_id, )
    }

    fn get_registry_for_image(&mut self, image_name: String) -> Result<models::ImageRegistryResponse, Self::Error> {
        <T as RegistryApi>::get_registry_for_image(self, image_name, )
    }

    fn update_registry(&mut self, registry_id: uuid::Uuid, body: models::UpdateRegistryRequest) -> Result<models::Registry, Self::Error> {
        <T as RegistryApi>::update_registry(self, registry_id, body, )
    }

}


pub trait SystemApi {
    type Error;


    /// Get Manager Version.
    fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error>;

}

pub trait SystemApiMut {
    type Error;


    /// Get Manager Version.
    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> SystemApiMut for T
where
    T: SystemApi<Error = E>,
{
    type Error = E;

    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error> {
        <T as SystemApi>::get_manager_version(self, )
    }

}


pub trait TaskApi {
    type Error;


    /// Get all the tasks.
    fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task.
    fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task.
    fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task.
    fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;

}

pub trait TaskApiMut {
    type Error;


    /// Get all the tasks.
    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task.
    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task.
    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task.
    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> TaskApiMut for T
where
    T: TaskApi<Error = E>,
{
    type Error = E;

    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
        <T as TaskApi>::get_all_tasks(self, task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, )
    }

    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
        <T as TaskApi>::get_task(self, task_id, )
    }

    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
        <T as TaskApi>::get_task_status(self, task_id, )
    }

    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
        <T as TaskApi>::update_task(self, task_id, body, )
    }

}


pub trait ToolsApi {
    type Error;


    /// Convert an application to run in EnclaveOS.
    fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;

}

pub trait ToolsApiMut {
    type Error;


    /// Convert an application to run in EnclaveOS.
    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> ToolsApiMut for T
where
    T: ToolsApi<Error = E>,
{
    type Error = E;

    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
        <T as ToolsApi>::convert_app(self, body, )
    }

}


pub trait UsersApi {
    type Error;


    /// Current user accepts latest terms and conditions.
    fn accept_terms_and_conditions(&self) -> Result<(), Self::Error>;

    /// Change user password.
    fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address.
    fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user.
    fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Completely delete a user profile from system
    fn delete_user_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Removed user's association with an account.
    fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user.
    fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all user's information.
    fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user.
    fn get_logged_in_user(&self) -> Result<models::User, Self::Error>;

    /// Get details of a particular user.
    fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user.
    fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations.
    fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address.
    fn resend_confirm_email(&self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account.
    fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password.
    fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, and the role of a user. User with MANAGER access role can only update another user.
    fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user.
    fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

}

pub trait UsersApiMut {
    type Error;


    /// Current user accepts latest terms and conditions.
    fn accept_terms_and_conditions(&mut self) -> Result<(), Self::Error>;

    /// Change user password.
    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address.
    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user.
    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Completely delete a user profile from system
    fn delete_user_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Removed user's association with an account.
    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user.
    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all user's information.
    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user.
    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error>;

    /// Get details of a particular user.
    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user.
    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations.
    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address.
    fn resend_confirm_email(&mut self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account.
    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password.
    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, and the role of a user. User with MANAGER access role can only update another user.
    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user.
    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> UsersApiMut for T
where
    T: UsersApi<Error = E>,
{
    type Error = E;

    fn accept_terms_and_conditions(&mut self) -> Result<(), Self::Error> {
        <T as UsersApi>::accept_terms_and_conditions(self, )
    }

    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
        <T as UsersApi>::change_password(self, body, )
    }

    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
        <T as UsersApi>::confirm_email(self, body, )
    }

    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
        <T as UsersApi>::create_user(self, body, )
    }

    fn delete_user_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as UsersApi>::delete_user_account(self, user_id, )
    }

    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as UsersApi>::delete_user_from_account(self, user_id, )
    }

    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
        <T as UsersApi>::forgot_password(self, body, )
    }

    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
        <T as UsersApi>::get_all_users(self, all_search, limit, offset, sort_by, )
    }

    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error> {
        <T as UsersApi>::get_logged_in_user(self, )
    }

    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
        <T as UsersApi>::get_user(self, user_id, )
    }

    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
        <T as UsersApi>::invite_user(self, body, )
    }

    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
        <T as UsersApi>::process_invitations(self, body, )
    }

    fn resend_confirm_email(&mut self) -> Result<(), Self::Error> {
        <T as UsersApi>::resend_confirm_email(self, )
    }

    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as UsersApi>::resend_invitation(self, user_id, )
    }

    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
        <T as UsersApi>::reset_password(self, user_id, body, )
    }

    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
        <T as UsersApi>::update_user(self, user_id, body, )
    }

    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
        <T as UsersApi>::validate_password_reset_token(self, user_id, body, )
    }

}


pub trait WorkflowApi {
    type Error;



    fn create_workflow_graph(&self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

    /// Delete a particular draft workflow
    fn delete_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<(), Self::Error>;


    fn get_all_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular draft workflow
    fn get_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error>;


    fn update_workflow_graph(&self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

}

pub trait WorkflowApiMut {
    type Error;



    fn create_workflow_graph(&mut self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

    /// Delete a particular draft workflow
    fn delete_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<(), Self::Error>;


    fn get_all_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular draft workflow
    fn get_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error>;


    fn update_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> WorkflowApiMut for T
where
    T: WorkflowApi<Error = E>,
{
    type Error = E;

    fn create_workflow_graph(&mut self, body: models::CreateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        <T as WorkflowApi>::create_workflow_graph(self, body, )
    }

    fn delete_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<(), Self::Error> {
        <T as WorkflowApi>::delete_workflow_graph(self, graph_id, )
    }

    fn get_all_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, parent_graph_id: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllWorkflowGraphsResponse, Self::Error> {
        <T as WorkflowApi>::get_all_workflow_graphs(self, name, description, all_search, parent_graph_id, sort_by, limit, offset, )
    }

    fn get_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::WorkflowGraph, Self::Error> {
        <T as WorkflowApi>::get_workflow_graph(self, graph_id, )
    }

    fn update_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::UpdateWorkflowGraph) -> Result<models::WorkflowGraph, Self::Error> {
        <T as WorkflowApi>::update_workflow_graph(self, graph_id, body, )
    }

}


pub trait WorkflowFinalApi {
    type Error;



    fn create_final_workflow_graph(&self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error>;

    /// Delete a particular final workflow
    fn delete_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error>;


    fn get_all_final_workflow_graphs(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular final workflow version
    fn get_final_workflow_graph(&self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error>;

    /// Get details of a particular final workflow
    fn get_full_final_workflow_graph(&self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error>;

    /// Create a new version for a particular final workflow
    fn update_final_workflow_graph(&self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error>;

}

pub trait WorkflowFinalApiMut {
    type Error;



    fn create_final_workflow_graph(&mut self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error>;

    /// Delete a particular final workflow
    fn delete_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error>;


    fn get_all_final_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error>;

    /// Get details of a particular final workflow version
    fn get_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error>;

    /// Get details of a particular final workflow
    fn get_full_final_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error>;

    /// Create a new version for a particular final workflow
    fn update_final_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> WorkflowFinalApiMut for T
where
    T: WorkflowFinalApi<Error = E>,
{
    type Error = E;

    fn create_final_workflow_graph(&mut self, body: models::CreateFinalWorkflowGraph) -> Result<models::FinalWorkflow, Self::Error> {
        <T as WorkflowFinalApi>::create_final_workflow_graph(self, body, )
    }

    fn delete_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<(), Self::Error> {
        <T as WorkflowFinalApi>::delete_final_workflow_graph(self, graph_id, version, )
    }

    fn get_all_final_workflow_graphs(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllFinalWorkflowGraphsResponse, Self::Error> {
        <T as WorkflowFinalApi>::get_all_final_workflow_graphs(self, name, description, all_search, sort_by, limit, offset, )
    }

    fn get_final_workflow_graph(&mut self, graph_id: uuid::Uuid, version: String) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        <T as WorkflowFinalApi>::get_final_workflow_graph(self, graph_id, version, )
    }

    fn get_full_final_workflow_graph(&mut self, graph_id: uuid::Uuid) -> Result<models::FinalWorkflow, Self::Error> {
        <T as WorkflowFinalApi>::get_full_final_workflow_graph(self, graph_id, )
    }

    fn update_final_workflow_graph(&mut self, graph_id: uuid::Uuid, body: models::CreateWorkflowVersionRequest) -> Result<models::VersionInFinalWorkflow, Self::Error> {
        <T as WorkflowFinalApi>::update_final_workflow_graph(self, graph_id, body, )
    }

}


pub trait ZoneApi {
    type Error;


    /// Get zone details.
    fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get the authentication token.
    fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones.
    fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error>;

}

pub trait ZoneApiMut {
    type Error;


    /// Get zone details.
    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get the authentication token.
    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones.
    fn get_zones(&mut self) -> Result<Vec<models::Zone>, Self::Error>;

}

// This is mostly so that we don't have to convert all the malbork APIs to
// ApiMut at once.
impl<T, E> ZoneApiMut for T
where
    T: ZoneApi<Error = E>,
{
    type Error = E;

    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
        <T as ZoneApi>::get_zone(self, zone_id, )
    }

    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
        <T as ZoneApi>::get_zone_join_token(self, zone_id, )
    }

    fn get_zones(&mut self) -> Result<Vec<models::Zone>, Self::Error> {
        <T as ZoneApi>::get_zones(self, )
    }

}



#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

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
use std::convert::TryFrom;

/// Very simple error type - just holds a description of the error. This is useful for human
/// diagnosis and troubleshooting, but not for applications to parse. The justification for this
/// is to deny applications visibility into the communication layer, forcing the application code
/// to act solely on the logical responses that the API provides, promoting abstraction in the
/// application code.

#[derive(Debug)]
pub struct ApiError {
    message: String,
    error_type: SimpleErrorType,
}

impl ApiError {
    pub fn message(&self) -> &str {
        &self.message
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
    pub fn new(message: String, error_type: SimpleErrorType) -> ApiError {
        ApiError {
            message,
            error_type
        }
    }
}

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
        ApiError::new(e.to_string(), SimpleErrorType::Temporary)
    }
}

impl From<String> for ApiError {
    fn from(e: String) -> Self {
        ApiError::new(e, SimpleErrorType::Temporary)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::new(format!("Response body did not match the schema: {}", e), SimpleErrorType::Temporary)
    }
}

#[derive(Debug)]
pub struct ServerError {
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    BadRequest,
    Forbidden,
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
            ErrorType::BadRequest => write!(f, "{}", "BadRequest"),
            ErrorType::Forbidden => write!(f, "{}", "Forbidden"),
            ErrorType::InvalidPathParameter => write!(f, "{}", "InvalidPathParameter"),
            ErrorType::InvalidBodyParameter => write!(f, "{}", "InvalidBodyParameter"),
            ErrorType::InvalidQueryParameter => write!(f, "{}", "InvalidQueryParameter"),
            ErrorType::MissingParameter => write!(f, "{}", "MissingParameter"),
            ErrorType::NotFound => write!(f, "{}", "NotFound"),
            ErrorType::MethodNotAllowed => write!(f, "{}", "MethodNotAllowed"),
            ErrorType::InvalidHeader => write!(f, "{}", "InvalidHeader"),
        }
    }
}

/// Describes SHA256 hash sum in byte format
#[derive(Debug)]
pub struct Sha256Hash([u8; SHA256_BYTE_LENGTH]);

impl TryFrom<&str> for Sha256Hash {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != SHA256_CHAR_LENGTH {
            return Err(format!("SHA-256 string should be exactly {} characters long, instead got a string of len {}", SHA256_CHAR_LENGTH, value.len()))
        } else if !(value.chars().all(|c| c.is_ascii_hexdigit())) {
            return Err(format!("SHA-256 string should contain only hexadecimal characters in the format [0-9a-fA-F], but got {}", value))
        } else {
            let mut result = [0u8; SHA256_BYTE_LENGTH];

            for i in 0..SHA256_BYTE_LENGTH {
                // We iterate input string by chunks of 2 because 1 hex char is half a byte.
                let chunk = &value[2 * i..2 * i + 2];
                result[i] = u8::from_str_radix(chunk, 16).map_err(|err| {
                    format!(
                        "Invalid hex format for chunk '{}' at position {}. Error {:?}",
                        chunk, i, err
                    )
                })?;
            }

            Ok(Sha256Hash(result))
        }
    }
}
