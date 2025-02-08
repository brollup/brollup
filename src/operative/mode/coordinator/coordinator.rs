use crate::dkgops::DKGOps;
use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::peer::PeerKind;
use crate::peer_manager::PeerManager;
use crate::session::SessionCtx;
use crate::tcp::tcp::open_port;
use crate::{baked, key::KeyHolder};
use crate::{ccli, nns, tcp, Network, OperatingMode, DKG_MANAGER, PEER_MANAGER, SESSION_CTX};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;

#[tokio::main]
pub async fn run(keys: KeyHolder, _network: Network) {
    let mode = OperatingMode::Coordinator;

    // 1. Check if this is a valid coordinator.
    if keys.public_key() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing coordinator..");

    // 2. Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // 3. Open port 6272 for incoming connections.
    match open_port().await {
        true => println!("{}", format!("Opened port '{}'.", baked::PORT).green()),
        false => (),
    }

    // 4. Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // 5. Initialize peer manager.
    let operator_set = baked::OPERATOR_SET.to_vec();
    let mut peer_manager: PEER_MANAGER =
        match PeerManager::new(&nns_client, PeerKind::Operator, &operator_set).await {
            Some(manager) => manager,
            None => return eprintln!("{}", "Error initializing Peer manager.".red()),
        };

    // 6. Initialize DKG Manager.
    let mut dkg_manager: DKG_MANAGER = match DKGManager::new() {
        Some(manager) => manager,
        None => return eprintln!("{}", "Error initializing DKG manager.".red()),
    };

    // 7. Run background preprocessing for the DKG Manager.
    dkg_manager.run_preprocessing(&mut peer_manager).await;

    let mut cov_session: SESSION_CTX = SessionCtx::construct();

    // 8. Run TCP server.
    {
        let nns_client = nns_client.clone();
        let dkg_manager = Arc::clone(&dkg_manager);
        let cov_session = Arc::clone(&cov_session);

        let _ = tokio::spawn(async move {
            let _ =
                tcp::server::run(mode, &nns_client, &keys, &dkg_manager, Some(cov_session)).await;
        });
    }

    // 9. CLI
    cli(&mut peer_manager, &mut dkg_manager, &mut cov_session).await;
}

pub async fn cli(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &mut DKG_MANAGER,
    cov_session: &mut SESSION_CTX,
) {
    println!(
        "{}",
        "Enter command (type help for options, type exit to quit):".cyan()
    );

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("{}", format!("Invalid line.").yellow());
                continue;
            }
        };

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            // Main commands:
            "exit" => break,
            "clear" => ccli::clear::command(),
            "dkg" => ccli::dkg::command(parts, peer_manager, dkg_manager).await,
            "ops" => ccli::ops::command(peer_manager).await,
            "covr" => ccli::covr::command(parts, peer_manager, dkg_manager, cov_session).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
