#[macro_use]
extern crate log;

use chrono::Datelike;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    openssl::init();

    let matches = clap::Command::new("verisign-com-net-test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Test runner for the Verisign COM/NET EPP test")
        .author("Q of AS207960 <q@as207960.net>")
        .arg(
            clap::Arg::new("acct1")
                .short('1')
                .long("account_1")
                .value_name("FILE")
                .required(true)
                .help("Config file for the first account"),
        )
        .arg(
            clap::Arg::new("acct2")
                .short('2')
                .long("account_2")
                .value_name("FILE")
                .required(true)
                .help("Config file for the second account"),
        )
        .arg(
            clap::Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .required(true)
                .help("Domain to use for testing"),
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
    let domain = matches.get_one::<String>("domain").unwrap();

    let log_dir_path = matches.get_one::<std::path::PathBuf>("log").unwrap();
    match std::fs::create_dir_all(log_dir_path) {
        Ok(()) => {}
        Err(e) => {
            error!("Can't create log directory: {}", e);
            return;
        }
    }

    let conf_file_1_path = matches.get_one::<String>("acct1").unwrap();
    let conf_file_2_path = matches.get_one::<String>("acct2").unwrap();

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

    let epp_client_1 = epp_proxy::create_client(storage_1, &conf_1, &pkcs11_engine, false).await;
    let epp_client_2 = epp_proxy::create_client(storage_2, &conf_2, &pkcs11_engine, false).await;

    // Establish a session using the EPP Login SESSION command with your OT&E1 account
    let (mut cmd_tx_1, mut ready_rx_1) = epp_client_1.start();

    info!("Awaiting client 1 to become ready...");
    let login_trans_id = ready_rx_1.next().await.unwrap();
    info!("Login transaction ID: {:#?}", login_trans_id);

    info!("Finding available domain");
    // Using your OT&E1 account, perform a CHECK domain command until you find an available domain
    info!(
        "{:#?}",
        epp_proxy::client::domain::check(domain, None, None, None, &mut cmd_tx_1)
            .await
            .unwrap()
    );

    // Create a new domain name using the ADD domain command with your OT&E1 account logon, term of
    // registration should be 2 years
    info!("======");
    info!("Creating domain");
    let domain_create_res = epp_proxy::client::domain::create(
        epp_proxy::client::domain::CreateInfo {
            domain,
            nameservers: vec![],
            period: Some(epp_proxy::client::Period {
                unit: epp_proxy::client::PeriodUnit::Years,
                value: 2,
            }),
            auth_info: "test_auth1",
            registrant: "UNUSED",
            contacts: vec![],
            donuts_fee_agreement: None,
            eurid_data: None,
            fee_agreement: None,
            launch_create: None,
            isnic_payment: None,
            sec_dns: None,
            personal_registration: None,
            keysys: None,
            nominet_ext: None,
        },
        &mut cmd_tx_1,
    )
    .await
    .unwrap();
    info!("{:#?}", domain_create_res);

    // Create child nameserver 1 of the newly created domain using the ADD nameserver command with
    // your OT&E1 account logon

    info!("======");
    info!("Creating host 1");
    info!(
        "{:#?}",
        epp_proxy::client::host::create(
            &format!("ns1.{}", domain),
            vec![epp_proxy::client::host::Address {
                address: "1.1.1.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }],
            None,
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Create child name server 2 of the newly created domain using the ADD nameserver command with
    // your OT&E1 account logon

    info!("======");
    info!("Creating host 2");
    info!(
        "{:#?}",
        epp_proxy::client::host::create(
            &format!("ns2.{}", domain),
            vec![epp_proxy::client::host::Address {
                address: "1.0.0.1".to_string(),
                ip_version: epp_proxy::client::host::AddressVersion::IPv4,
            }],
            None,
            &mut cmd_tx_1
        )
        .await
        .unwrap()
    );

    // Update domain to attach the child nameservers to the newly created domain using the MOD
    // domain command with your OT&E1 account logon

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
                eurid_data: None,
                keysys: None,
                nominet_ext: None,
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
                eurid_data: None,
                keysys: None,
                nominet_ext: None
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
                eurid_data: None,
                keysys: None,
                nominet_ext: None
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
                eurid_data: None,
                keysys: None,
                nominet_ext: None,
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
