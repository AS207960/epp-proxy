#[macro_use]
extern crate log;

use chrono::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::Command::new("pir-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the PIR EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("acct")
                .short('a')
                .long("account")
                .takes_value(true)
                .required(true)
                .help("Config file for the EPP account"),
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

    let log_dir_path: &std::path::Path = matches.value_of("log").unwrap().as_ref();
    match std::fs::create_dir_all(&log_dir_path) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory: {}", e);
            return;
        }
    }

    let conf_file_path = matches.value_of("acct").unwrap();

    let conf_file = match std::fs::File::open(conf_file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Can't open config file {}: {}", conf_file_path, e);
            return;
        }
    };

    let mut conf: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file) {
        Ok(c) => c,
        Err(e) => {
            error!("Can't parse config file {}: {}", conf_file_path, e);
            return;
        }
    };

    let log_dir = log_dir_path.join(&conf.id);
    match std::fs::create_dir_all(&log_dir) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory for {}: {}", conf.id, e);
            return;
        }
    }

    conf.errata = Some("pir".to_string());
    conf.tag = "ClientX".to_string();
    conf.password = "foo-BAR2#123".to_string();
    conf.new_password = None;

    let epp_client = epp_proxy::create_client(log_dir.clone(), &conf, &pkcs11_engine, true).await;

    // 2.2.2 Authentication
    let (cmd_tx, mut ready_rx) = epp_client.start();

    info!("Awaiting client to become ready...");
    let login_trans_id = ready_rx.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // 2.2.3 Change Password
    info!("Changing password");

    info!("Logging out of account");
    let final_cmd = epp_proxy::client::logout(cmd_tx).await.unwrap();
    println!("Final command transaction: {:#?}", final_cmd.transaction_id);

    conf.new_password = Some("bar-FOO2#123".to_string());

    let epp_client = epp_proxy::create_client(log_dir, &conf, &pkcs11_engine, true).await;
    let (mut cmd_tx, mut ready_rx) = epp_client.start();

    info!("Awaiting client to become ready...");
    let login_trans_id = ready_rx.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // 2.3.1.1 Check Contact OTE-C1 (Contact Available)
    info!("Checking contact available");
    epp_proxy::client::contact::check("OTE-C1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.2 Create Contact OTE-C1
    info!("Creating contact");
    epp_proxy::client::contact::create(
        "OTE-C1",
        epp_proxy::client::contact::NewContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "John Doe".to_string(),
                organisation: Some("Example Corp. Inc".to_string()),
                streets: vec!["123 Example St.".to_string(), "Suite 100".to_string()],
                city: "Anytown".to_string(),
                province: Some("Any Prov".to_string()),
                postal_code: Some("A1A1A1".to_string()),
                country_code: "CA".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            internationalised_address: None,
            phone: Some(epp_proxy::client::Phone {
                number: "+1.4165555555".to_string(),
                extension: Some("1111".to_string()),
            }),
            fax: Some(epp_proxy::client::Phone {
                number: "+1.4165555556".to_string(),
                extension: None,
            }),
            email: "jdoe@test.test".to_string(),
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: "my_secret1".to_string(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.3 Check Contact (Contact Not Available)
    info!("Checking contact not available");
    epp_proxy::client::contact::check("OTE-C1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.4 Query Contact OTE-C1
    info!("Getting contact info");
    epp_proxy::client::contact::info("OTE-C1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.5 Check Contact OTE-C2 (Contact Available)
    info!("Checking contact available");
    epp_proxy::client::contact::check("OTE-C2", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.6 Create Contact OTE-C2
    info!("Creating contact");
    epp_proxy::client::contact::create(
        "OTE-C2",
        epp_proxy::client::contact::NewContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "John Doe".to_string(),
                organisation: Some("Example Corp. Inc".to_string()),
                streets: vec!["123 Example St.".to_string(), "Suite 100".to_string()],
                city: "Anytown".to_string(),
                province: Some("Any Prov".to_string()),
                postal_code: Some("A1A1A1".to_string()),
                country_code: "CA".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            internationalised_address: None,
            phone: Some(epp_proxy::client::Phone {
                number: "+1.4165555555".to_string(),
                extension: Some("1111".to_string()),
            }),
            fax: Some(epp_proxy::client::Phone {
                number: "+1.4165555556".to_string(),
                extension: None,
            }),
            email: "jdoe@test.test".to_string(),
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: "my_secret1".to_string(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.7 Check Contact OTE-C3 (Contact Available)
    info!("Checking contact available");
    epp_proxy::client::contact::check("OTE-C3", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.8 Create Contact OTE-C3
    info!("Creating contact");
    epp_proxy::client::contact::create(
        "OTE-C3",
        epp_proxy::client::contact::NewContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "John Doe".to_string(),
                organisation: Some("Example Corp. Inc".to_string()),
                streets: vec!["123 Example St.".to_string(), "Suite 100".to_string()],
                city: "Anytown".to_string(),
                province: Some("Any Prov".to_string()),
                postal_code: Some("A1A1A1".to_string()),
                country_code: "CA".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            internationalised_address: None,
            phone: Some(epp_proxy::client::Phone {
                number: "+1.4165555555".to_string(),
                extension: Some("1111".to_string()),
            }),
            fax: Some(epp_proxy::client::Phone {
                number: "+1.4165555556".to_string(),
                extension: None,
            }),
            email: "jdoe@test.test".to_string(),
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: "my_secret1".to_string(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.9 Check Contact OTE-C4 (Contact Available)
    info!("Checking contact available");
    epp_proxy::client::contact::check("OTE-C4", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.10 Create Contact OTE-C4
    info!("Creating contact");
    epp_proxy::client::contact::create(
        "OTE-C4",
        epp_proxy::client::contact::NewContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "John Doe".to_string(),
                organisation: Some("Example Corp. Inc".to_string()),
                streets: vec!["123 Example St.".to_string(), "Suite 100".to_string()],
                city: "Anytown".to_string(),
                province: Some("Any Prov".to_string()),
                postal_code: Some("A1A1A1".to_string()),
                country_code: "CA".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            internationalised_address: None,
            phone: Some(epp_proxy::client::Phone {
                number: "+1.4165555555".to_string(),
                extension: Some("1111".to_string()),
            }),
            fax: Some(epp_proxy::client::Phone {
                number: "+1.4165555556".to_string(),
                extension: None,
            }),
            email: "jdoe@test.test".to_string(),
            entity_type: None,
            trading_name: None,
            company_number: None,
            disclosure: None,
            auth_info: "my_secret1".to_string(),
            eurid_info: None,
            isnic_info: None,
            qualified_lawyer: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.11 Update Contact (Change Element)
    info!("Updating contact");
    epp_proxy::client::contact::update(
        "OTE-C3",
        vec![],
        vec![],
        epp_proxy::client::contact::UpdateContactData {
            local_address: Some(epp_proxy::client::contact::Address {
                name: "Jane Smith".to_string(),
                organisation: Some("Example Corp. Inc".to_string()),
                streets: vec!["123 Example St.".to_string(), "Suite 100".to_string()],
                city: "Anytown".to_string(),
                province: Some("Any Prov".to_string()),
                postal_code: Some("A1A1A1".to_string()),
                country_code: "CA".to_string(),
                identity_number: None,
                birth_date: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.12 Update Contact (Remove Element)
    info!("Updating contact");
    epp_proxy::client::contact::update(
        "OTE-C3",
        vec![],
        vec![],
        epp_proxy::client::contact::UpdateContactData {
            fax: Some(epp_proxy::client::Phone {
                number: "".to_string(),
                extension: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.13 Update Contact (Add Element)
    info!("Updating contact");
    epp_proxy::client::contact::update(
        "OTE-C3",
        vec![],
        vec![],
        epp_proxy::client::contact::UpdateContactData {
            fax: Some(epp_proxy::client::Phone {
                number: "+1.4165555556".to_string(),
                extension: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.14 Check Name Server (Foreign Registry - Available)
    info!("Checking nameserver");
    epp_proxy::client::host::check("ns1.example.com", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.15 Create Name Server (Foreign Registry)
    info!("Creating nameserver");
    epp_proxy::client::host::create("ns1.example.com", vec![], None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.16 Check Name Server (Foreign Registry - Available)
    info!("Checking nameserver");
    epp_proxy::client::host::check("ns2.example.com", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.17 Create Name Server (Foreign Registry)
    info!("Creating nameserver");
    epp_proxy::client::host::create("ns2.example.com", vec![], None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.18 Check Domain (Domain Available for Registration)
    info!("Checking domain");
    epp_proxy::client::domain::check("example.org", None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.19 Create domain
    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "example.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 1,
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string(),
                },
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.com".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.com".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.20 Check Domain (Domain Not Available for Registration)
    info!("Checking domain not available");
    epp_proxy::client::domain::check("example.org", None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.21 Query Domain
    info!("Querying domain");
    epp_proxy::client::domain::info("example.org", None, None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.22 Check Name Server (Available)
    info!("Checking nameserver");
    epp_proxy::client::host::check("ns1.example.org", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.23 Create Name Server
    info!("Creating nameserver");
    epp_proxy::client::host::create(
        "ns1.example.org",
        vec![epp_proxy::client::host::Address {
            address: "203.171.1.93".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv4,
        }],
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.24 Check Name Server (Unavailable)
    info!("Checking nameserver unavailable");
    epp_proxy::client::host::check("ns1.example.org", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.25 Query Name Server
    info!("Querying nameserver");
    epp_proxy::client::host::info("ns1.example.org", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.26 Check Name Server (Available)
    info!("Checking nameserver");
    epp_proxy::client::host::check("ns2.example.org", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.27 Create Name Server
    info!("Creating nameserver");
    epp_proxy::client::host::create(
        "ns2.example.org",
        vec![epp_proxy::client::host::Address {
            address: "203.171.1.94".to_string(),
            ip_version: epp_proxy::client::host::AddressVersion::IPv4,
        }],
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.28 Update Name Server (Add IP Address)
    info!("Updating nameserver");
    epp_proxy::client::host::update(
        "ns2.example.org",
        vec![epp_proxy::client::host::UpdateObject::Address(
            epp_proxy::client::host::Address {
                address: "203.171.1.95".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            },
        )],
        vec![],
        None,
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.29 Update Name Server (Remove IP Address)
    info!("Updating nameserver");
    epp_proxy::client::host::update(
        "ns2.example.org",
        vec![],
        vec![epp_proxy::client::host::UpdateObject::Address(
            epp_proxy::client::host::Address {
                address: "203.171.1.95".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            },
        )],
        None,
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.30 Check Domain (Domain Available for Registration)
    info!("Checking domain");
    epp_proxy::client::domain::check("domain.org", None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.1.19 Create domain
    info!("Creating domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "domain.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 1,
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string(),
                },
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.org".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.org".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.32 Query Domain
    info!("Querying domain");
    let domain_info =
        epp_proxy::client::domain::info("domain.org", None, None, None, None, &mut cmd_tx)
            .await
            .unwrap();

    // 2.3.1.33 Renew Domain
    info!("Renewing domain");
    epp_proxy::client::domain::renew(
        "domain.org",
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 3,
        }),
        domain_info.response.expiry_date.unwrap(),
        None,
        None,
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.34 Update Domain – Change Name Servers
    info!("Updating domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "domain.org",
            add: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(
                        "ns1.example.com".to_string(),
                    ),
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(
                        "ns1.example.com".to_string(),
                    ),
                ),
            ],
            remove: vec![
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(
                        "ns1.example.org".to_string(),
                    ),
                ),
                epp_proxy::client::domain::UpdateObject::Nameserver(
                    epp_proxy::client::domain::InfoNameserver::HostOnly(
                        "ns2.example.org".to_string(),
                    ),
                ),
            ],
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.35 Update Domain - Change Contact
    info!("Updating domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "domain.org",
            add: vec![epp_proxy::client::domain::UpdateObject::Contact(
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C4".to_string(),
                },
            )],
            remove: vec![epp_proxy::client::domain::UpdateObject::Contact(
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string(),
                },
            )],
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.36 Update Domain – Change Authorization Information
    info!("Updating domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "domain.org",
            new_auth_info: Some("new_secret1"),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.1.37 Update Domain - Change Domain Status
    info!("Updating domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "domain.org",
            add: vec![epp_proxy::client::domain::UpdateObject::Status(
                epp_proxy::client::domain::Status::ClientUpdateProhibited,
            )],
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.2.1 Contact Transfer Request
    info!("Requesting contact transfer");
    epp_proxy::client::contact::transfer_request("OTE-C5", "my_secret1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.2.2 Query Contact Transfer
    info!("Querying contact transfer");
    epp_proxy::client::contact::transfer_query("OTE-C5", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.2.3 Approve Contact Transfer
    info!("Approving contact transfer");
    epp_proxy::client::contact::transfer_accept("OTE-C6", "my_secret1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.2.4 Reject Contact Transfer
    info!("Rejecting contact transfer");
    epp_proxy::client::contact::transfer_reject("OTE-C7", "my_secret1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.2.5 Domain Transfer Request
    info!("Requesting domain transfer");
    epp_proxy::client::domain::transfer_request(
        "transfer3.org",
        None,
        "my_secret1Y",
        None,
        None,
        None,
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.3.2.6 Approve Domain Transfer
    info!("Approving domain transfer");
    epp_proxy::client::domain::transfer_query("transfer2.org", Some("my_secret1X"), &mut cmd_tx)
        .await
        .unwrap();
    epp_proxy::client::domain::transfer_accept("transfer2.org", Some("my_secret1X"), &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.2.7 Reject Domain Transfer
    info!("Rejecting domain transfer");
    epp_proxy::client::domain::transfer_reject("transfer1.org", Some("my_secret1X"), &mut cmd_tx)
        .await
        .unwrap();

    // 2.3.3.1 Correctly Handle 2003 Exception
    info!("Causing 2003 error");
    assert!(epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "exception.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 1
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string()
                }
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.org".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.org".to_string()),
            ],
            auth_info: "",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.3.3.2 Correctly Handle 2005 Exception
    info!("Causing 2005 error");
    assert!(epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "-*invalid.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 1
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string()
                }
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.org".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.org".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.3.3.3 Correctly Handle 2306 Exception
    info!("Causing 2306 error");
    assert!(epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "exception.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 99
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string()
                }
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.org".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.org".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.3.3.4 Correctly Handle 2002 Exception
    info!("Causing 2002 error");
    assert!(epp_proxy::client::domain::renew(
        "example.org",
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 1
        }),
        Utc.ymd(2011, 6, 21).and_hms(0, 0, 0),
        None,
        None,
        None,
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.3.3.5 Correctly Handle 2303 Exception
    info!("Causing 2303 error");
    assert!(epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "exception.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2
            }),
            registrant: "OTE-C99",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string()
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string()
                }
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.org".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.org".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: None,
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.3.3.6 Correctly Handle 2305 Exception
    info!("Causing 2305 error");
    assert!(epp_proxy::client::contact::delete("OTE-C2", &mut cmd_tx)
        .await
        .is_err());

    // 2.3.3.7 Correctly Handle 2201 Exception
    info!("Causing 2201 error");
    assert!(
        epp_proxy::client::domain::delete("transfer3.org", None, None, None, &mut cmd_tx)
            .await
            .is_err()
    );

    // 2.4.1.1 Check Domain (Domain Available for Registration)
    info!("Checking DNSSEC domain");
    epp_proxy::client::domain::check("dsdomain1.org", None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.4.1.2 Create Domain with DS Record
    info!("Creating DNSSEC domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "dsdomain1.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 5,
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string(),
                },
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.com".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.com".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12345,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "49FD46E6C4B45C55D4AC49FD46E6C4B45C55D4AC".to_string(),
                        key_data: None,
                    },
                ]),
            }),
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.3 Create Domain with multiple DS Records
    info!("Creating DNSSEC domain");
    epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: "dsdomain2.org",
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 5,
            }),
            registrant: "OTE-C1",
            contacts: vec![
                epp_proxy::client::domain::InfoContact {
                    contact_type: "admin".to_string(),
                    contact_id: "OTE-C2".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "billing".to_string(),
                    contact_id: "OTE-C3".to_string(),
                },
                epp_proxy::client::domain::InfoContact {
                    contact_type: "tech".to_string(),
                    contact_id: "OTE-C4".to_string(),
                },
            ],
            nameservers: vec![
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns1.example.com".to_string()),
                epp_proxy::client::domain::InfoNameserver::HostOnly("ns2.example.com".to_string()),
            ],
            auth_info: "my_secret1",
            sec_dns: Some(epp_proxy::client::domain::SecDNSData {
                max_sig_life: None,
                data: epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12346,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "49FD46E6C4B45C55D4AC49FD46E6C4B45C55D4AD".to_string(),
                        key_data: None,
                    },
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12344,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "49FC66E6C4B45C56D4AC49FD46E6C4B45C55D4AE".to_string(),
                        key_data: None,
                    },
                ]),
            }),
            launch_create: None,
            fee_agreement: None,
            donuts_fee_agreement: None,
            eurid_data: None,
            isnic_payment: None,
            personal_registration: None,
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.4 Query domain that has DS Data
    info!("Querying DNSSEC domain");
    epp_proxy::client::domain::info("dsdomain1.org", None, None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.4.1.5 Update Domain- Adding Single DS Data
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: None,
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12348,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "38EC35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                ])),
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.6 Update Domain – Changing DS Data
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12348,
                            algorithm: 3,
                            digest_type: 1,
                            digest: "38EC35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                            key_data: None,
                        },
                    ]),
                )),
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12349,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "38EF35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                ])),
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.7 Update Domain – Adding Multiple DS Records
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: None,
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12350,
                        algorithm: 4,
                        digest_type: 1,
                        digest: "38AB35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12351,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "38AA35D5B3A34B44C39B38EC35D5B3A34B44C39C".to_string(),
                        key_data: None,
                    },
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12352,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "38AC35D5B3A34B44C39B38EC35D5B3A34B44C39D".to_string(),
                        key_data: None,
                    },
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12353,
                        algorithm: 4,
                        digest_type: 2,
                        digest: "651463E06F19D2FCA0215F129F54A2E0A4771EBBA37D8AB1103BCD279F0719E6"
                            .to_string(),
                        key_data: None,
                    },
                ])),
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.8 Update Domain – Remove Multiple DS Records
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12350,
                            algorithm: 4,
                            digest_type: 1,
                            digest: "38AB35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                            key_data: None,
                        },
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12351,
                            algorithm: 3,
                            digest_type: 1,
                            digest: "38AA35D5B3A34B44C39B38EC35D5B3A34B44C39C".to_string(),
                            key_data: None,
                        },
                    ]),
                )),
                add: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.9 Update Domain – Remove Single DS Record (Update: Remove)
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "dsdomain1.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12345,
                            algorithm: 3,
                            digest_type: 1,
                            digest: "49FD46E6C4B45C55D4AC49FD46E6C4B45C55D4AC".to_string(),
                            key_data: None,
                        },
                    ]),
                )),
                add: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.10 Update Domain – Adding and Removing Multiple DS Records
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12352,
                            algorithm: 3,
                            digest_type: 1,
                            digest: "38AC35D5B3A34B44C39B38EC35D5B3A34B44C39D".to_string(),
                            key_data: None,
                        },
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 12353,
                            algorithm: 4,
                            digest_type: 2,
                            digest:
                                "651463E06F19D2FCA0215F129F54A2E0A4771EBBA37D8AB1103BCD279F0719E6"
                                    .to_string(),
                            key_data: None,
                        },
                    ]),
                )),
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12350,
                        algorithm: 4,
                        digest_type: 1,
                        digest: "38AB35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12351,
                        algorithm: 3,
                        digest_type: 1,
                        digest: "38AA35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                ])),
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.1.11 Update Domain – Remove All DS Records
    info!("Updating DNSSEC domain");
    epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                new_max_sig_life: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::All(true)),
                add: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .unwrap();

    // 2.4.2.1 Correctly Handle 2306 Error Exception
    info!("Causing 2306 error");
    assert!(epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12350,
                        // Should be 300 but the value is a u8 so not allowed, hopefully this also causes the error
                        algorithm: 255,
                        digest_type: 255,
                        digest: "38AB35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                        key_data: None,
                    },
                ])),
                remove: None,
                new_max_sig_life: None,
            }),
            ..Default::default()
        },
        &mut cmd_tx,
    )
    .await
    .is_err());

    // 2.4.2.2 Correctly Handle 2303 Error Exception (Remove Single DS Record)
    info!("Causing 2303 error");
    assert!(epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                remove: Some(epp_proxy::client::domain::UpdateSecDNSRemove::Data(
                    epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                        epp_proxy::client::domain::SecDNSDSData {
                            key_tag: 54321,
                            algorithm: 3,
                            digest_type: 1,
                            digest: "38AB35D5B3A34B44C39B38EC35D5B3A34B44C39B".to_string(),
                            key_data: None,
                        },
                    ])
                )),
                add: None,
                new_max_sig_life: None
            }),
            ..Default::default()
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.4.2.3 Correctly Handle 2005 Error Exception (Adding Digest with space in between)
    info!("Causing 2005 error");
    assert!(epp_proxy::client::domain::update(
        epp_proxy::client::domain::UpdateInfo {
            domain: "example.org",
            sec_dns: Some(epp_proxy::client::domain::UpdateSecDNS {
                urgent: None,
                add: Some(epp_proxy::client::domain::SecDNSDataType::DSData(vec![
                    epp_proxy::client::domain::SecDNSDSData {
                        key_tag: 12355,
                        algorithm: 4,
                        digest_type: 2,
                        digest: "C06D93103F046E056033CA1D47CCD31F60DC7CE8E1BF C381A1252879C98752EE"
                            .to_string(),
                        key_data: None,
                    },
                ])),
                remove: None,
                new_max_sig_life: None
            }),
            ..Default::default()
        },
        &mut cmd_tx
    )
    .await
    .is_err());

    // 2.4.3.1 Delete a Domain (dsdomain1.org)
    info!("Deleting DNSSEC domain");
    epp_proxy::client::domain::delete("dsdomain1.org", None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.4.3.2 Delete a Domain (dsdomain2.org)
    info!("Deleting DNSSEC domain");
    epp_proxy::client::domain::delete("dsdomain2.org", None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.1 Delete Domain (example.org)
    info!("Deleting domain");
    epp_proxy::client::domain::delete("example.org", None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.2 Delete Domain (domain.org)
    info!("Deleting domain");
    epp_proxy::client::domain::delete("domain.org", None, None, None, &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.3 Delete Contact (OTE-C1)
    info!("Deleting contact");
    epp_proxy::client::contact::delete("OTE-C1", &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.4 Delete Contact (OTE-C2)
    info!("Deleting contact");
    epp_proxy::client::contact::delete("OTE-C2", &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.5 Delete Contact (OTE-C3)
    info!("Deleting contact");
    epp_proxy::client::contact::delete("OTE-C3", &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.6 Delete Contact (OTE-C4)
    info!("Deleting contact");
    epp_proxy::client::contact::delete("OTE-C4", &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.7 Delete Name Server (ns1.example.com)
    info!("Deleting host");
    epp_proxy::client::host::delete("ns1.example.com", &mut cmd_tx)
        .await
        .unwrap();

    // 2.6.8 Delete Name Server (ns2.example.com)
    info!("Deleting host");
    epp_proxy::client::host::delete("ns2.example.com", &mut cmd_tx)
        .await
        .unwrap();

    // 2.7.1 Keep Session Alive
    info!("Waiting 30 mins");
    tokio::time::sleep(std::time::Duration::from_secs(30 * 60)).await;

    // 2.7.2 Request Message Queue Information
    info!("Requesting first message from queue");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx).await.unwrap();

    // 2.7.3 Ack Queued Message
    info!("Acking message");
    epp_proxy::client::poll::poll_ack(&poll_msg.response.unwrap().id, &mut cmd_tx)
        .await
        .unwrap();

    // 2.8 End Session
    info!("Logging out");
    let final_cmd = epp_proxy::client::logout(cmd_tx).await.unwrap();
    println!("Final command transaction: {:#?}", final_cmd.transaction_id);
}
