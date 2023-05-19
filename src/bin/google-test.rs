#[macro_use]
extern crate log;

use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::Command::new("google-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the Google EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("acct-ga-1")
                .short('3')
                .long("account_ga_1")
                .takes_value(true)
                .required(true)
                .help("Config file for the first General Availability account"),
        )
        .arg(
            clap::Arg::new("acct-ga-2")
                .short('4')
                .long("account_ga_2")
                .takes_value(true)
                .required(true)
                .help("Config file for the second General Availability account"),
        )
        .arg(
            clap::Arg::new("acct-sunrise")
                .short('1')
                .long("account_sunrise")
                .takes_value(true)
                .required(true)
                .help("Config file for the sunrise account"),
        )
        .arg(
            clap::Arg::new("registrar_name")
                .short('r')
                .long("registrar_name")
                .takes_value(true)
                .required(true)
                .help("Registrar account name"),
        )
        .arg(
            clap::Arg::new("domain")
                .short('d')
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("Domain to use for testing (without TLD)"),
        )
        .arg(
            clap::Arg::new("domain_idn")
                .short('i')
                .long("idn_domain")
                .takes_value(true)
                .required(true)
                .help("IDN Domain to use for testing (without TLD)"),
        )
        .arg(
            clap::Arg::new("tmcnis_user")
                .short('u')
                .long("tmcnis_user")
                .takes_value(true)
                .required(true)
                .help("Username for trademark CNIS"),
        )
        .arg(
            clap::Arg::new("tmcnis_pass")
                .short('p')
                .long("tmcnis_password")
                .takes_value(true)
                .required(true)
                .help("Password for trademark CNIS"),
        )
        .arg(
            clap::Arg::new("hsm_conf")
                .short('h')
                .long("hsm-conf")
                .takes_value(true)
                .help("Where to read the HSM config file from"),
        )
        .arg(
            clap::Arg::new("log")
                .long("log")
                .takes_value(true)
                .default_value("./log/")
                .help("Directory to write command logs to"),
        )
        .get_matches();

    let pkcs11_engine = epp_proxy::setup_pkcs11_engine(matches.value_of("hsm_conf")).await;
    let domain = matches.value_of("domain").unwrap();
    let domain_idn = matches.value_of("domain_idn").unwrap();
    let registrar_name = matches.value_of("registrar_name").unwrap();
    let tmcnis_user = matches.value_of("tmcnis_user").unwrap();
    let tmcnis_pass = matches.value_of("tmcnis_pass").unwrap();

    let ga_tld = format!("{}-ga", registrar_name);
    let sunrise_tld = format!("{}-sunrise", registrar_name);
    let ga_domain = format!("{}.{}", domain, ga_tld);
    let ga_domain_ns1 = format!("ns1.{}", ga_domain);
    let ga_domain_ns2 = format!("ns2.{}", ga_domain);
    let ga_domain_idn = format!("{}.{}", domain_idn, ga_tld);
    let ga_domain_claims = format!("test-and-validate.{}", ga_tld);
    let ga_domain_premium = format!("rich.{}", ga_tld);

    let log_dir_path: &std::path::Path = matches.value_of("log").unwrap().as_ref();
    match std::fs::create_dir_all(&log_dir_path) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory: {}", e);
            return;
        }
    }

    let conf_file_ga_1_path = matches.value_of("acct-ga-1").unwrap();
    let conf_file_ga_2_path = matches.value_of("acct-ga-2").unwrap();
    let conf_file_sunrise_path = matches.value_of("acct-sunrise").unwrap();

    let conf_file_ga_1 = match std::fs::File::open(conf_file_ga_1_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_ga_1_path, e);
            return;
        }
    };
    let conf_file_ga_2 = match std::fs::File::open(conf_file_ga_2_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_ga_2_path, e);
            return;
        }
    };
    let conf_file_sunrise = match std::fs::File::open(conf_file_sunrise_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_sunrise_path, e);
            return;
        }
    };

    let conf_ga_1: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file_ga_1) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_ga_1_path, e);
            return;
        }
    };
    let conf_ga_2: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file_ga_2) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_ga_1_path, e);
            return;
        }
    };
    let conf_sunrise: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file_sunrise) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_ga_1_path, e);
            return;
        }
    };

    let log_dir_ga_1 = log_dir_path.join(&conf_ga_1.id);
    match std::fs::create_dir_all(&log_dir_ga_1) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory for {}: {}", conf_ga_1.id, e);
            return;
        }
    }
    let log_dir_ga_2 = log_dir_path.join(&conf_ga_2.id);
    match std::fs::create_dir_all(&log_dir_ga_2) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory for {}: {}", conf_ga_2.id, e);
            return;
        }
    }
    let log_dir_sunrise = log_dir_path.join(&conf_sunrise.id);
    match std::fs::create_dir_all(&log_dir_sunrise) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory for {}: {}", conf_sunrise.id, e);
            return;
        }
    }

    let epp_client_ga_1 =
        epp_proxy::create_client(log_dir_ga_1, &conf_ga_1, &pkcs11_engine, true).await;
    let epp_client_ga_2 =
        epp_proxy::create_client(log_dir_ga_2, &conf_ga_2, &pkcs11_engine, true).await;
    let epp_client_sunrise =
        epp_proxy::create_client(log_dir_sunrise, &conf_sunrise, &pkcs11_engine, true).await;

    // 2.1 - Login
    let (mut cmd_tx_ga_1, mut ready_rx_ga_1) = epp_client_ga_1.start();
    let (mut cmd_tx_ga_2, mut ready_rx_ga_2) = epp_client_ga_2.start();

    info!("Awaiting client GA 1 to become ready...");
    let login_trans_id = ready_rx_ga_1.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);
    info!("Awaiting client GA 2 to become ready...");
    let login_trans_id = ready_rx_ga_2.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // Misc setup
    info!("Setting up contacts");

    info!("Finding available contact ID");
    let mut contact_id_i = 1;
    let contact_id = loop {
        let contact_id = format!("STACLAR-{}", contact_id_i);
        let res = epp_proxy::client::contact::check(&contact_id, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break contact_id;
        } else {
            contact_id_i += 1;
        }
    };

    info!("Creating contact");
    epp_proxy::client::contact::create(
        &contact_id,
        epp_proxy::client::contact::NewContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "Boaty McBoat Face".to_string(),
                organisation: None,
                streets: vec!["10 Downing St".to_string()],
                city: "London".to_string(),
                province: Some("England".to_string()),
                postal_code: Some("SW1A 2AA".to_string()),
                country_code: "GB".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            internationalised_address: None,
            phone: Some(epp_proxy::client::Phone {
                number: "+44.1818118181".to_string(),
                extension: None,
            }),
            fax: None,
            email: "test@example.com".to_string(),
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: "test_auth1".to_string(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    info!("Creating nameservers");
    let res = epp_proxy::client::host::check("ns1.as207960.net", &mut cmd_tx_ga_1)
        .await
        .unwrap();
    if res.response.avail {
        epp_proxy::client::host::create("ns1.as207960.net", vec![], None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
    }
    let res = epp_proxy::client::host::check("ns2.as207960.net", &mut cmd_tx_ga_1)
        .await
        .unwrap();
    if res.response.avail {
        epp_proxy::client::host::create("ns2.as207960.net", vec![], None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
    }

    // 2.2.2 - Perform a check to see that an ASCII domain label is available.
    info!("Finding available domain");
    epp_proxy::client::domain::check(&ga_domain, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.2.3 - Create a domain with that ASCII label with all fields populated
    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string()),
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6687,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559"
                            .to_string(),
                        key_data: None,
                    },
                ]),
            }),
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.2.4 - Perform a check and verify that the ASCII domain label is no longer available
    info!("Checking domain was registered");
    epp_proxy::client::domain::check(&ga_domain, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.3.2 - Perform a check to see that a Japanese IDN domain label is available
    info!("Finding available IDN domain");
    epp_proxy::client::domain::check(&ga_domain_idn, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.3.3 - Create a domain with that Japanese IDN label with all fields populated
    info!("Creating IDN domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain_idn,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string()),
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6687,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559"
                            .to_string(),
                        key_data: None,
                    },
                ]),
            }),
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.3.4 - Perform a check and verify that the Japanese IDN domain label is no longer available
    info!("Checking IDN domain was registered");
    epp_proxy::client::domain::check(&ga_domain_idn, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.4.2 - Perform a check to see that domain name “test‑and‑validate.<registrar name>‑ga” is available
    info!("Finding available claims domain");
    epp_proxy::client::domain::check(&ga_domain_claims, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.4.3 - Perform a check to retrieve the TCNID for “test‑and‑validate”
    info!("Getting claims domain TCNID");
    let mut claims_res = epp_proxy::client::domain::launch_claims_check(
        &ga_domain_claims,
        epp_proxy::client::launch::LaunchClaimsCheck {
            phase: epp_proxy::client::launch::LaunchPhase {
                phase_type: epp_proxy::client::launch::PhaseType::Claims,
                phase_name: None,
            },
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();
    let claims_key = claims_res.response.claims_key.pop().unwrap().key;

    // 2.4.4 - Look up the Claims Notice using the TCNID in the TMDB.
    info!("Getting claims notice");
    let req_client = reqwest::Client::new();
    let claims_notice_txt = req_client
        .get(format!("https://test.tmcnis.org/cnis/{}.xml", claims_key))
        .basic_auth(tmcnis_user, Some(tmcnis_pass))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let claims_notice_msg: epp_proxy::proto::tm_notice::TMMessage =
        xml_serde::from_str(&claims_notice_txt).unwrap();

    // 2.4.5 - Create the domain name “test‑and‑validate.<registrar name>‑ga” with all fields populated
    info!("Creating claims domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain_claims,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string()),
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: Some(epp_proxy::client::launch::LaunchCreate {
                phase: epp_proxy::client::launch::LaunchPhase {
                    phase_type: epp_proxy::client::launch::PhaseType::Claims,
                    phase_name: None,
                },
                code_mark: vec![],
                signed_mark: None,
                create_type: epp_proxy::client::launch::LaunchCreateType::Registration,
                notices: vec![epp_proxy::client::launch::Notice {
                    notice_id: claims_notice_msg.notice.id,
                    validator: None,
                    not_after: claims_notice_msg.notice.not_after,
                    accepted_date: claims_notice_msg.notice.not_before,
                }],
                core_nic: vec![],
            }),
            isnic_payment: None,
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6687,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559"
                            .to_string(),
                        key_data: None,
                    },
                ]),
            }),
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.4.6 - Perform a check and verify that the domain name “test‑and‑validate.<registrar name>‑ga” is no longer available
    info!("Checking claims domain was registered");
    epp_proxy::client::domain::check(&ga_domain_claims, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.5.2 - Perform a check to see that the domain name “rich.<registrar name>‑ga” is available, and that it is a premium domain name
    info!("Finding available premium domain");
    let premium_check_res = epp_proxy::client::domain::check(
        &ga_domain_premium,
        Some(epp_proxy::client::fee::FeeCheck {
            currency: None,
            commands: vec![epp_proxy::client::fee::FeeCheckCommand {
                command: epp_proxy::client::fee::Command::Create,
                period: Some(epp_proxy::client::Period {
                    unit: epp_proxy::client::PeriodUnit::Years,
                    value: 2,
                }),
                phase: None,
                sub_phase: None,
            }],
        }),
        None,
        None,
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();
    let premium_fee_check = premium_check_res
        .response
        .fee_check
        .unwrap()
        .commands
        .into_iter()
        .find(|c| c.command == epp_proxy::client::fee::Command::Create)
        .unwrap();

    // 2.5.3 - Create a domain with the domain name “rich.<registrar name>‑ga” with all fields populated
    info!("Creating premium domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain_premium,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string()),
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: Some(epp_proxy::client::fee::FeeAgreement {
                currency: Some(premium_fee_check.currency),
                fees: premium_fee_check.fees,
            }),
            launch_create: None,
            isnic_payment: None,
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6687,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559"
                            .to_string(),
                        key_data: None,
                    },
                ]),
            }),
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.5.4 - Perform a check and verify that the domain name “rich.<registrar name>‑ga” is no longer available.
    info!("Checking premium domain was registered");
    epp_proxy::client::domain::check(&ga_domain_premium, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.6.2 - Create two subordinate host objects underneath domain created in Step 2.2.
    // Each host object should have a single IPv4 and IPv6 address.
    info!("Creating hosts");
    epp_proxy::client::host::create(
        &ga_domain_ns1,
        vec![
            epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            },
            epp_proxy::client::host::Address {
                address: "2606:4700:4700::1111".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            },
        ],
        None,
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();
    epp_proxy::client::host::create(
        &ga_domain_ns2,
        vec![
            epp_proxy::client::host::Address {
                address: "1.0.0.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            },
            epp_proxy::client::host::Address {
                address: "2606:4700:4700::1001".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            },
        ],
        None,
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.6.3 - Change the domain's nameservers to point at the new host objects
    info!("Adding hosts to domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: &ga_domain,
            add: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns1.clone()),
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns2.clone()),
                ),
            ],
            remove: vec![],
            new_auth_info: None,
            new_registrant: None,
            sec_dns: None,
            launch_info: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            isnic_info: None,
            eurid_data: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.7.2 - Remove the nameservers set in Step 2.6
    info!("Removing hosts from domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: &ga_domain,
            add: vec![],
            remove: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns1.clone()),
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns2.clone()),
                ),
            ],
            new_auth_info: None,
            new_registrant: None,
            sec_dns: None,
            launch_info: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            isnic_info: None,
            eurid_data: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.7.3 - Change the IPv4 and IPv6 addresses of the host objects
    info!("Updating host objects");
    epp_proxy::client::host::update(
        &ga_domain_ns1,
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.1.1.2".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1112".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            }),
        ],
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1111".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            }),
        ],
        None,
        None,
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();
    epp_proxy::client::host::update(
        &ga_domain_ns2,
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.0.0.2".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1002".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            }),
        ],
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.0.0.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1001".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            }),
        ],
        None,
        None,
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.7.4 - Delete the subordinate host objects
    info!("Deleting host objects");
    epp_proxy::client::host::delete(&ga_domain_ns1, &mut cmd_tx_ga_1)
        .await
        .unwrap();
    epp_proxy::client::host::delete(&ga_domain_ns2, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    info!("Waiting for host pending delete to expire");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        let res = epp_proxy::client::host::check(&ga_domain_ns1, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break;
        }
    }
    loop {
        let res = epp_proxy::client::host::check(&ga_domain_ns2, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }

    // 2.8.2 - Change the DNSSEC information of the domain created in Step 2.2
    info!("Updating DNSSEC on domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: &ga_domain,
            add: vec![],
            remove: vec![],
            new_auth_info: None,
            new_registrant: None,
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 6687,
                            algorithm: 13,
                            digest_type: 2,
                            digest:
                                "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559"
                                    .to_string(),
                            key_data: None,
                        },
                    ]),
                )),
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6689,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D558"
                            .to_string(),
                        key_data: None,
                    },
                ])),
                new_max_sig_life: None,
            }),
            launch_info: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            isnic_info: None,
            eurid_data: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.9.2 - Query the info of the domain created in Step 2.2 and verify that it is still in the add grace period
    info!("Checking domain in AGP");
    epp_proxy::client::domain::info(&ga_domain, None, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.9.3 - Delete domain created in Step 2.2
    info!("Deleting domain");
    epp_proxy::client::domain::delete(&ga_domain, None, None, None, None,&mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.9.4 - Perform a check and verify that the domain label is now available
    info!("Checking domain now available");
    epp_proxy::client::domain::check(&ga_domain, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.10.2 - Wait at least 60 minutes for the add grace period to expire
    info!("Waiting for AGP to expire");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        let res = epp_proxy::client::domain::info(
            &ga_domain_idn,
            None,
            None,
            None,
            None,
            &mut cmd_tx_ga_1,
        )
        .await
        .unwrap();
        if !res
            .response
            .rgp_state
            .contains(&epp_proxy::client::rgp::RGPState::AddPeriod)
        {
            break;
        }
    }

    // 2.10.3 - Query the info of the domain created in Step 2.3 and verify that it is not in the add grace period
    info!("Checking IDN domain not in AGP");
    epp_proxy::client::domain::info(&ga_domain_idn, None, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.10.4 - Delete domain created in Step 2.3
    info!("Deleting IDN domain");
    epp_proxy::client::domain::delete(&ga_domain_idn, None, None, None, None,&mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.10.5 - Perform a check and verify that the domain label is still not available
    info!("Checking domain still not available");
    epp_proxy::client::domain::check(&ga_domain_idn, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.11.2 - Query the info of the domain used in Step 2.10 and verify that it has pending delete
    // status and is in the redemption grace period
    info!("Checking IDN domain in RGP");
    epp_proxy::client::domain::info(&ga_domain_idn, None, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.11.3 - Restore the domain. Note that no restore report is required
    info!("Restoring IDN domain");
    epp_proxy::client::rgp::request(&ga_domain_idn, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.11.4 - Query the info of the domain again verify that it no longer has pending delete status
    info!("Checking IDN domain no longer in RGP");
    epp_proxy::client::domain::info(&ga_domain_idn, None, None, None, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.12.2 - Delete the domain used in Step 2.11
    info!("Deleting IDN domain");
    epp_proxy::client::domain::delete(&ga_domain_idn, None, None, None, None,&mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.12.3 - Wait at least 15 minutes
    info!("Waiting for RGP to expire");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        let res = epp_proxy::client::domain::check(&ga_domain_idn, None, None, None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break;
        }
    }

    // 2.12.4 - Perform a poll command and verify the receipt of a poll message announcing the release of the domain
    info!("Polling deletion message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx_ga_1)
        .await
        .unwrap();
    epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.13.1 - Using account <registrar name>‑3 create a domain
    info!("Finding available domain");
    let mut trans_domain_i = 1;
    let trans_domain_1 = loop {
        let trans_domain = format!("staclar-{}.{}", trans_domain_i, ga_tld);
        let res = epp_proxy::client::domain::check(&trans_domain, None, None, None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break trans_domain;
        } else {
            trans_domain_i += 1;
        }
    };

    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &trans_domain_1,
            nameservers: vec![],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.13.2 - Using account <registrar name>‑4 request a transfer on the domain created above
    info!("Requesting transfer");
    epp_proxy::client::domain::transfer_request(
        &trans_domain_1,
        None,
        "test_auth1",
        None,
        None,
        None,
        None,
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.13.2.a - Perform a transfer query and verify that the domain’s transfer status is now pending
    info!("Checking transfer is pending");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_1,
        Some("test_auth1"),
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.14.1 - Using account <registrar name>‑3 create a domain
    info!("Finding available domain");
    trans_domain_i += 1;
    let trans_domain_2 = loop {
        let trans_domain = format!("staclar-{}.{}", trans_domain_i, ga_tld);
        let res = epp_proxy::client::domain::check(&trans_domain, None, None, None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break trans_domain;
        } else {
            trans_domain_i += 1;
        }
    };

    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &trans_domain_2,
            nameservers: vec![],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.14.2 - Using account <registrar name>‑4 request a transfer on the domain created above
    info!("Requesting transfer");
    epp_proxy::client::domain::transfer_request(
        &trans_domain_2,
        None,
        "test_auth1",
        None,
        None,
        None,
        None,
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.14.2.a - Perform a transfer query and verify that the domain’s transfer status is now pending
    info!("Checking transfer is pending");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_2,
        Some("test_auth1"),
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.15.1 - Using account <registrar name>‑3 create a domain
    info!("Finding available domain");
    trans_domain_i += 1;
    let trans_domain_3 = loop {
        let trans_domain = format!("staclar-{}.{}", trans_domain_i, ga_tld);
        let res = epp_proxy::client::domain::check(&trans_domain, None, None, None, &mut cmd_tx_ga_1)
            .await
            .unwrap();
        if res.response.avail {
            break trans_domain;
        } else {
            trans_domain_i += 1;
        }
    };

    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &trans_domain_3,
            nameservers: vec![],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.15.2 - Using account <registrar name>‑4 request a transfer on the domain created above
    info!("Requesting transfer");
    epp_proxy::client::domain::transfer_request(
        &trans_domain_3,
        None,
        "test_auth1",
        None,
        None,
        None,
        None,
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.15.2.a - Perform a transfer query and verify that the domain’s transfer status is now pending
    info!("Checking transfer is pending");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_3,
        Some("test_auth1"),
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.16.1 - Using account <registrar name>‑3 approve a pending transfer away from this registrar
    // on the domain created in Step 2.13
    info!("Accepting transfer");
    epp_proxy::client::domain::transfer_accept(&trans_domain_1, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.16.1.b - Perform a transfer query and verify that the domain’s transfer status is now client approved
    info!("Checking transfer is approved");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_1,
        Some("test_auth1"),
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.16.2 - Using account <registrar name>‑4 perform a poll command and verify the receipt of a
    // poll message announcing the approval of the transfer of the domain
    info!("Polling transfer message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx_ga_2)
        .await
        .unwrap();
    epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx_ga_2)
        .await
        .unwrap();

    // 2.16.2.b - Do an info on the domain and verify its sponsoring client is now set to <registrar name>‑4
    info!("Checking domain sponsorship changed");
    epp_proxy::client::domain::info(&trans_domain_1, None, None, None, None, &mut cmd_tx_ga_2)
        .await
        .unwrap();

    // 2.17.1 - Using account <registrar name>‑3 deny a pending transfer away from this registrar on the domain created in Step 2.14
    info!("Denying transfer");
    epp_proxy::client::domain::transfer_reject(&trans_domain_2, None, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    // 2.17.1.b - Perform a transfer query and verify that the domain’s transfer status is now client rejected
    info!("Checking transfer is denied");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_2,
        Some("test_auth1"),
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.17.2 - Using account <registrar name>‑4 perform a poll command and verify the receipt of a
    // poll message announcing the denial of the transfer of the domain
    info!("Polling transfer message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx_ga_2)
        .await
        .unwrap();
    epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx_ga_2)
        .await
        .unwrap();

    // 2.17.2.b - Do an info on the domain and verify its sponsoring client is still set to <registrar name>‑3
    info!("Checking domain sponsorship not changed");
    epp_proxy::client::domain::info(
        &trans_domain_2,
        Some("test_auth1"),
        None,
        None,
        None,
        &mut cmd_tx_ga_2,
    )
    .await
    .unwrap();

    // 2.18.1 - Using account <registrar name>‑4 cancel the pending transfer on the domain created in Step 2.15
    info!("Canceling transfer");
    epp_proxy::client::domain::transfer_cancel(&trans_domain_3, None, &mut cmd_tx_ga_2)
        .await
        .unwrap();

    // 2.18.1.b - Perform a transfer query and verify that the domain’s transfer status is now client cancelled
    info!("Checking transfer is cancelled");
    epp_proxy::client::domain::transfer_query(
        &trans_domain_3,
        Some("test_auth1"),
        &mut cmd_tx_ga_1,
    )
    .await
    .unwrap();

    // 2.18.2 - Using account <registrar name>‑3 perform a poll command and verify the receipt of a
    // poll message announcing the cancellation of the transfer of the domain
    info!("Polling transfer message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx_ga_1)
        .await
        .unwrap();
    epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx_ga_1)
        .await
        .unwrap();

    info!("Logging out of GA accounts");
    let final_cmd_ga_1 = epp_proxy::client::logout(cmd_tx_ga_1).await.unwrap();
    let final_cmd_ga_2 = epp_proxy::client::logout(cmd_tx_ga_2).await.unwrap();

    println!(
        "Final command transaction: {:#?}",
        final_cmd_ga_1.transaction_id
    );
    println!(
        "Final command transaction: {:#?}",
        final_cmd_ga_2.transaction_id
    );

    let (mut cmd_tx_sunrise, mut ready_rx_sunrise) = epp_client_sunrise.start();

    info!("Awaiting client sunrise to become ready...");
    let login_trans_id = ready_rx_sunrise.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // 3.1.2 - Create a domain using an encoded signed mark provided by the TMCH for testing purposes
    info!("Creating domain with SMD");
    let test_smd = include_str!("./test-smd.txt");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &format!("test-and-validate.{}", sunrise_tld),
            nameservers: vec![],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: contact_id.clone(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: contact_id.clone(),
                },
            ],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: Some(epp_proxy::client::launch::LaunchCreate {
                phase: epp_proxy::client::launch::LaunchPhase {
                    phase_type: epp_proxy::client::launch::PhaseType::Sunrise,
                    phase_name: None,
                },
                code_mark: vec![],
                signed_mark: Some(test_smd.to_string()),
                create_type: epp_proxy::client::launch::LaunchCreateType::Registration,
                notices: vec![],
                core_nic: vec![],
            }),
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_sunrise,
    )
    .await
    .unwrap();

    info!("Logging out of sunrise account");
    let final_cmd_sunrise = epp_proxy::client::logout(cmd_tx_sunrise).await.unwrap();
    println!(
        "Final command transaction: {:#?}",
        final_cmd_sunrise.transaction_id
    );
}
