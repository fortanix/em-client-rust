/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */



pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

    lazy_static! {
        /// Create Mime objects for the response content types for CreateAccount
        pub static ref CREATE_ACCOUNT_CREATED_ACCOUNT_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAccount
        pub static ref GET_ACCOUNT_PARTICULAR_ACCOUNT_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAccounts
        pub static ref GET_ACCOUNTS_ALL_ACCOUNT_DETAILS_FOR_CURRENT_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateAccount
        pub static ref UPDATE_ACCOUNT_UPDATED_ACCOUNT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetClusterCsr
        pub static ref GET_CLUSTER_CSR_CSR_FOR_THE_PUBLIC_CLIENT_INTERFACE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AddApplication
        pub static ref ADD_APPLICATION_DETAILS_OF_AN_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllApps
        pub static ref GET_ALL_APPS_SEARCH_RESULT_FOR_APP_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetApp
        pub static ref GET_APP_DETAILS_OF_A_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAppCertificate
        pub static ref GET_APP_CERTIFICATE_APP_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAppNodeCertificateDetails
        pub static ref GET_APP_NODE_CERTIFICATE_DETAILS_APP_CERTIFICATE_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateApp
        pub static ref UPDATE_APP_DETAILS_OF_A_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAuditLogs
        pub static ref GET_AUDIT_LOGS_SEARCH_RESULT_FOR_AUDIT_LOGS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticateUser
        pub static ref AUTHENTICATE_USER_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticateUserToken
        pub static ref AUTHENTICATE_USER_TOKEN_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for InitiateOAuth
        pub static ref INITIATE_O_AUTH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for OauthCallback
        pub static ref OAUTH_CALLBACK_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ConvertAppBuild
        pub static ref CONVERT_APP_BUILD_DETAILS_OF_THE_CREATED_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateBuild
        pub static ref CREATE_BUILD_DETAILS_OF_THE_CREATED_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllBuilds
        pub static ref GET_ALL_BUILDS_SEARCH_RESULT_FOR_BUILD_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetBuild
        pub static ref GET_BUILD_DETAILS_OF_A_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetBuildDeployments
        pub static ref GET_BUILD_DEPLOYMENTS_DETAILS_OF_BUILD_DEPLOYMENTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetCertificate
        pub static ref GET_CERTIFICATE_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for NewCertificate
        pub static ref NEW_CERTIFICATE_CERTIFICATE_ISSUANCE_TASK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllNodes
        pub static ref GET_ALL_NODES_SEARCH_RESULT_FOR_NODE_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNode
        pub static ref GET_NODE_DETAILS_OF_A_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodeCertificate
        pub static ref GET_NODE_CERTIFICATE_NODE_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodeCertificateDetails
        pub static ref GET_NODE_CERTIFICATE_DETAILS_CERTIFICATE_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodeCertificateNodeAgent
        pub static ref GET_NODE_CERTIFICATE_NODE_AGENT_NODE_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ProvisionNode
        pub static ref PROVISION_NODE_NODE_CREATION_TASK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetManagerVersion
        pub static ref GET_MANAGER_VERSION_MANAGER_VERSION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllTasks
        pub static ref GET_ALL_TASKS_SEARCH_RESULT_FOR_TASK_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetTask
        pub static ref GET_TASK_TASK_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetTaskStatus
        pub static ref GET_TASK_STATUS_TASK_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateTask
        pub static ref UPDATE_TASK_UPDATED_TASK_STATUS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ConvertApp
        pub static ref CONVERT_APP_REGISTRY_AND_IMAGE_NAME_FOR_THE_OUTPUT_CONTAINER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ConfirmEmail
        pub static ref CONFIRM_EMAIL_CONFIRMS_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateUser
        pub static ref CREATE_USER_CREATED_USER_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllUsers
        pub static ref GET_ALL_USERS_SEARCH_RESULT_FOR_USERS_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetLoggedInUser
        pub static ref GET_LOGGED_IN_USER_USER_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetUser
        pub static ref GET_USER_USER_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for InviteUser
        pub static ref INVITE_USER_INVITED_USER_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateUser
        pub static ref UPDATE_USER_UPDATED_USER_STATUS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ValidatePasswordResetToken
        pub static ref VALIDATE_PASSWORD_RESET_TOKEN_VALIDATES_PASSWORD_RESET_TOKEN_FOR_THE_USER_RESPONSE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for WhitelistUser
        pub static ref WHITELIST_USER_USER_WAS_WHITELISTED_SUCCESSFULLY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateZone
        pub static ref CREATE_ZONE_DETAILS_OF_THE_ZONE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetZone
        pub static ref GET_ZONE_DETAILS_OF_THE_ZONE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetZoneJoinToken
        pub static ref GET_ZONE_JOIN_TOKEN_JOIN_TOKEN_FOR_THE_ZONE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetZones
        pub static ref GET_ZONES_DETAILS_OF_ALL_ZONES: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

    lazy_static! {
        /// Create Mime objects for the request content types for CreateAccount
        pub static ref CREATE_ACCOUNT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateAccount
        pub static ref UPDATE_ACCOUNT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ClusterConfigure
        pub static ref CLUSTER_CONFIGURE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateCluster
        pub static ref CREATE_CLUSTER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for GetClusterCsr
        pub static ref GET_CLUSTER_CSR: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for JoinCluster
        pub static ref JOIN_CLUSTER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ProvisionNodeBackend
        pub static ref PROVISION_NODE_BACKEND: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AddApplication
        pub static ref ADD_APPLICATION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AppHeartbeat
        pub static ref APP_HEARTBEAT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateApp
        pub static ref UPDATE_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for InitiateOAuth
        pub static ref INITIATE_O_AUTH: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ConvertAppBuild
        pub static ref CONVERT_APP_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateBuild
        pub static ref CREATE_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for NewCertificate
        pub static ref NEW_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ProvisionNode
        pub static ref PROVISION_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateTask
        pub static ref UPDATE_TASK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ConvertApp
        pub static ref CONVERT_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for BlacklistUser
        pub static ref BLACKLIST_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ChangePassword
        pub static ref CHANGE_PASSWORD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ConfirmEmail
        pub static ref CONFIRM_EMAIL: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateUser
        pub static ref CREATE_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ForgotPassword
        pub static ref FORGOT_PASSWORD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for InviteUser
        pub static ref INVITE_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ProcessInvitations
        pub static ref PROCESS_INVITATIONS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ResetPassword
        pub static ref RESET_PASSWORD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateUser
        pub static ref UPDATE_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ValidatePasswordResetToken
        pub static ref VALIDATE_PASSWORD_RESET_TOKEN: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateZone
        pub static ref CREATE_ZONE: Mime = "application/json".parse().unwrap();
    }

}
