#[macro_use]
extern crate log;

use futures::StreamExt;

const ALPHABET: [char; 63] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::App::new("verisign-name-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the Verisign .name EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("acct")
                .short('a')
                .long("account")
                .takes_value(true)
                .required(true)
                .help("Config file for the account"),
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

    let conf: epp_proxy::ConfigFile = match serde_json::from_reader(conf_file) {
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

    let test_2ld = format!("{}.name", nanoid::nanoid!(16, &ALPHABET));
    let test_3ld = format!("test.{}.name", nanoid::nanoid!(16, &ALPHABET));
    let out_of_zone_ns = format!("ns1.{}.com", nanoid::nanoid!(16, &ALPHABET));
    let contact_id = nanoid::nanoid!(16, &ALPHABET);

    let epp_client = epp_proxy::create_client(log_dir, &conf, &pkcs11_engine, false).await;

    // 2.1.2.1 EPP login command
    let (mut cmd_tx, mut ready_rx) = epp_client.start();

    info!("Awaiting client to become ready...");
    let login_trans_id = ready_rx.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    // 2.1.2.2 Poll Request command
    info!("Polling 1 message");
    let poll_msg = epp_proxy::client::poll::poll(&mut cmd_tx).await.unwrap();
    info!("{:#?}", poll_msg);

    // 2.1.2.3 Poll Acknowledge command
    if let Some(res) = poll_msg.response {
        info!("======");
        info!("Acknowledging message");
        info!(
            "{:#?}",
            epp_proxy::client::poll::poll_ack(&res.id, &mut cmd_tx)
                .await
                .unwrap()
        );
    }

    // 2.1.2.4 Add Contact
    info!("======");
    info!("Creating contact");
    info!(
        "{:#?}",
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
            },
            &mut cmd_tx,
        )
        .await
        .unwrap()
    );

    // 2.1.2.5 Add Out of zone nameserver
    info!("======");
    info!("Creating out of zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::create(
            &out_of_zone_ns,
            vec![], None, &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.6 Check 2LD
    info!("======");
    info!("Checking second level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::check(&test_2ld, None, None, &mut cmd_tx)
        .await
        .unwrap()
    );

    // 2.1.2.7 Create 2LD
    info!("======");
    info!("Creating second level domain");
    let domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &test_2ld,
            nameservers: vec![],
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
            sec_dns: None,
        },
        &mut cmd_tx,
    )
        .await
        .unwrap();
    info!("{:#?}", domain_create_res);

    // 2.1.2.8 Add In zone nameserver
    info!("======");
    info!("Creating in zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::create(
            &format!("ns1.{}.name", &test_2ld),
            vec![epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }], None, &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.9 Delete In zone nameserver
    info!("======");
    info!("Deleting in zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::delete(
            &format!("ns1.{}.name", &test_2ld), &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.10 Renew 2LD
    info!("======");
    info!("Renewing second level domain");
    let renew_res = epp_proxy::client::domain::renew(
        &test_2ld,
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 2,
        }),
        domain_create_res.response.data.expiration_date.unwrap(),
        None,
        None,
        None,
        &mut cmd_tx,
    )
        .await
        .unwrap();
    info!("{:#?}", renew_res);

    // 2.1.2.11 Update 2LD
    info!("======");
    info!("Updating second level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain: &test_2ld,
                add: vec![
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientRenewProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientTransferProhibited
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
            &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.12 Info 2LD
    info!("======");
    info!("Getting second level domain info");
    info!(
        "{:#?}",
        epp_proxy::client::domain::info(&test_2ld, None, None, None, None, &mut cmd_tx)
            .await
            .unwrap()
    );

    // 2.1.2.13 Delete 2LD
    info!("======");
    info!("Deleting second level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::delete(&test_2ld, None, None, None, &mut cmd_tx)
            .await
            .unwrap()
    );

    // 2.1.2.14 Check 3LD
    info!("======");
    info!("Checking third level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::check(&test_3ld, None, None, &mut cmd_tx)
        .await
        .unwrap()
    );

    // 2.1.2.15 Create 3LD
    info!("======");
    info!("Creating third level domain");
    let domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain: &test_3ld,
            nameservers: vec![],
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
            sec_dns: None,
        },
        &mut cmd_tx,
    )
        .await
        .unwrap();
    info!("{:#?}", domain_create_res);

    // 2.1.2.16 Add In zone nameserver
    info!("======");
    info!("Creating in zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::create(
            &format!("ns1.{}.name", &test_3ld),
            vec![epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }], None, &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.17 Delete In zone nameserver
    info!("======");
    info!("Deleting in zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::delete(
            &format!("ns1.{}.name", &test_3ld), &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.18 Renew 3LD
    info!("======");
    info!("Renewing third level domain");
    let renew_res = epp_proxy::client::domain::renew(
        &test_3ld,
        Some(epp_proxy::client::Period {
            unit: epp_proxy::client::PeriodUnit::Years,
            value: 2,
        }),
        domain_create_res.response.data.expiration_date.unwrap(),
        None,
        None,
        None,
        &mut cmd_tx,
    )
        .await
        .unwrap();
    info!("{:#?}", renew_res);

    // 2.1.2.19 Update 3LD
    info!("======");
    info!("Updating third level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::update(
            epp_proxy::client::domain::UpdateInfo {
                domain: &test_3ld,
                add: vec![
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientRenewProhibited
                    ),
                    epp_proxy::client::domain::UpdateObject::Status(
                        epp_proxy::client::domain::Status::ClientTransferProhibited
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
            &mut cmd_tx
        )
        .await
        .unwrap()
    );

    // 2.1.2.20 Info 3LD
    info!("======");
    info!("Getting third level domain info");
    info!(
        "{:#?}",
        epp_proxy::client::domain::info(&test_3ld, None, None, None, None, &mut cmd_tx)
            .await
            .unwrap()
    );

    // 2.1.2.21 Delete 3LD
    info!("======");
    info!("Deleting third level domain");
    info!(
        "{:#?}",
        epp_proxy::client::domain::delete(&test_3ld, None, None, None, &mut cmd_tx)
            .await
            .unwrap()
    );

    // 2.1.2.25 Delete Contact
    info!("======");
    info!("Deleting contact");
    info!(
        "{:#?}",
        epp_proxy::client::contact::delete(&contact_id, &mut cmd_tx)
            .await
            .unwrap()
    );

    // 2.1.2.26 Delete Out of zone nameserver
    info!("======");
    info!("Deleting out of zone nameserver");
    info!(
        "{:#?}",
        epp_proxy::client::host::delete(&out_of_zone_ns, &mut cmd_tx)
            .await
            .unwrap()
    );

    let final_cmd = epp_proxy::client::logout(cmd_tx).await.unwrap();
    println!("Final command transaction: {:#?}", final_cmd.transaction_id);
}
