#[macro_use]
extern crate log;

use futures::StreamExt;

const ALPHABET: [char; 62] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::Command::new("donuts-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the Donuts EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("acct-1")
                .short('1')
                .long("account_1")
                .value_name("FILE")
                .required(true)
                .help("Config file for the first account"),
        )
        .arg(
            clap::Arg::new("acct-2")
                .short('2')
                .long("account_2")
                .value_name("FILE")
                .required(true)
                .help("Config file for the second account"),
        )
        .arg(
            clap::Arg::new("claims_domain")
                .short('c')
                .long("claims_domain")
                .value_name("DOMAIN")
                .required(true)
                .help("Domain to use for claims testing"),
        )
        .arg(
            clap::Arg::new("tmcnis_user")
                .short('u')
                .long("tmcnis_user")
                .value_name("USER")
                .required(true)
                .help("Username for trademark CNIS"),
        )
        .arg(
            clap::Arg::new("tmcnis_pass")
                .short('p')
                .long("tmcnis_password")
                .value_name("PASS")
                .required(true)
                .help("Password for trademark CNIS"),
        )
        .arg(
            clap::Arg::new("test_smd")
                .short('s')
                .long("test_smd")
                .value_name("FILE")
                .required(true)
                .help("Test SMD file"),
        )
        .arg(
            clap::Arg::new("test_smd_label")
                .short('l')
                .long("test_smd_label")
                .value_name("LABEL")
                .required(true)
                .help("Test SMD label"),
        )
        .arg(
            clap::Arg::new("premium_domain_1")
                .long("premium_domain_1")
                .value_name("DOMAIN")
                .required(true)
                .help("Premium domain 1"),
        )
        .arg(
            clap::Arg::new("premium_domain_2")
                .long("premium_domain_2")
                .value_name("DOMAIN")
                .required(true)
                .help("Premium domain 2"),
        )
        .arg(
            clap::Arg::new("premium_domain_3")
                .long("premium_domain_3")
                .value_name("DOMAIN")
                .required(true)
                .help("Premium domain 3"),
        )
        .arg(
            clap::Arg::new("premium_domain_dr")
                .long("premium_domain_delete_restore")
                .value_name("DOMAIN")
                .required(true)
                .help("Premium domain for delete / restore"),
        )
        .arg(
            clap::Arg::new("hsm_conf")
                .short('p')
                .long("hsm-conf")
                .value_name("FILE")
                .help("Where to read the HSM config file from"),
        )
        .arg(
            clap::Arg::new("log")
                .long("log")
                .value_name("DIR")
                .default_value("./log/")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Directory to write command logs to"),
        )
        .get_matches();

    let pkcs11_engine =
        epp_proxy::setup_pkcs11_engine(matches.get_one::<String>("hsm_conf").map(|x| x.as_str()))
            .await;
    let claims_domain = matches.get_one::<String>("claims_domain").unwrap();
    let test_smd = matches.get_one::<String>("test_smd").unwrap();
    let test_smd_label = matches.get_one::<String>("test_smd_label").unwrap();
    let premium_domain_1 = matches.get_one::<String>("premium_domain_1").unwrap();
    let premium_domain_2 = matches.get_one::<String>("premium_domain_2").unwrap();
    let premium_domain_3 = matches.get_one::<String>("premium_domain_3").unwrap();
    let premium_domain_dr = matches.get_one::<String>("premium_domain_dr").unwrap();
    let tmcnis_user = matches.get_one::<String>("tmcnis_user").unwrap();
    let tmcnis_pass = matches.get_one::<String>("tmcnis_pass").unwrap();

    let dpml_domain = format!("{}.dpml.zone", test_smd_label);
    let dpml_override_domain = format!("{}.test1ga", test_smd_label);
    let eap_domain = format!("{}.19earlyaccess", nanoid::nanoid!(16, &ALPHABET));

    let log_dir_path = matches.get_one::<std::path::PathBuf>("log").unwrap();
    match std::fs::create_dir_all(log_dir_path) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory: {}", e);
            return;
        }
    }

    let conf_file_1_path = matches.get_one::<String>("acct-1").unwrap();
    let conf_file_2_path = matches.get_one::<String>("acct-2").unwrap();

    let conf_file_1 = match std::fs::File::open(conf_file_1_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_1_path, e);
            return;
        }
    };
    let conf_file_2 = match std::fs::File::open(conf_file_2_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_2_path, e);
            return;
        }
    };
    let test_smd = match std::fs::read_to_string(test_smd) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open test SMD file {}: {}", test_smd, e);
            return;
        }
    };

    let conf_1: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file_1) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_1_path, e);
            return;
        }
    };
    let conf_2: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file_2) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_1_path, e);
            return;
        }
    };

    let storage = epp_proxy::FSStorage::new(log_dir_path.clone());
    let storage_1 = epp_proxy::StorageScoped::new(Box::new(storage.clone()), &conf_1.id);
    let storage_2 = epp_proxy::StorageScoped::new(Box::new(storage), &conf_2.id);

    let epp_client_1 = epp_proxy::create_client(storage_1, &conf_1, &pkcs11_engine, true).await;
    let epp_client_2 = epp_proxy::create_client(storage_2, &conf_2, &pkcs11_engine, true).await;

    // 2.1 - Login
    let (mut cmd_tx_1, mut ready_rx_1) = epp_client_1.start();
    let (mut cmd_tx_2, mut ready_rx_2) = epp_client_2.start();

    info!("Awaiting client 1 to become ready...");
    let login_trans_id = ready_rx_1.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);
    info!("Awaiting client 2 to become ready...");
    let login_trans_id = ready_rx_2.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // Misc setup
    info!("Setting up contacts");

    info!("Finding available contact ID");
    let mut contact_id_i = 1;
    let contact_id = loop {
        let contact_id = format!("STACLAR-{}", contact_id_i);
        let res = epp_proxy::client::contact::check(&contact_id, &mut cmd_tx_1)
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
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Perform a check command for a domain.
    info!("Domain check");
    epp_proxy::client::domain::check(
        &format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
        None,
        None,
        None,
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Submit a Sunrise application
    info!("Creating sunrise domain");
    let test_smd_2 = include_str!("./test-smd.txt");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "test-and-validate.sunrisemark2",
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
                signed_mark: Some(test_smd_2.to_string()),
                create_type: epp_proxy::client::launch::LaunchCreateType::Application,
                notices: vec![],
                core_nic: vec![],
            }),
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Register 5 domain names
    info!("Creating 5 domains");

    let domains = vec![
        format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
        format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
        format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
        format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
        format!("{}.test1ga", nanoid::nanoid!(16, &ALPHABET)),
    ];

    for domain in domains.iter() {
        epp_proxy::client::domain::create(
            epp_proxy::client::domain::CreateInfo {
                domain: domain.as_str(),
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
            &mut cmd_tx_1,
        )
        .await
        .unwrap();
    }
    info!("GA domain names: {:?}", domains);

    // Transfer 2 domain names from your OTE1 account to your OTE2 account
    info!("Transferring two domains");
    for domain in &domains[0..2] {
        epp_proxy::client::domain::transfer_request(
            domain.as_str(),
            None,
            "test_auth1",
            None,
            None,
            None,
            None,
            &mut cmd_tx_2,
        )
        .await
        .unwrap();

        epp_proxy::client::domain::transfer_accept(domain.as_str(), None, &mut cmd_tx_1)
            .await
            .unwrap();
    }

    // Register a domain name that is subject to Claims.
    info!("Creating claims domain");
    info!("Claims domain: {}", claims_domain);
    let mut claims_res = epp_proxy::client::domain::launch_claims_check(
        claims_domain,
        epp_proxy::client::launch::LaunchClaimsCheck {
            phase: epp_proxy::client::launch::LaunchPhase {
                phase_type: epp_proxy::client::launch::PhaseType::Claims,
                phase_name: None,
            },
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();
    let claims_key = claims_res.response.claims_key.pop().unwrap().key;

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

    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: claims_domain,
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
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Register a DPML Block, in .dpml.zone
    info!("Creating DPML block");
    info!("DPML block: {}", dpml_domain);
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &dpml_domain,
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
                    phase_type: epp_proxy::client::launch::PhaseType::Custom,
                    phase_name: Some("dpml".to_string()),
                },
                code_mark: vec![],
                signed_mark: Some(format!("<smd:encodedSignedMark xmlns:smd=\"urn:ietf:params:xml:ns:signedMark-1.0\">{}</smd:encodedSignedMark>", test_smd)),
                create_type: epp_proxy::client::launch::LaunchCreateType::Registration,
                notices: vec![],
                core_nic: vec![],
            }),
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Perform a DPML Block Override
    info!("Creating DPML block override");
    info!("DPML override domain: {}", dpml_override_domain);
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &dpml_override_domain,
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
                    phase_type: epp_proxy::client::launch::PhaseType::Custom,
                    phase_name: Some("dpml".to_string()),
                },
                code_mark: vec![],
                signed_mark: Some(format!("<smd:encodedSignedMark xmlns:smd=\"urn:ietf:params:xml:ns:signedMark-1.0\">{}</smd:encodedSignedMark>", test_smd)),
                create_type: epp_proxy::client::launch::LaunchCreateType::Registration,
                notices: vec![],
                core_nic: vec![],
            }),
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Register an Early Access domain
    info!("Create early access domain");
    info!("EAP domain: {}", eap_domain);
    let eap_check = epp_proxy::client::domain::check(&eap_domain, None, None, None, &mut cmd_tx_1)
        .await
        .unwrap();
    let eap_fee = eap_check
        .response
        .donuts_fee_check
        .unwrap()
        .sets
        .into_iter()
        .find(|s| s.category.name == Some("earlyAccess".to_string()))
        .unwrap();
    let eap_fee_command = eap_fee
        .fees
        .into_iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Create)
        .unwrap();

    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &eap_domain,
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
            donuts_fee_agreement: Some(epp_proxy::client::fee::DonutsFeeData {
                sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                    category: eap_fee.category,
                    fee_type: eap_fee.fee_type,
                    fees: vec![epp_proxy::client::fee::DonutsAmount {
                        value: eap_fee_command.value,
                        command: eap_fee_command.command,
                        command_name: eap_fee_command.command_name,
                    }],
                }],
            }),
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Register a Premium domain name for 3 separate premium price points
    info!("Create 3 premium domains");
    let premium_check_1 =
        epp_proxy::client::domain::check(premium_domain_1, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap();
    let premium_fee_1 = premium_check_1
        .response
        .donuts_fee_check
        .unwrap()
        .sets
        .into_iter()
        .find(|s| s.category.name == Some("premium".to_string()))
        .unwrap();
    let premium_fee_command_1 = premium_fee_1
        .fees
        .iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Create)
        .unwrap();
    let premium_fee_transfer_command_1 = premium_fee_1
        .fees
        .iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Transfer)
        .unwrap();

    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: premium_domain_1,
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
            donuts_fee_agreement: Some(epp_proxy::client::fee::DonutsFeeData {
                sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                    category: premium_fee_1.category.clone(),
                    fee_type: premium_fee_1.fee_type.clone(),
                    fees: vec![epp_proxy::client::fee::DonutsAmount {
                        value: premium_fee_command_1.value.clone(),
                        command: premium_fee_command_1.command,
                        command_name: premium_fee_command_1.command_name.clone(),
                    }],
                }],
            }),
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    let premium_check_2 =
        epp_proxy::client::domain::check(premium_domain_2, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap();
    let premium_fee_2 = premium_check_2
        .response
        .donuts_fee_check
        .unwrap()
        .sets
        .into_iter()
        .find(|s| s.category.name == Some("premium".to_string()))
        .unwrap();
    let premium_fee_command_2 = premium_fee_2
        .fees
        .iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Create)
        .unwrap();
    let premium_fee_renew_command_2 = premium_fee_2
        .fees
        .iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Renew)
        .unwrap();

    let premium_domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: premium_domain_2,
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
            donuts_fee_agreement: Some(epp_proxy::client::fee::DonutsFeeData {
                sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                    category: premium_fee_2.category.clone(),
                    fee_type: premium_fee_2.fee_type.clone(),
                    fees: vec![epp_proxy::client::fee::DonutsAmount {
                        value: premium_fee_command_2.value.clone(),
                        command: premium_fee_command_2.command,
                        command_name: premium_fee_command_2.command_name.clone(),
                    }],
                }],
            }),
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    let premium_check_3 =
        epp_proxy::client::domain::check(premium_domain_3, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap();
    let premium_fee_3 = premium_check_3
        .response
        .donuts_fee_check
        .unwrap()
        .sets
        .into_iter()
        .find(|s| s.category.name == Some("premium".to_string()))
        .unwrap();
    let premium_fee_command_3 = premium_fee_3
        .fees
        .into_iter()
        .find(|f| f.command == epp_proxy::client::fee::Command::Create)
        .unwrap();

    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: premium_domain_3,
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
            donuts_fee_agreement: Some(epp_proxy::client::fee::DonutsFeeData {
                sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                    category: premium_fee_3.category,
                    fee_type: premium_fee_3.fee_type,
                    fees: vec![epp_proxy::client::fee::DonutsAmount {
                        value: premium_fee_command_3.value,
                        command: premium_fee_command_3.command,
                        command_name: premium_fee_command_3.command_name,
                    }],
                }],
            }),
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Transfer a Premium name from your OTE1 account to your OTE2 account
    info!("Transferring premium domain");
    epp_proxy::client::domain::transfer_request(
        premium_domain_1,
        None,
        "test_auth1",
        None,
        Some(epp_proxy::client::fee::DonutsFeeData {
            sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                category: premium_fee_1.category,
                fee_type: premium_fee_1.fee_type,
                fees: vec![epp_proxy::client::fee::DonutsAmount {
                    value: premium_fee_transfer_command_1.value.clone(),
                    command: premium_fee_transfer_command_1.command,
                    command_name: premium_fee_transfer_command_1.command_name.clone(),
                }],
            }],
        }),
        None,
        None,
        &mut cmd_tx_2,
    )
    .await
    .unwrap();

    epp_proxy::client::domain::transfer_accept(premium_domain_1, None, &mut cmd_tx_1)
        .await
        .unwrap();

    // Renew a Premium name
    info!("Renewing premium domain");
    epp_proxy::client::domain::renew(
        premium_domain_2,
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 2,
        }),
        premium_domain_create_res
            .response
            .data
            .expiration_date
            .unwrap(),
        None,
        Some(epp_proxy::client::fee::DonutsFeeData {
            sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                category: premium_fee_2.category.clone(),
                fee_type: premium_fee_2.fee_type.clone(),
                fees: vec![epp_proxy::client::fee::DonutsAmount {
                    value: premium_fee_renew_command_2.value.clone(),
                    command: premium_fee_renew_command_2.command,
                    command_name: premium_fee_renew_command_2.command_name.clone(),
                }],
            }],
        }),
        None,
        None,
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    // Delete and Restore a Premium name
    info!("Deleting and restoring premiun name");
    let premium_check_dr =
        epp_proxy::client::domain::check(premium_domain_dr, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap();
    let premium_fee_dr = premium_check_dr
        .response
        .donuts_fee_check
        .unwrap()
        .sets
        .into_iter()
        .find(|s| s.category.name == Some("premium".to_string()))
        .unwrap();
    let premium_fee_restore_command_dr = premium_fee_2
        .fees
        .into_iter()
        .find(|f| {
            f.command == epp_proxy::client::fee::Command::Update
                && f.command_name == Some("restore".to_string())
        })
        .unwrap();

    epp_proxy::client::domain::delete(premium_domain_dr, None, None, None, None, &mut cmd_tx_1)
        .await
        .unwrap();

    epp_proxy::client::rgp::request(
        premium_domain_dr,
        Some(epp_proxy::client::fee::DonutsFeeData {
            sets: vec![epp_proxy::client::fee::DonutsFeeSet {
                category: premium_fee_dr.category,
                fee_type: premium_fee_dr.fee_type,
                fees: vec![epp_proxy::client::fee::DonutsAmount {
                    value: premium_fee_restore_command_dr.value.clone(),
                    command: premium_fee_restore_command_dr.command,
                    command_name: premium_fee_restore_command_dr.command_name.clone(),
                }],
            }],
        }),
        &mut cmd_tx_1,
    )
    .await
    .unwrap();

    info!("Logging out of accounts");
    let final_cmd_1 = epp_proxy::client::logout(cmd_tx_1).await.unwrap();
    let final_cmd_2 = epp_proxy::client::logout(cmd_tx_2).await.unwrap();

    println!(
        "Final command transaction: {:#?}",
        final_cmd_1.transaction_id
    );
    println!(
        "Final command transaction: {:#?}",
        final_cmd_2.transaction_id
    );
}
