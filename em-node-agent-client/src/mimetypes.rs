/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

    lazy_static! {
        /// Create Mime objects for the response content types for AppHeartbeat
        pub static ref APP_HEARTBEAT_DETAILS_OF_THE_APP_HEARTBEAT_REQUEST: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetIssueCertificateResponse
        pub static ref GET_ISSUE_CERTIFICATE_RESPONSE_TARGET_INFO_FOR_NODE_PROVISIONING_ENCLAVE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for IssueCertificate
        pub static ref ISSUE_CERTIFICATE_DETAILS_OF_THE_CERTIFICATE_ISSUANCE_TASK: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetFortanixAttestation
        pub static ref GET_FORTANIX_ATTESTATION_FORTANIX_ATTESTATION_BY_ENCLAVE_MANAGER_FOR_THE_APPLICATION: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetTargetInfo
        pub static ref GET_TARGET_INFO_TARGET_INFO_FOR_NODE_PROVISIONING_ENCLAVE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the response content types for GetAgentVersion
        pub static ref GET_AGENT_VERSION_AGENT_VERSION: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

    lazy_static! {
        /// Create Mime objects for the request content types for AppHeartbeat
        pub static ref APP_HEARTBEAT: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for IssueCertificate
        pub static ref ISSUE_CERTIFICATE: Mime = "application/json".parse().unwrap();
    }

    lazy_static! {
        /// Create Mime objects for the request content types for GetFortanixAttestation
        pub static ref GET_FORTANIX_ATTESTATION: Mime = "application/json".parse().unwrap();
    }

}
