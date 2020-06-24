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



pub trait Api {
    type Error;



    /// Create a new account
    fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete account
    fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account
    fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts
    fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on
    fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update account
    fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;




    /// Add an application
    fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Get all apps information
    fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app
    fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate
    fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an App's Certificate for a Node
    fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Update details of a particular app
    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;




    /// Non OAuth based User authentication
    fn authenticate_user(&self) -> Result<models::AuthResponse, Self::Error>;

    /// Convert a docker image and create a new build
    fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new build
    fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular build
    fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all builds information
    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular build
    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of a build
    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;



    /// Retrieve a certificate
    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Deactivate a particular node
    fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all nodes information
    fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular node
    fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested node's certificate
    fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an node's certificate
    fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;


    /// Get Manager Version
    fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error>;



    /// Get all the tasks
    fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task
    fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task
    fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task
    fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;



    /// Convert an application to run in EnclaveOS
    fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;



    /// Disable a user using its email
    fn blacklist_user(&self, body: models::UserBlacklistRequest) -> Result<(), Self::Error>;

    /// Change user password
    fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address
    fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user
    fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Removed user's association with an account
    fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user
    fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all users information
    fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user
    fn get_logged_in_user(&self) -> Result<models::User, Self::Error>;

    /// Get details of a particular User
    fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user
    fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations
    fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address
    fn resend_confirm_email(&self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account
    fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password
    fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, role of a user. User with MANAGER access role can only update another user
    fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user
    fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

    /// Whitelist a user
    fn whitelist_user(&self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error>;



    /// Get zone details
    fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get authentication token
    fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones
    fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error>;

}

pub trait ApiMut {
    type Error;



    /// Create a new account
    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete account
    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account
    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts
    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on
    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update account
    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;




    /// Add an application
    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Get all apps information
    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app
    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate
    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an App's Certificate for a Node
    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Update details of a particular app
    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;




    /// Non OAuth based User authentication
    fn authenticate_user(&mut self) -> Result<models::AuthResponse, Self::Error>;

    /// Convert a docker image and create a new build
    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new build
    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular build
    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all builds information
    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular build
    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of a build
    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;



    /// Retrieve a certificate
    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Deactivate a particular node
    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all nodes information
    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular node
    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested node's certificate
    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an node's certificate
    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Get Manager Version
    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error>;



    /// Get all the tasks
    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task
    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task
    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task
    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;



    /// Convert an application to run in EnclaveOS
    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;



    /// Disable a user using its email
    fn blacklist_user(&mut self, body: models::UserBlacklistRequest) -> Result<(), Self::Error>;

    /// Change user password
    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address
    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user
    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Removed user's association with an account
    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user
    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all users information
    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user
    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error>;

    /// Get details of a particular User
    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user
    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations
    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address
    fn resend_confirm_email(&mut self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account
    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password
    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, role of a user. User with MANAGER access role can only update another user
    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user
    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

    /// Whitelist a user
    fn whitelist_user(&mut self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error>;




    /// Get zone details
    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get authentication token
    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones
    fn get_zones(&mut self) -> Result<Vec<models::Zone>, Self::Error>;


}

impl<T, E> Api for T
where
T: AccountsApi<Error = E> + AppApi<Error = E> + AuthApi<Error = E> + BuildApi<Error = E> + CertificateApi<Error = E> + NodeApi<Error = E> + SystemApi<Error = E> + TaskApi<Error = E> + ToolsApi<Error = E> + UsersApi<Error = E> + ZoneApi<Error = E> + 
{
type Error = E;


    
        fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error> {
        self.create_account(body, )
        }
    
        fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_account(account_id, )
        }
    
        fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error> {
        self.get_account(account_id, )
        }
    
        fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error> {
        self.get_accounts()
        }
    
        fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.select_account(account_id, )
        }
    
        fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error> {
        self.update_account(account_id, body, )
        }
    

    
    
        fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error> {
        self.add_application(body, )
        }
    
        fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error> {
        self.get_all_apps(name, description, all_search, limit, offset, sort_by, )
        }
    
        fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error> {
        self.get_app(app_id, )
        }
    
        fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_app_certificate(node_id, app_id, )
        }
    
        fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.get_app_node_certificate_details(node_id, app_id, )
        }
    
        fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        self.update_app(app_id, body, )
        }
    

    
        fn authenticate_user(&self) -> Result<models::AuthResponse, Self::Error> {
        self.authenticate_user()
        }
    
        fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error> {
        self.convert_app_build(body, )
        }
    
        fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error> {
        self.create_build(body, )
        }
    
        fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_build(build_id, )
        }
    
        fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        self.get_all_builds(all_search, docker_image_name, deployed_status, status, limit, offset, sort_by, )
        }
    
        fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        self.get_build(build_id, )
        }
    
        fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        self.get_build_deployments(build_id, status, all_search, sort_by, limit, offset, )
        }
    

    
        fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_certificate(cert_id, )
        }
    
        fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.deactivate_node(node_id, )
        }
    
        fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error> {
        self.get_all_nodes(name, description, sgx_version, all_search, status, limit, offset, sort_by, )
        }
    
        fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error> {
        self.get_node(node_id, )
        }
    
        fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_node_certificate(node_id, )
        }
    
        fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error> {
        self.get_node_certificate_details(node_id, )
        }
    
        fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error> {
        self.get_manager_version()
        }
    

    
        fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error> {
        self.get_all_tasks(task_type, status, requester, approver, all_search, limit, offset, sort_by, base_filters, )
        }
    
        fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error> {
        self.get_task(task_id, )
        }
    
        fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error> {
        self.get_task_status(task_id, )
        }
    
        fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error> {
        self.update_task(task_id, body, )
        }
    

    
        fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error> {
        self.convert_app(body, )
        }
    

    
        fn blacklist_user(&self, body: models::UserBlacklistRequest) -> Result<(), Self::Error> {
        self.blacklist_user(body, )
        }
    
        fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error> {
        self.change_password(body, )
        }
    
        fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error> {
        self.confirm_email(body, )
        }
    
        fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error> {
        self.create_user(body, )
        }
    
        fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.delete_user_from_account(user_id, )
        }
    
        fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error> {
        self.forgot_password(body, )
        }
    
        fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error> {
        self.get_all_users(all_search, limit, offset, sort_by, )
        }
    
        fn get_logged_in_user(&self) -> Result<models::User, Self::Error> {
        self.get_logged_in_user()
        }
    
        fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error> {
        self.get_user(user_id, )
        }
    
        fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error> {
        self.invite_user(body, )
        }
    
        fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error> {
        self.process_invitations(body, )
        }
    
        fn resend_confirm_email(&self) -> Result<(), Self::Error> {
        self.resend_confirm_email()
        }
    
        fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error> {
        self.resend_invitation(user_id, )
        }
    
        fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error> {
        self.reset_password(user_id, body, )
        }
    
        fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error> {
        self.update_user(user_id, body, )
        }
    
        fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error> {
        self.validate_password_reset_token(user_id, body, )
        }
    
        fn whitelist_user(&self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error> {
        self.whitelist_user(user_id, user_token, )
        }
    

        
        fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error> {
        self.get_zone(zone_id, )
        }
    
        fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error> {
        self.get_zone_join_token(zone_id, )
        }
    
        fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error> {
        self.get_zones()
        }

}

impl<T, E> ApiMut for T
where
    T: AccountsApiMut<Error = E> + AppApiMut<Error = E> + AuthApiMut<Error = E> + BuildApiMut<Error = E> + CertificateApiMut<Error = E> + NodeApiMut<Error = E> + SystemApiMut<Error = E> + TaskApiMut<Error = E> + ToolsApiMut<Error = E> + UsersApiMut<Error = E> + ZoneApiMut<Error = E> + 
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

    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        self.update_app(app_id, body, )
    }

    fn authenticate_user(&mut self) -> Result<models::AuthResponse, Self::Error> {
        self.authenticate_user()
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

    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        self.get_all_builds(all_search, docker_image_name, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        self.get_build(build_id, )
    }

    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        self.get_build_deployments(build_id, status, all_search, sort_by, limit, offset, )
    }



    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.get_certificate(cert_id, )
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



    fn blacklist_user(&mut self, body: models::UserBlacklistRequest) -> Result<(), Self::Error> {
        self.blacklist_user(body, )
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

    fn whitelist_user(&mut self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error> {
        self.whitelist_user(user_id, user_token, )
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

    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        self.borrow_mut().update_app(app_id, body, )
    }


    fn authenticate_user(&self) -> Result<models::AuthResponse, Self::Error> {
        self.borrow_mut().authenticate_user()
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

    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        self.borrow_mut().get_all_builds(all_search, docker_image_name, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        self.borrow_mut().get_build(build_id, )
    }

    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        self.borrow_mut().get_build_deployments(build_id, status, all_search, sort_by, limit, offset, )
    }



    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        self.borrow_mut().get_certificate(cert_id, )
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



    fn blacklist_user(&self, body: models::UserBlacklistRequest) -> Result<(), Self::Error> {
        self.borrow_mut().blacklist_user(body, )
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

    fn whitelist_user(&self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error> {
        self.borrow_mut().whitelist_user(user_id, user_token, )
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


    /// Create a new account
    fn create_account(&self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete account
    fn delete_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account
    fn get_account(&self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts
    fn get_accounts(&self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on
    fn select_account(&self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update account
    fn update_account(&self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;

}

pub trait AccountsApiMut {
    type Error;


    /// Create a new account
    fn create_account(&mut self, body: models::AccountRequest) -> Result<models::Account, Self::Error>;

    /// Delete account
    fn delete_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get a specific account
    fn get_account(&mut self, account_id: uuid::Uuid) -> Result<models::Account, Self::Error>;

    /// Get all accounts
    fn get_accounts(&mut self) -> Result<models::AccountListResponse, Self::Error>;

    /// Select a user's account to work on
    fn select_account(&mut self, account_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Update account
    fn update_account(&mut self, account_id: uuid::Uuid, body: models::AccountUpdateRequest) -> Result<models::Account, Self::Error>;

}

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


    /// Add an application
    fn add_application(&self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Get all apps information
    fn get_all_apps(&self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app
    fn get_app(&self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate
    fn get_app_certificate(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an App's Certificate for a Node
    fn get_app_node_certificate_details(&self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Update details of a particular app
    fn update_app(&self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;

}

pub trait AppApiMut {
    type Error;


    /// Add an application
    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error>;

    /// Get all apps information
    fn get_all_apps(&mut self, name: Option<String>, description: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllAppsResponse, Self::Error>;

    /// Get details of a particular app
    fn get_app(&mut self, app_id: uuid::Uuid) -> Result<models::App, Self::Error>;

    /// Get an attested app's certificate
    fn get_app_certificate(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an App's Certificate for a Node
    fn get_app_node_certificate_details(&mut self, node_id: uuid::Uuid, app_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;

    /// Update details of a particular app
    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error>;

}

impl<T, E> AppApiMut for T
where
    T: AppApi<Error = E>,
{
    type Error = E;

    fn add_application(&mut self, body: models::AppRequest) -> Result<models::App, Self::Error> {
        <T as AppApi>::add_application(self, body, )
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

    fn update_app(&mut self, app_id: uuid::Uuid, body: models::AppBodyUpdateRequest) -> Result<models::App, Self::Error> {
        <T as AppApi>::update_app(self, app_id, body, )
    }

}

pub trait AuthApi {
    type Error;


    /// Non OAuth based User authentication
    fn authenticate_user(&self) -> Result<models::AuthResponse, Self::Error>;
}

pub trait AuthApiMut {
    type Error;


    /// Non OAuth based User authentication
    fn authenticate_user(&mut self) -> Result<models::AuthResponse, Self::Error>;
}



impl<T, E> AuthApiMut for T
where
    T: AuthApi<Error = E>,
{
    type Error = E;

    fn authenticate_user(&mut self) -> Result<models::AuthResponse, Self::Error> {
        <T as AuthApi>::authenticate_user(self, )
    }
}


pub trait BuildApi {
    type Error;


    /// Convert a docker image and create a new build
    fn convert_app_build(&self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new build
    fn create_build(&self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular build
    fn delete_build(&self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all builds information
    fn get_all_builds(&self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular build
    fn get_build(&self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of a build
    fn get_build_deployments(&self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

}

pub trait BuildApiMut {
    type Error;


    /// Convert a docker image and create a new build
    fn convert_app_build(&mut self, body: models::ConvertAppBuildRequest) -> Result<models::Build, Self::Error>;

    /// Create a new build
    fn create_build(&mut self, body: models::CreateBuildRequest) -> Result<models::Build, Self::Error>;

    /// Delete a particular build
    fn delete_build(&mut self, build_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all builds information
    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error>;

    /// Get details of a particular build
    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error>;

    /// Get all deployments of a build
    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error>;

}



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

    fn get_all_builds(&mut self, all_search: Option<String>, docker_image_name: Option<String>, deployed_status: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllBuildsResponse, Self::Error> {
        <T as BuildApi>::get_all_builds(self, all_search, docker_image_name, deployed_status, status, limit, offset, sort_by, )
    }

    fn get_build(&mut self, build_id: uuid::Uuid) -> Result<models::Build, Self::Error> {
        <T as BuildApi>::get_build(self, build_id, )
    }

    fn get_build_deployments(&mut self, build_id: uuid::Uuid, status: Option<String>, all_search: Option<String>, sort_by: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetAllBuildDeploymentsResponse, Self::Error> {
        <T as BuildApi>::get_build_deployments(self, build_id, status, all_search, sort_by, limit, offset, )
    }

}


pub trait CertificateApi {
    type Error;


    /// Retrieve a certificate
    fn get_certificate(&self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

}

pub trait CertificateApiMut {
    type Error;


    /// Retrieve a certificate
    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;
}



impl<T, E> CertificateApiMut for T
where
    T: CertificateApi<Error = E>,
{
    type Error = E;

    fn get_certificate(&mut self, cert_id: uuid::Uuid) -> Result<models::Certificate, Self::Error> {
        <T as CertificateApi>::get_certificate(self, cert_id, )
    }

}


pub trait NodeApi {
    type Error;


    /// Deactivate a particular node
    fn deactivate_node(&self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all nodes information
    fn get_all_nodes(&self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular node
    fn get_node(&self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested node's certificate
    fn get_node_certificate(&self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an node's certificate
    fn get_node_certificate_details(&self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;
}

pub trait NodeApiMut {
    type Error;


    /// Deactivate a particular node
    fn deactivate_node(&mut self, node_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Get all nodes information
    fn get_all_nodes(&mut self, name: Option<String>, description: Option<String>, sgx_version: Option<String>, all_search: Option<String>, status: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllNodesResponse, Self::Error>;

    /// Get details of a particular node
    fn get_node(&mut self, node_id: uuid::Uuid) -> Result<models::Node, Self::Error>;

    /// Get an attested node's certificate
    fn get_node_certificate(&mut self, node_id: uuid::Uuid) -> Result<models::Certificate, Self::Error>;

    /// Get an node's certificate
    fn get_node_certificate_details(&mut self, node_id: uuid::Uuid) -> Result<models::CertificateDetails, Self::Error>;
}



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

}


pub trait SystemApi {
    type Error;


    /// Get Manager Version
    fn get_manager_version(&self) -> Result<models::VersionResponse, Self::Error>;

}

pub trait SystemApiMut {
    type Error;


    /// Get Manager Version
    fn get_manager_version(&mut self) -> Result<models::VersionResponse, Self::Error>;

}



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


    /// Get all the tasks
    fn get_all_tasks(&self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task
    fn get_task(&self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task
    fn get_task_status(&self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task
    fn update_task(&self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;

}

pub trait TaskApiMut {
    type Error;


    /// Get all the tasks
    fn get_all_tasks(&mut self, task_type: Option<String>, status: Option<String>, requester: Option<String>, approver: Option<String>, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>, base_filters: Option<String>) -> Result<models::GetAllTasksResponse, Self::Error>;

    /// Get details of a particular task
    fn get_task(&mut self, task_id: uuid::Uuid) -> Result<models::Task, Self::Error>;

    /// Get status and result of a particular task
    fn get_task_status(&mut self, task_id: uuid::Uuid) -> Result<models::TaskResult, Self::Error>;

    /// Update status of approver and task
    fn update_task(&mut self, task_id: uuid::Uuid, body: models::TaskUpdateRequest) -> Result<models::TaskResult, Self::Error>;

}



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


    /// Convert an application to run in EnclaveOS
    fn convert_app(&self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;

}

pub trait ToolsApiMut {
    type Error;


    /// Convert an application to run in EnclaveOS
    fn convert_app(&mut self, body: models::ConversionRequest) -> Result<models::ConversionResponse, Self::Error>;

}



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


    /// Disable a user using its email
    fn blacklist_user(&self, body: models::UserBlacklistRequest) -> Result<(), Self::Error>;

    /// Change user password
    fn change_password(&self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address
    fn confirm_email(&self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user
    fn create_user(&self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Removed user's association with an account
    fn delete_user_from_account(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user
    fn forgot_password(&self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all users information
    fn get_all_users(&self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user
    fn get_logged_in_user(&self) -> Result<models::User, Self::Error>;

    /// Get details of a particular User
    fn get_user(&self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user
    fn invite_user(&self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations
    fn process_invitations(&self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address
    fn resend_confirm_email(&self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account
    fn resend_invitation(&self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password
    fn reset_password(&self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, role of a user. User with MANAGER access role can only update another user
    fn update_user(&self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user
    fn validate_password_reset_token(&self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

    /// Whitelist a user
    fn whitelist_user(&self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error>;

}

pub trait UsersApiMut {
    type Error;


    /// Disable a user using its email
    fn blacklist_user(&mut self, body: models::UserBlacklistRequest) -> Result<(), Self::Error>;

    /// Change user password
    fn change_password(&mut self, body: models::PasswordChangeRequest) -> Result<(), Self::Error>;

    /// Confirms user's email address
    fn confirm_email(&mut self, body: models::ConfirmEmailRequest) -> Result<models::ConfirmEmailResponse, Self::Error>;

    /// Create a new user
    fn create_user(&mut self, body: models::SignupRequest) -> Result<models::User, Self::Error>;

    /// Removed user's association with an account
    fn delete_user_from_account(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Initiate password reset sequence for a user
    fn forgot_password(&mut self, body: models::ForgotPasswordRequest) -> Result<(), Self::Error>;

    /// Get all users information
    fn get_all_users(&mut self, all_search: Option<String>, limit: Option<i32>, offset: Option<i32>, sort_by: Option<String>) -> Result<models::GetAllUsersResponse, Self::Error>;

    /// Get details of the current logged in user
    fn get_logged_in_user(&mut self) -> Result<models::User, Self::Error>;

    /// Get details of a particular User
    fn get_user(&mut self, user_id: uuid::Uuid) -> Result<models::User, Self::Error>;

    /// Invite a user
    fn invite_user(&mut self, body: models::InviteUserRequest) -> Result<models::User, Self::Error>;

    /// Process a user's pending account invitations
    fn process_invitations(&mut self, body: models::ProcessInviteRequest) -> Result<(), Self::Error>;

    /// Resend email with link to confirm user's email address
    fn resend_confirm_email(&mut self) -> Result<(), Self::Error>;

    /// Resend invite to the user to join a specific account
    fn resend_invitation(&mut self, user_id: uuid::Uuid) -> Result<(), Self::Error>;

    /// Reset a user's password
    fn reset_password(&mut self, user_id: uuid::Uuid, body: models::PasswordResetRequest) -> Result<(), Self::Error>;

    /// Update status, name, role of a user. User with MANAGER access role can only update another user
    fn update_user(&mut self, user_id: uuid::Uuid, body: models::UpdateUserRequest) -> Result<models::User, Self::Error>;

    /// Validates password reset token for the user
    fn validate_password_reset_token(&mut self, user_id: uuid::Uuid, body: models::ValidateTokenRequest) -> Result<models::ValidateTokenResponse, Self::Error>;

    /// Whitelist a user
    fn whitelist_user(&mut self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error>;

}



impl<T, E> UsersApiMut for T
where
    T: UsersApi<Error = E>,
{
    type Error = E;

    fn blacklist_user(&mut self, body: models::UserBlacklistRequest) -> Result<(), Self::Error> {
        <T as UsersApi>::blacklist_user(self, body, )
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

    fn whitelist_user(&mut self, user_id: uuid::Uuid, user_token: Option<String>) -> Result<models::User, Self::Error> {
        <T as UsersApi>::whitelist_user(self, user_id, user_token, )
    }

}


pub trait ZoneApi {
    type Error;



    /// Get zone details
    fn get_zone(&self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get authentication token
    fn get_zone_join_token(&self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones
    fn get_zones(&self) -> Result<Vec<models::Zone>, Self::Error>;

}

pub trait ZoneApiMut {
    type Error;

    /// Get zone details
    fn get_zone(&mut self, zone_id: uuid::Uuid) -> Result<models::Zone, Self::Error>;

    /// Get authentication token
    fn get_zone_join_token(&mut self, zone_id: uuid::Uuid) -> Result<models::ZoneJoinToken, Self::Error>;

    /// Get all zones
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
