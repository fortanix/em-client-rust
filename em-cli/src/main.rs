/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
   Application provides a mini-cli to 'EM SaaS Manager' to be used for create-build flows in jenkins automation.
*/
#[macro_use]
extern crate lazy_static;

use crypto_hash::{Algorithm, Hasher};
use em_client::{models, Api, Client};
use hyper::header::{Authorization, Basic, Bearer};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use native_tls::{Certificate, TlsConnector};
use serde::{Deserialize, Serialize};
use serde_json;
use sgx_isa::Sigstruct;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut hex = String::new();
    for byte in bytes {
        write!(hex, "{:02x}", byte).unwrap();
    }
    hex
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    url: String,
    token: String,
    root_ca_str: Option<String>,
}

lazy_static! {
    static ref LOGIN_FILE: String = format!(
        "{}/.em-login.json",
        env::var("HOME").expect("Failed getting HOME environment variable")
    );
    static ref EDP_NAME: String = "EDP ENCLAVE APP - 5f42a1ee280cf158490a8".to_string();
    static ref MEM_SIZE: i64 = 1024;
    static ref THREADS: i32 = 128;
}

/// Store login data so we have persistence between commands.
fn store_login_data(
    url_str: &str,
    token_str: &str,
    root_ca_str: Option<String>,
) -> Result<(), String> {
    let mut file = File::create(&*LOGIN_FILE)
        .map_err(|e| format!("Failed storing login token in {}, {}", &*LOGIN_FILE, e))?;
    let data = LoginData {
        url: url_str.to_string(),
        token: token_str.to_string(),
        root_ca_str: root_ca_str,
    };
    file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();
    Ok(())
}

/// Reads stored bearer token, url and TLS security info, used for any non-login command.
fn get_login_data() -> Result<LoginData, String> {
    let mut f = File::open(&*LOGIN_FILE)
        .map_err(|e| format!("Failed opening login-token file {}, {}", &*LOGIN_FILE, e))?;

    let mut data = String::new();
    f.read_to_string(&mut data)
        .map_err(|e| format!("Failed reading login-token file {}, {}", &*LOGIN_FILE, e))?;

    Ok(serde_json::from_str(&data).unwrap())
}

/// Get an API Client using token cached on filesystem
fn get_cached_client() -> Result<Client, String> {
    let data = get_login_data()?;
    Ok(get_client(&data.url, Some(data.token), &data.root_ca_str))
}

/// Construct an API client with given parameters
fn get_client(url: &str, token: Option<String>, root_ca_str: &Option<String>) -> Client {
    let ssl = match root_ca_str {
        Some(str) => {
            let cert = Certificate::from_pem(str.as_bytes()).unwrap();

            let mut connector = TlsConnector::builder();
            connector.add_root_certificate(cert);
            NativeTlsClient::from(connector.build().unwrap())
        }
        None => NativeTlsClient::new().unwrap(),
    };

    let https_connector = HttpsConnector::new(ssl);
    let mut client = Client::try_new_with_connector(url, Some("https"), https_connector).unwrap();

    if let Some(token) = token {
        client.headers().set(Authorization(Bearer { token: token }));
    }
    client
}

/// Wrapper over username/password login.
fn login(client: &mut Client, username: &str, password: &str) -> Result<String, String> {
    client.headers().set(Authorization(Basic {
        username: username.to_string(),
        password: Some(password.to_string()),
    }));

    let auth = client
        .authenticate_user()
        .map_err(|e| format!("Authenticate user call failed, {}", e.0))?;
    Ok(auth.access_token.unwrap())
}

/// Construct a build request from a provided SIGSTRUCT.
///
/// Quick way to get a sigstruct for testing:
///     openssl genrsa -3 3072 > private.pem
///     rustc +nightly --target x86_64-fortanix-unknown-sgx ./test.rs
///     ftxsgx-elf2sgxs test --heap-size 0x20000 --stack-size 0x20000 --threads 1 --debug
///     sgxs-sign ./test.sgxs sigstruct.txt --key ./private.pem
///
fn parse_sigstruct(path: &str) -> Result<models::CreateBuildRequest, String> {
    let sigstruct =
        Sigstruct::try_copy_from(&std::fs::read(path).map_err(|e| format!("{}", e))?).unwrap();
    let mrenclave = to_hex(&sigstruct.enclavehash);

    let mut hasher = Hasher::new(Algorithm::SHA256);
    hasher
        .write_all(&sigstruct.modulus)
        .map_err(|e| format!("Failed calculating mrsigner: {}", e))?;
    let mrsigner = to_hex(&hasher.finish());

    let result = models::CreateBuildRequest {
        docker_info: None,
        mrenclave: mrenclave,
        mrsigner: mrsigner,
        isvprodid: sigstruct.isvprodid as i32,
        isvsvn: sigstruct.isvsvn as i32,
        app_id: None,
        app_name: None,
        mem_size: None,
        threads: None,
        advanced_settings: None,
    };

    Ok(result)
}

/// List of all supported commands. Names match those from openAPI definitions, parameters are only the mandatory ones at the moment.
fn args_desc<'a>() -> clap::App<'a, 'a> {
    use clap::{AppSettings, Arg, SubCommand};

    clap::App::new("em-build-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("user")
                .subcommand(
                    SubCommand::with_name("login")
                        .arg(
                            Arg::with_name("url")
                                .required(true)
                                .help("URL to login to."),
                        )
                        .arg(
                            Arg::with_name("username")
                                .required(true)
                                .help("Username for authentication if using Basic authentication"),
                        )
                        .arg(
                            Arg::with_name("password")
                                .required(true)
                                .help("Password for authentication if using Basic authentication"),
                        )
                        .arg(
                            Arg::with_name("add-root-ca")
                                .takes_value(true)
                                .long("--add-root-ca")
                                .help("Root CA certificate to add for verification"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("create")
                        .arg(Arg::with_name("url").required(true).help("Manager URL."))
                        .arg(Arg::with_name("username").required(true).help("Username"))
                        .arg(Arg::with_name("password").required(true).help("Password"))
                        .arg(
                            Arg::with_name("add-root-ca")
                                .takes_value(true)
                                .long("--add-root-ca")
                                .help("Root CA certificate to add for verification"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("account")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(
                    SubCommand::with_name("create").arg(
                        Arg::with_name("name").required(true).help(
                            "Name of the account. Accounts must be unique within an EM instance",
                        ),
                    ),
                )
                .subcommand(
                    SubCommand::with_name("select").arg(
                        Arg::with_name("account-id")
                            .required(true)
                            .help("Account UUID to select"),
                    ),
                ),
        )
        .subcommand(
            SubCommand::with_name("app")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(
                    SubCommand::with_name("create")
                        .arg(
                            Arg::with_name("name")
                                .required(true)
                                .help("Name of the app"),
                        )
                        .arg(Arg::with_name("isvprodid").required(true).help("ISVPRODID"))
                        .arg(Arg::with_name("isvsvn").required(true).help("ISVSVN"))
                        .arg(
                            Arg::with_name("allowed-domains")
                                .help("Comma separated allowed domains"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("update")
                        .arg(Arg::with_name("app-id").required(true).help("App uuid"))
                        .arg(
                            Arg::with_name("allowed-domains")
                                .takes_value(true)
                                .long("--allowed-domains")
                                .help("Comma separated allowed domains"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(
                    SubCommand::with_name("create")
                        .arg(Arg::with_name("app-id").required(true).help("App ID"))
                        .arg(
                            Arg::with_name("sigstruct-path")
                                .required(true)
                                .help("path to sigstruct to use for build"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .arg(Arg::with_name("build-id").required(true).help("build uuid")),
                )
                .subcommand(
                    SubCommand::with_name("parse-sigstruct").arg(
                        Arg::with_name("path")
                            .required(true)
                            .help("Path to sigstruct file"),
                    ),
                ),
        )
        .subcommand(
            SubCommand::with_name("task")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(
                    SubCommand::with_name("get")
                        .arg(Arg::with_name("task-id").required(true).help("task uuid")),
                )
                .subcommand(
                    SubCommand::with_name("update")
                        .arg(Arg::with_name("task-id").required(true).help("task uuid"))
                        .arg(
                            Arg::with_name("status")
                                .required(true)
                                .help("approved or denied"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("zone")
                .subcommand(SubCommand::with_name("list"))
                .subcommand(
                    SubCommand::with_name("get")
                        .arg(Arg::with_name("zone-id").required(true).help("zone uuid")),
                )
                .subcommand(
                    SubCommand::with_name("get-join-token")
                        .arg(Arg::with_name("zone-id").required(true).help("zone uuid")),
                ),
        )
        .subcommand(SubCommand::with_name("node").subcommand(SubCommand::with_name("list")))
}

/// Implement a CLI using node-manager openAPI definitions to be used in external scripts and logic.
/// Script logic can be implemented in other rust apps as well.
fn main() -> Result<(), String> {
    let matches = args_desc().get_matches();

    // Items below can be auto-generated with a bit of effort, they just read parameters, call API and print out result nicely.
    match matches.subcommand() {
        ("user", Some(matches)) => match matches.subcommand() {
            ("create", Some(param)) => {
                let url = param.value_of("url").unwrap();
                let username = param.value_of("username").unwrap().to_string();
                let password = param.value_of("password").unwrap().to_string();

                let root_ca_str: Option<String> = param.value_of("add-root-ca").map(|path| {
                    std::fs::read_to_string(path).expect("Failed reading root-ca certificate")
                });

                let client = get_client(&url, None, &root_ca_str);
                let request = models::SignupRequest {
                    user_email: username,
                    user_password: password,
                    first_name: None,
                    last_name: None,
                };
                let result = client
                    .create_user(request)
                    .map_err(|e| format!("create-user failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("login", Some(param)) => {
                let url = param.value_of("url").unwrap();
                let username = param.value_of("username").unwrap();
                let password = param.value_of("password").unwrap();

                let root_ca_str: Option<String> = param.value_of("add-root-ca").map(|path| {
                    std::fs::read_to_string(path).expect("Failed reading root-ca certificate")
                });

                let mut client = get_client(&url, None, &root_ca_str);
                let token = login(&mut client, username, password)?;

                store_login_data(url, &token, root_ca_str)?;
                println!("Logged in.");
            }
            _ => (),
        },
        ("account", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => {
                let client = get_cached_client()?;
                let result = client
                    .get_accounts()
                    .map_err(|e| format!("get-accounts failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("create", Some(param)) => {
                let client = get_cached_client()?;
                let request = models::AccountRequest {
                    name: param.value_of("name").unwrap().to_string(),
                    custom_logo: None,
                };
                let result = client
                    .create_account(request)
                    .map_err(|e| format!("create-account failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("select", Some(param)) => {
                let account_id = param.value_of("account-id").unwrap();
                let account_uuid = Uuid::parse_str(account_id).map_err(|e| {
                    format!(
                        "select-account UUID parsing failed for \"{}\": {}",
                        account_id, e
                    )
                })?;

                let client = get_cached_client()?;
                client
                    .select_account(account_uuid)
                    .map_err(|e| format!("select-account failed: {}", e))?;

                println!("Account selected.");
            }
            _ => (),
        },
        ("app", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(_)) => {
                    let client = get_cached_client()?;
                    let result = client
                        .get_all_apps(None, None, None, None, None, None)
                        .map_err(|e| format!("get_all_apps failed: {}", e))?;
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                ("update", Some(param)) => {
                    let client = get_cached_client()?;

                    let app_id = param.value_of("app-id").unwrap();
                    let app_uuid = Uuid::parse_str(app_id).map_err(|e| {
                        format!("update-app UUID parsing failed for \"{}\": {}", app_id, e)
                    })?;

                    let domains = Some(
                        param
                            .value_of("allowed-domains")
                            .unwrap()
                            .split(",")
                            .map(|s| s.to_string())
                            .collect(),
                    );

                    let request = models::AppBodyUpdateRequest {
                        description: None,
                        input_image_name: None,
                        output_image_name: None,
                        isvsvn: None,
                        mem_size: None,
                        threads: None,
                        allowed_domains: domains,
                        advanced_settings: None,
                    };

                    let result = client
                        .update_app(app_uuid, request)
                        .map_err(|e| format!("create-account failed: {}", e))?;
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                ("create", Some(param)) => {
                    let client = get_cached_client()?;

                    let mut domains = None;
                    if let Some(allowed_domains) = param.value_of("allowed-domains") {
                        domains = Some(allowed_domains.split(",").map(|s| s.to_string()).collect());
                    }

                    // This is kept at a minimum for now as optional parameters are not used.
                    //
                    // To extend either make all parameters as CLI arguments and have a huge command
                    // or do the rest via updates or build the request command and do a commit at the end
                    let request = models::AppRequest {
                        name: param.value_of("name").unwrap().to_string(),
                        description: None,
                        input_image_name: EDP_NAME.clone(),
                        output_image_name: EDP_NAME.clone(),
                        isvprodid: param.value_of("isvprodid").unwrap().parse::<i32>().unwrap(),
                        isvsvn: param.value_of("isvsvn").unwrap().parse::<i32>().unwrap(),
                        mem_size: *MEM_SIZE,
                        threads: *THREADS,
                        allowed_domains: domains,
                        advanced_settings: None,
                    };

                    let result = client
                        .add_application(request)
                        .map_err(|e| format!("add-application failed: {}", e))?;
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                _ => (),
            }
        }
        ("build", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => {
                let client = get_cached_client()?;
                let result = client
                    .get_all_builds(None, None, None, None, None, None, None)
                    .map_err(|e| format!("get-all-builds failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("parse-sigstruct", Some(param)) => {
                let path = param.value_of("path").unwrap();
                let result = parse_sigstruct(path)?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("delete", Some(param)) => {
                let build_id = param.value_of("build-id").unwrap();
                let build_uuid = Uuid::parse_str(build_id).map_err(|e| {
                    format!(
                        "delete-build UUID parsing failed for \"{}\": {}",
                        build_id, e
                    )
                })?;

                let client = get_cached_client()?;
                client
                    .delete_build(build_uuid)
                    .map_err(|e| format!("delete-build failed: {}", e))?;
                println!("Delete succesful");
            }
            ("create", Some(param)) => {
                let path = param.value_of("sigstruct-path").unwrap();

                let client = get_cached_client()?;
                let mut request = parse_sigstruct(path)?;

                if let Some(app_id) = param.value_of("app-id") {
                    let app_uuid = Uuid::parse_str(app_id).map_err(|e| {
                        format!("create-build UUID parsing failed for \"{}\": {}", app_id, e)
                    })?;
                    request.app_id = Some(app_uuid);
                }

                let result = client
                    .create_build(request)
                    .map_err(|e| format!("create-build failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            _ => (),
        },
        ("task", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => {
                let client = get_cached_client()?;
                let result = client
                    .get_all_tasks(None, None, None, None, None, None, None, None, None)
                    .map_err(|e| format!("get-all-tasks failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("get", Some(param)) => {
                let task_id = param.value_of("task-id").unwrap();
                let task_uuid = Uuid::parse_str(task_id).map_err(|e| {
                    format!("get-task UUID parsing failed for \"{}\": {}", task_id, e)
                })?;

                let client = get_cached_client()?;
                let result = client
                    .get_task(task_uuid)
                    .map_err(|e| format!("get-task failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("update", Some(param)) => {
                let task_id = param.value_of("task-id").unwrap();
                let task_uuid = Uuid::parse_str(task_id).map_err(|e| {
                    format!("update-task UUID parsing failed for \"{}\": {}", task_id, e)
                })?;

                let status = param.value_of("status").unwrap().to_string();
                let approval_status =
                    models::ApprovalStatus::from_str(status.to_uppercase().as_str()).map_err(
                        |_| format!("expected approved or denied as parameter, got: {}", status),
                    )?;
                let client = get_cached_client()?;

                let request = models::TaskUpdateRequest {
                    status: approval_status,
                };

                let result = client
                    .update_task(task_uuid, request)
                    .map_err(|e| format!("update-task failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            _ => (),
        },
        ("zone", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => {
                let client = get_cached_client()?;
                let result = client
                    .get_zones()
                    .map_err(|e| format!("get-zones failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("get", Some(param)) => {
                let zone_id = param.value_of("zone-id").unwrap();
                let zone_uuid = Uuid::parse_str(zone_id).map_err(|e| {
                    format!("get-zone UUID parsing failed for \"{}\": {}", zone_id, e)
                })?;

                let client = get_cached_client()?;
                let result = client
                    .get_zone(zone_uuid)
                    .map_err(|e| format!("get-zone failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            ("get-join-token", Some(param)) => {
                let zone_id = param.value_of("zone-id").unwrap();
                let zone_uuid = Uuid::parse_str(zone_id).map_err(|e| {
                    format!(
                        "get-zone-join-token UUID parsing failed for \"{}\": {}",
                        zone_id, e
                    )
                })?;

                let client = get_cached_client()?;
                let result = client
                    .get_zone_join_token(zone_uuid)
                    .map_err(|e| format!("get-zone-join-token failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            _ => (),
        },
        ("node", Some(matches)) => match matches.subcommand() {
            ("list", Some(_)) => {
                let client = get_cached_client()?;
                let result = client
                    .get_all_nodes(None, None, None, None, None, None, None, None)
                    .map_err(|e| format!("get_all_nodes failed: {}", e))?;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            _ => (),
        },
        _ => (),
    }
    Ok(())
}
