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
        pub static ref CREATE_ACCOUNT_THE_DETAILS_OF_THE_ACCOUNT_CREATED: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAccount
        pub static ref GET_ACCOUNT_PARTICULAR_ACCOUNT_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAccounts
        pub static ref GET_ACCOUNTS_ALL_ACCOUNT_DETAILS_FOR_THE_CURRENT_USER: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateAccount
        pub static ref UPDATE_ACCOUNT_UPDATED_ACCOUNT: Mime = "application/json".parse().unwrap();
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
        pub static ref GET_APP_DETAILS_OF_AN_APP: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the response content types for GetAppsUniqueLabels
        pub static ref GET_APPS_UNIQUE_LABELS_APPLICATIONS_LABELS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateApp
        pub static ref UPDATE_APP_DETAILS_OF_AN_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateApplicationConfig
        pub static ref CREATE_APPLICATION_CONFIG_DETAILS_OF_AN_APP_CONFIG: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllApplicationConfigs
        pub static ref GET_ALL_APPLICATION_CONFIGS_SEARCH_RESULT_FOR_APP_CONFIG_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetApplicationConfig
        pub static ref GET_APPLICATION_CONFIG_DETAILS_OF_AN_APP_CONFIG: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetRuntimeApplicationConfig
        pub static ref GET_RUNTIME_APPLICATION_CONFIG_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetSpecificRuntimeApplicationConfig
        pub static ref GET_SPECIFIC_RUNTIME_APPLICATION_CONFIG_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateApplicationConfig
        pub static ref UPDATE_APPLICATION_CONFIG_DETAILS_OF_AN_APP_CONFIG: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ApproveApprovalRequest
        pub static ref APPROVE_APPROVAL_REQUEST_DETAILS_ABOUT_THE_SPECIFIED_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateApprovalRequest
        pub static ref CREATE_APPROVAL_REQUEST_A_NEWLY_CREATED_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for DenyApprovalRequest
        pub static ref DENY_APPROVAL_REQUEST_DETAILS_ABOUT_THE_SPECIFIED_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllApprovalRequests
        pub static ref GET_ALL_APPROVAL_REQUESTS_SEARCH_RESULT_FOR_APPROVAL_REQUEST_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetApprovalRequest
        pub static ref GET_APPROVAL_REQUEST_DETAILS_ABOUT_THE_SPECIFIED_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetApprovalRequestResult
        pub static ref GET_APPROVAL_REQUEST_RESULT_DETAILS_ABOUT_THE_SPECIFIED_APPROVAL_REQUEST_RESULT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for AuthenticateUser
        pub static ref AUTHENTICATE_USER_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ConvertAppBuild
        pub static ref CONVERT_APP_BUILD_DETAILS_OF_THE_CREATED_IMAGE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateBuild
        pub static ref CREATE_BUILD_DETAILS_OF_THE_CREATED_IMAGE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllBuilds
        pub static ref GET_ALL_BUILDS_SEARCH_RESULT_FOR_IMAGE_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetBuild
        pub static ref GET_BUILD_DETAILS_OF_AN_IMAGE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetBuildDeployments
        pub static ref GET_BUILD_DEPLOYMENTS_DETAILS_OF_IMAGE_DEPLOYMENTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateBuild
        pub static ref UPDATE_BUILD_DETAILS_OF_A_COMPUTE_NODE: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the response content types for CreateDataset
        pub static ref CREATE_DATASET_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllDatasets
        pub static ref GET_ALL_DATASETS_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetDataset
        pub static ref GET_DATASET_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateDataset
        pub static ref UPDATE_DATASET_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllNodes
        pub static ref GET_ALL_NODES_SEARCH_RESULT_FOR_COMPUTE_NODE_OBJECTS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNode
        pub static ref GET_NODE_DETAILS_OF_A_COMPUTE_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodeCertificate
        pub static ref GET_NODE_CERTIFICATE_COMPUTE_NODE_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodeCertificateDetails
        pub static ref GET_NODE_CERTIFICATE_DETAILS_CERTIFICATE_DETAILS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetNodesUniqueLabels
        pub static ref GET_NODES_UNIQUE_LABELS_NODES_LABELS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for ProvisionNode
        pub static ref PROVISION_NODE_COMPUTE_NODE_CREATION_TASK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateNode
        pub static ref UPDATE_NODE_DETAILS_OF_A_COMPUTE_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateNodeStatus
        pub static ref UPDATE_NODE_STATUS_CONFIGURATION_INFORMATION_FOR_THE_NODE_AGENT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateRegistry
        pub static ref CREATE_REGISTRY_REGISTRY_INFORMATION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllRegistries
        pub static ref GET_ALL_REGISTRIES_GET_DETAILS_OF_ALL_REGISTRY_IN_THE_ACCOUNT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetRegistry
        pub static ref GET_REGISTRY_DETAILS_OF_A_REGISTRY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetRegistryForApp
        pub static ref GET_REGISTRY_FOR_APP_REGISTRY_DETAILS_THAT_WILL_BE_USED_FROM_SAVED_REGISTRY_CREDENTIALS_FOR_THE_PARTICULAR_APP_IMAGES: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetRegistryForImage
        pub static ref GET_REGISTRY_FOR_IMAGE_REGISTRY_DETAIL_THAT_WILL_BE_USED_FROM_SAVED_REGISTRY_CREDENTIALS_FOR_THE_PARTICULAR_IMAGE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateRegistry
        pub static ref UPDATE_REGISTRY_REGISTRY_INFORMATION: Mime = "application/json".parse().unwrap();
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
        pub static ref GET_ALL_USERS_SEARCH_RESULT_FOR_USER: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the response content types for CreateWorkflowGraph
        pub static ref CREATE_WORKFLOW_GRAPH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllWorkflowGraphs
        pub static ref GET_ALL_WORKFLOW_GRAPHS_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetWorkflowGraph
        pub static ref GET_WORKFLOW_GRAPH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateWorkflowGraph
        pub static ref UPDATE_WORKFLOW_GRAPH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for CreateFinalWorkflowGraph
        pub static ref CREATE_FINAL_WORKFLOW_GRAPH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAllFinalWorkflowGraphs
        pub static ref GET_ALL_FINAL_WORKFLOW_GRAPHS_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetFinalWorkflowGraph
        pub static ref GET_FINAL_WORKFLOW_GRAPH_: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetFullFinalWorkflowGraph
        pub static ref GET_FULL_FINAL_WORKFLOW_GRAPH_THE_WORKFLOW_WITH_ALL_CONTAINED_GRAPH_VERSIONS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for UpdateFinalWorkflowGraph
        pub static ref UPDATE_FINAL_WORKFLOW_GRAPH_THE_DATA_FOR_THE_CREATED_VERSION_WITHIN_THE_WORKFLOW: Mime = "application/json".parse().unwrap();
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
        pub static ref GET_ZONES_DETAILS_OF_ALL_THE_ZONES: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the request content types for AddApplication
        pub static ref ADD_APPLICATION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateApp
        pub static ref UPDATE_APP: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateApplicationConfig
        pub static ref CREATE_APPLICATION_CONFIG: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateApplicationConfig
        pub static ref UPDATE_APPLICATION_CONFIG: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ApproveApprovalRequest
        pub static ref APPROVE_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateApprovalRequest
        pub static ref CREATE_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for DenyApprovalRequest
        pub static ref DENY_APPROVAL_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for AuthenticateUser
        pub static ref AUTHENTICATE_USER: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the request content types for UpdateBuild
        pub static ref UPDATE_BUILD: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for NewCertificate
        pub static ref NEW_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateDataset
        pub static ref CREATE_DATASET: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateDataset
        pub static ref UPDATE_DATASET: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for ProvisionNode
        pub static ref PROVISION_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateNode
        pub static ref UPDATE_NODE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateNodeStatus
        pub static ref UPDATE_NODE_STATUS: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateRegistry
        pub static ref CREATE_REGISTRY: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateRegistry
        pub static ref UPDATE_REGISTRY: Mime = "application/json".parse().unwrap();
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
        /// Create Mime objects for the request content types for CreateWorkflowGraph
        pub static ref CREATE_WORKFLOW_GRAPH: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateWorkflowGraph
        pub static ref UPDATE_WORKFLOW_GRAPH: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for CreateFinalWorkflowGraph
        pub static ref CREATE_FINAL_WORKFLOW_GRAPH: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for UpdateFinalWorkflowGraph
        pub static ref UPDATE_FINAL_WORKFLOW_GRAPH: Mime = "application/json".parse().unwrap();
    }

}
