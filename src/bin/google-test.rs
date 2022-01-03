#[macro_use]
extern crate log;

use chrono::Datelike;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::App::new("google-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the Google EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::with_name("acct-ga-1")
                .short("aga1")
                .long("account_ga_1")
                .takes_value(true)
                .required(true)
                .help("Config file for the first General Availability account"),
        )
        .arg(
            clap::Arg::with_name("acct-ga-2")
                .short("aga2")
                .long("account_ga_2")
                .takes_value(true)
                .required(true)
                .help("Config file for the second General Availability account"),
        )
        .arg(
            clap::Arg::with_name("acct-sunrise")
                .short("as")
                .long("account_sunrise")
                .takes_value(true)
                .required(true)
                .help("Config file for the sunrise account"),
        )
        .arg(
            clap::Arg::with_name("registrar_name")
                .short("r")
                .long("registrar_name")
                .takes_value(true)
                .required(true)
                .help("Registrar account name"),
        )
        .arg(
            clap::Arg::with_name("domain")
                .short("d")
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("Domain to use for testing (without TLD)"),
        )
        .arg(
            clap::Arg::with_name("idn_domain")
                .short("id")
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("IDN Domain to use for testing (without TLD)"),
        )
        .arg(
            clap::Arg::with_name("tmcnis_user")
                .short("tmu")
                .long("tmcnis_user")
                .takes_value(true)
                .required(true)
                .help("Username for trademark CNIS"),
        )
        .arg(
            clap::Arg::with_name("tmcnis_pass")
                .short("tmp")
                .long("tmcnis_password")
                .takes_value(true)
                .required(true)
                .help("Password for trademark CNIS"),
        )
        .arg(
            clap::Arg::with_name("hsm_conf")
                .short("h")
                .long("hsm-conf")
                .takes_value(true)
                .help("Where to read the HSM config file from"),
        )
        .arg(
            clap::Arg::with_name("log")
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
    let ga_domain = format!("{}.{}", domain, ga_tld);
    let ga_domain_ns1 = format!("ns1.{}", ga_domain);
    let ga_domain_ns2 = format!("ns2.{}", ga_domain);
    let ga_domain_idn = format!("{}.{}", domain_idn, ga_tld);
    let ga_domain_claims = format!("test‑and‑validate.{}", ga_tld);
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

    let epp_client_ga_1 = epp_proxy::create_client(log_dir_ga_1, &conf_ga_1, &pkcs11_engine, false).await;
    let epp_client_ga_2 = epp_proxy::create_client(log_dir_ga_2, &conf_ga_2, &pkcs11_engine, false).await;
    let epp_client_sunrise = epp_proxy::create_client(log_dir_sunrise, &conf_sunrise, &pkcs11_engine, false).await;

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
        let mut contact_id = format!("STACLAR-{}", contact_id_i);
        let res = epp_proxy::client::contact::check(&contact_id, &mut cmd_tx_ga_1).await.unwrap();
        if res.response.avail {
            break contact_id;
        } else {
            contact_id_i += 1;
        }
    };

    info!("Creating contact");
    epp_proxy::client::contact::create(&contact_id, epp_proxy::client::contact::NewContactData {
        local_address: Some(epp_proxy::client::contact::Address {
            name: "Boaty McBoat Face".to_string(),
            organisation: None,
            streets: vec![
                "10 Downing St".to_string()
            ],
            city: "London".to_string(),
            province: Some("England".to_string()),
            postal_code: Some("SW1A 2AA".to_string()),
            country_code: "GB".to_string(),
            identity_number: None,
            birth_date: None
        }),
        internationalised_address: None,
        phone: Some(epp_proxy::client::Phone {
            number: "+44.1818118181".to_string(),
            extension: None
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
        qualified_lawyer: None
    }, &mut cmd_tx_ga_1).await.unwrap();


    info!("Creating nameservers");
    let res = epp_proxy::client::host::check("ns1.as207960.net", &mut cmd_tx_ga_1).await.unwrap();
    if !res.response.avail {
        epp_proxy::client::host::create("ns1.as207960.net", vec![], None, &mut cmd_tx_ga_1).await.unwrap();
    }
    let res = epp_proxy::client::host::check("ns2.as207960.net", &mut cmd_tx_ga_1).await.unwrap();
    if !res.response.avail {
        epp_proxy::client::host::create("ns2.as207960.net", vec![], None, &mut cmd_tx_ga_1).await.unwrap();
    }

    // 2.2.2 - Perform a check to see that an ASCII domain label is available.
    info!("Finding available domain");
    epp_proxy::client::domain::check(&ga_domain, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.2.3 - Create a domain with that ASCII label with all fields populated
    info!("Creating domain");
    let domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string())
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![epp_proxy::client::domain::InfoContact {
                contact_type: "admin".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "tech".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "billing".to_string(),
                contact_id: contact_id.clone()
            }],
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
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559".to_string(),
                        key_data: None
                    }
                ])
            }),
        },
        &mut cmd_tx_ga_1,
    ).await.unwrap();

    // 2.2.4 - Perform a check and verify that the ASCII domain label is no longer available
    info!("Checking domain was registered");
    epp_proxy::client::domain::check(&ga_domain, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.3.2 - Perform a check to see that a Japanese IDN domain label is available
    info!("Finding available IDN domain");
    epp_proxy::client::domain::check(&ga_idn_domain, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.3.3 - Create a domain with that Japanese IDN label with all fields populated
    info!("Creating IDN domain");
    let idn_domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_idn_domain,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string())
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![epp_proxy::client::domain::InfoContact {
                contact_type: "admin".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "tech".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "billing".to_string(),
                contact_id: contact_id.clone()
            }],
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
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559".to_string(),
                        key_data: None
                    }
                ])
            }),
        },
        &mut cmd_tx_ga_1,
    ).await.unwrap();

    // 2.3.4 - Perform a check and verify that the Japanese IDN domain label is no longer available
    info!("Checking IDN domain was registered");
    epp_proxy::client::domain::check(&ga_idn_domain, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.4.2 - Perform a check to see that domain name “test‑and‑validate.<registrar name>‑ga” is available
    info!("Finding available claims domain");
    epp_proxy::client::domain::check(&ga_domain_claims, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.4.3 - Perform a check to retrieve the TCNID for “test‑and‑validate”
    info!("Getting claims domain TCNID");
    let mut claims_res = epp_proxy::client::domain::launch_claims_check(
        &ga_domain_claims,
        epp_proxy::client::launch::LaunchClaimsCheck {
            phase: epp_proxy::client::launch::LaunchPhase {
                phase_type: epp_proxy::client::launch::PhaseType::Claims,
                phase_name: None
            }
        },
        &mut cmd_tx_ga_1
    ).await.unwrap();
    let claims_key = claims_res.response.claims_key.pop().unwrap().key;

    // 2.4.4 - Look up the Claims Notice using the TCNID in the TMDB.
    info!("Getting claims notice");
    let req_client = reqwest::Client::new();
    let claims_notice_txt = req_client.get(format!("https://test.tmcnis.org/cnis/{}.xml", claims_key))
        .basic_auth(tmcnis_user, Some(tmcnis_pass))
        .send().await.unwrap()
        .text().await.unwrap();
    let claims_notice_msg: epp_proxy::proto::tm_notice::TMMessage = xml_serde::from_str(&claims_notice_txt).unwrap();

    // 2.4.5 - Create the domain name “test‑and‑validate.<registrar name>‑ga” with all fields populated
    info!("Creating claims domain");
    let claims_domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain_claims,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string())
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![epp_proxy::client::domain::InfoContact {
                contact_type: "admin".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "tech".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "billing".to_string(),
                contact_id: contact_id.clone()
            }],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: Some(epp_proxy::client::launch::LaunchCreate {
                phase: epp_proxy::client::launch::LaunchPhase {
                    phase_type: epp_proxy::client::launch::PhaseType::Claims,
                    phase_name: None
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
                core_nic: vec![]
            }),
            isnic_payment: None,
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 6687,
                        algorithm: 13,
                        digest_type: 2,
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559".to_string(),
                        key_data: None
                    }
                ])
            }),
        },
        &mut cmd_tx_ga_1,
    ).await.unwrap();

    // 2.4.6 - Perform a check and verify that the domain name “test‑and‑validate.<registrar name>‑ga” is no longer available
    info!("Checking claims domain was registered");
    epp_proxy::client::domain::check(&ga_domain_claims, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.5.2 - Perform a check to see that the domain name “rich.<registrar name>‑ga” is available, and that it is a premium domain name
    info!("Finding available premium domain");
    let premium_check_res = epp_proxy::client::domain::check(
        &ga_domain_premium, Some(epp_proxy::client::fee::FeeCheck {
            currency: None,
            commands: vec![epp_proxy::client::fee::FeeCheckCommand {
                command: epp_proxy::client::fee::Command::Create,
                period: Some(epp_proxy::client::Period {
                    unit: epp_proxy::client::PeriodUnit::Years,
                    value: 2,
                })
            }]
        }), None, &mut cmd_tx_ga_1
    ).await.unwrap();
    let premium_fee_check = premium_check_res.response.fee_check.unwrap()
        .commands.into_iter().filter(|c| c.command == epp_proxy::client::fee::Command::Create)
        .next().unwrap();

    // 2.5.3 - Create a domain with the domain name “rich.<registrar name>‑ga” with all fields populated
    info!("Creating premium domain");
    let idn_domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &ga_domain_premium,
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.as207960.net".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.as207960.net".to_string())
            ],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: &contact_id,
            contacts: vec![epp_proxy::client::domain::InfoContact {
                contact_type: "admin".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "tech".to_string(),
                contact_id: contact_id.clone()
            }, epp_proxy::client::domain::InfoContact {
                contact_type: "billing".to_string(),
                contact_id: contact_id.clone()
            }],
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
                        digest: "66818ACF61D1EF06C90B5871A045E2302A7474A6BAC046FE3FE23B9338F9D559".to_string(),
                        key_data: None
                    }
                ])
            }),
        },
        &mut cmd_tx_ga_1,
    ).await.unwrap();

    // 2.5.4 - Perform a check and verify that the domain name “rich.<registrar name>‑ga” is no longer available.
    info!("Checking IDN domain was registered");
    epp_proxy::client::domain::check(&ga_domain_premium, None, None, &mut cmd_tx_ga_1).await.unwrap();

    // 2.6.2 - Create two subordinate host objects underneath domain created in Step 2.2.
    // Each host object should have a single IPv4 and IPv6 address.
    info!("Creating hosts");
    epp_proxy::client::host::create(&ga_domain_ns1, vec![epp_proxy::client::host::Address {
            address: "1.1.1.1".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv4,
        }, epp_proxy::client::host::Address {
            address: "2606:4700:4700::1111".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv6,
        }], None, &mut cmd_tx_ga_1
    ).await.unwrap();
    epp_proxy::client::host::create(&ga_domain_ns2, vec![epp_proxy::client::host::Address {
            address: "1.0.0.1".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv4,
        }, epp_proxy::client::host::Address {
            address: "2606:4700:4700::1001".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv6,
        }], None, &mut cmd_tx_ga_1
    ).await.unwrap();

    // 2.6.3 - Change the domain's nameservers to point at the new host objects
    info!("Adding hosts to domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: &ga_domain,
            add: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns1.clone())
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns2.clone())
                )
            ],
            remove: vec![],
            new_auth_info: None,
            new_registrant: None,
            sec_dns: None,
            launch_info: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            isnic_info: None,
            eurid_data: None
        },
        &mut cmd_tx_ga_1
    ).await.unwrap();

    // 2.7.2 - Remove the nameservers set in Step 2.6
    info!("Removing hosts from domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: &ga_domain,
            add: vec![],
            remove: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns1.clone())
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(ga_domain_ns2.clone())
                )
            ],
            new_auth_info: None,
            new_registrant: None,
            sec_dns: None,
            launch_info: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            isnic_info: None,
            eurid_data: None
        },
        &mut cmd_tx_ga_1
    ).await.unwrap();

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
            })
        ],
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1111".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            })
        ],
        None, None, &mut cmd_tx_ga_1
    ).await.unwrap();
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
            })
        ],
        vec![
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "1.0.0.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }),
            epp_proxy::client::host::UpdateObject::Address(epp_proxy::client::host::Address {
                address: "2606:4700:4700::1001".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv6,
            })
        ],
        None, None, &mut cmd_tx_ga_1
    ).await.unwrap();

    // 2.7.4 - Delete the subordinate host objects
    info!("Deleting host objects");
    epp_proxy::client::host::delete(&ga_domain_ns1, &mut cmd_tx_ga_1).await.unwrap();
    epp_proxy::client::host::delete(&ga_domain_ns2, &mut cmd_tx_ga_1).await.unwrap();

    

    info!("======");
    info!("Adding hosts to domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain,
                add: vec![
                    epp_proxy::client::domain::UpdateObject::Nameserver(
                        epp_proxy::client::domain::InfoNameserver::HostOnly(format!(
                            "ns1.{}",
                            domain
                        ))
                    ),
                    epp_proxy::client::domain::UpdateObject::Nameserver(
                        epp_proxy::client::domain::InfoNameserver::HostOnly(format!(
                            "ns2.{}",
                            domain
                        ))
                    )
                ],
                remove: vec![],
                new_auth_info: None,
                new_registrant: None,
                sec_dns: None,
                launch_info: None,
                fee_agreement: None,
                donuts_fee_agreement: None,
                isnic_info: None,
                eurid_data: None
            },
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Update the domain to add the domain client statuses of ClientHold, ClientUpdateProhibited,
    // ClientDeleteProhibited, and ClientTransferProhibited within one command using the MOD domain
    // command with your OT&E1 account logon

    info!("======");
    info!("Adding statuses to domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain,
                add: vec![
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientHold
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientDeleteProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientTransferProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientUpdateProhibited
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
                eurid_data: None
            },
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Perform an INFO on the domain to verify the update using the STATUS-FULL command with your
    // OT&E1 account logon

    info!("======");
    info!("Getting domain info");
    info!(
        "{:#?}",
        epp_proxy::client::domain::info(domain, None, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap()
    );

    // UPDATE domain to remove domain client statuses of ClientHold, ClientUpdateProhibited,
    // ClientDeleteProhibited, and ClientTransferProhibited within one command using the MOD domain
    // command with your OT&E1 account logon

    info!("======");
    info!("Removing statuses from domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain,
                add: vec![],
                remove: vec![
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientHold
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientDeleteProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientTransferProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientUpdateProhibited
                    ),
                ],
                new_auth_info: None,
                new_registrant: None,
                sec_dns: None,
                launch_info: None,
                fee_agreement: None,
                donuts_fee_agreement: None,
                isnic_info: None,
                eurid_data: None
            },
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Perform an INFO on the domain to verify the update using the STATUS-FULL command with your
    // OT&E1 account logon

    info!("======");
    info!("Getting domain info");
    info!(
        "{:#?}",
        epp_proxy::client::domain::info(domain, None, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap()
    );

    // Update the domain with <new Auth Info> AUTH INFO code using the MOD domain command with your
    // OT&E1 account logon

    info!("======");
    info!("Updating auth info");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain,
                add: vec![],
                remove: vec![],
                new_auth_info: Some("test_auth2"),
                new_registrant: None,
                sec_dns: None,
                launch_info: None,
                fee_agreement: None,
                donuts_fee_agreement: None,
                isnic_info: None,
                eurid_data: None
            },
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Update the IP Address of child nameserver 1 of the newly created domain using the MOD
    // nameserver command with your OT&E1 account logon

    info!("======");
    info!("Updating host 1 IP address");
    info!(
        "{:#?}",
        epp_proxy::client::host::update(
            &format!("ns1.{}", domain),
            vec![epp_proxy::client::host::UpdateObject::Address(
                epp_proxy::client::host::Address {
                    address: "1.1.1.2".to_string(),
                    ip_version: epp_proxy::client::host::AddressVersion::IPv4,
                }
            )],
            vec![epp_proxy::client::host::UpdateObject::Address(
                epp_proxy::client::host::Address {
                    address: "1.1.1.1".to_string(),
                    ip_version: epp_proxy::client::host::AddressVersion::IPv4,
                }
            )],
            None,
            None,
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Perform a HELLO command with your OT&E1 account logon
    let (sender, _) = futures::channel::oneshot::channel();
    cmd_tx_1
        .try_send(epp_proxy::client::RequestMessage::Hello(Box::new(
            epp_proxy::client::BlankRequest {
                return_path: sender,
            },
        )))
        .unwrap();

    // Renew your newly created domain for 2 years using the RENEW domain command with your OT&E1
    // account logon, term of renewal should be 2 years

    info!("======");
    info!("Renewing domain");
    let renew_res = epp_proxy::client::domain::renew(
        domain,
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 2,
        }),
        domain_create_res.response.data.expiration_date.unwrap(),
        None,
        None,
        None,
        &mut cmd_tx_1,
    )
    .await
    .unwrap();
    info!("{:#?}", renew_res);

    // Establish a second session using the EPP Login SESSION command with your OT&E2 account logon.
    let (mut cmd_tx_2, mut ready_rx_2) = epp_client_2.start();

    info!("Awaiting client 2 to become ready...");
    ready_rx_2.next().await.unwrap();

    // Perform an INFO on the newly created domain using the STATUS-FULL command with your OT&E2
    // account logon and the <new Auth Info> AUTH INFO code

    info!("======");
    info!("Getting domain info with auth");
    info!(
        "{:#?}",
        epp_proxy::client::domain::info(
            domain,
            Some("test_auth2"),
            None,
            None,
            None,
            &mut cmd_tx_2
        )
        .await
        .unwrap()
    );

    // Initiate a Transfer domain request on the newly created domain using the TRANSFER-REQUEST
    // command with your OT&E2 account logon

    info!("======");
    info!("Requesting domain transfer");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_request(
            domain,
            None,
            "test_auth2",
            None,
            None,
            None,
            &mut cmd_tx_2
        )
        .await
        .unwrap()
    );

    // Perform a TRANSFER-QUERY command using your OT&E2 account logon

    info!("======");
    info!("Querying transfer status");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_query(domain, Some("test_auth2"), &mut cmd_tx_2)
            .await
            .unwrap()
    );

    // Approve the transfer using the TRANSFER-APPROVE command with your OT&E1 account logon

    info!("======");
    info!("Accepting transfer request");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_accept(domain, None, &mut cmd_tx_1)
            .await
            .unwrap()
    );

    // Perform a POLL-REQUEST command to check for messages in Poll Queue using your OT&E1 account
    // logon

    info!("======");
    info!("Polling 1 message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx_1).await.unwrap();
    info!("{:#?}", poll_msg);

    // Acknowledge the first poll message using the POLL-ACK command with your OT&E1 account logon

    info!("======");
    info!("Acknowledging message");
    info!(
        "{:#?}",
        epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx_1)
            .await
            .unwrap()
    );

    // Initiate a Transfer domain request again on the newly created domain using the
    // TRANSFER-REQUEST command with your OT&E1 account logon

    info!("======");
    info!("Requesting domain transfer");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_request(
            domain,
            None,
            "test_auth2",
            None,
            None,
            None,
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Perform a TRANSFER-QUERY command using your OT&E2 account logon

    info!("======");
    info!("Querying transfer status");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_query(domain, None, &mut cmd_tx_2)
            .await
            .unwrap()
    );

    // Reject the transfer of the newly created domain using the TRANSFER-REJECT command with your
    // OT&E2 account logon

    info!("======");
    info!("Rejecting transfer request");
    info!(
        "{:#?}",
        epp_proxy::client::domain::transfer_reject(domain, None, &mut cmd_tx_2)
            .await
            .unwrap()
    );

    // Sync the domain to the 15th day of the next month using the SYNC domain command with your
    // OT&E2 account logon, the sync date should be 15

    info!("======");
    info!("Syncing domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::verisign_sync(
            domain,
            (renew_res.response.data.new_expiry_date.unwrap().month() % 12) + 1,
            15,
            &mut cmd_tx_2
        )
        .await
        .unwrap()
    );

    epp_proxy::client::logout(cmd_tx_1).await.unwrap();
    let final_cmd = epp_proxy::client::logout(cmd_tx_2).await.unwrap();

    println!("Final command transaction: {:#?}", final_cmd.transaction_id);
}
