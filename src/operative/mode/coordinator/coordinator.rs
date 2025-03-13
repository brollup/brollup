use crate::blacklist::BlacklistDirectory;
use crate::dkgops::DKGOps;
use crate::nns::client::NNSClient;
use crate::noist::manager::DKGManager;
use crate::peer::PeerKind;
use crate::peer_manager::PeerManager;
use crate::rpc::bitcoin_rpc::validate_rpc;
use crate::rpcholder::RPCHolder;
use crate::session::ccontext::{CContextRunner, CSessionCtx};
use crate::tcp::tcp::open_port;
use crate::{baked, key::KeyHolder};
use crate::{
    ccli, nns, tcp, Network, OperatingMode, BLIST_DIRECTORY, CSESSION_CTX, DKG_MANAGER,
    PEER_MANAGER,
};
use colored::Colorize;
use std::io::{self, BufRead};
use std::sync::Arc;

#[tokio::main]
pub async fn run(keys: KeyHolder, network: Network, rpc_holder: RPCHolder) {
    let mode = OperatingMode::Coordinator;

    // #1 Validate Bitcoin RPC.
    if let Err(err) = validate_rpc(&rpc_holder, network).await {
        println!("{} {}", "Bitcoin RPC Error: ".red(), err);
        return;
    }

    // #2 Check if this is a valid coordinator.
    if keys.public_key().serialize_xonly() != baked::COORDINATOR_WELL_KNOWN {
        eprintln!("{}", "Coordinator <nsec> does not match.".red());
        return;
    }

    println!("{}", "Initializing coordinator..");

    // #3 Initialize NNS client.
    let nns_client = NNSClient::new(&keys).await;

    // #4 Open port 6272 for incoming connections.
    match open_port().await {
        true => println!("{}", format!("Opened port '{}'.", baked::PORT).green()),
        false => (),
    }

    // #5 Run NNS server.
    {
        let nns_client = nns_client.clone();
        let _ = tokio::spawn(async move {
            let _ = nns::server::run(&nns_client, mode).await;
        });
    }

    // #6 Initialize peer manager.
    let operator_set = baked::OPERATOR_SET.to_vec();
    let mut peer_manager: PEER_MANAGER =
        match PeerManager::new(&nns_client, PeerKind::Operator, &operator_set).await {
            Some(manager) => manager,
            None => return eprintln!("{}", "Error initializing Peer manager.".red()),
        };

    // #7 Initialize DKG Manager.
    let mut dkg_manager: DKG_MANAGER = match DKGManager::new() {
        Some(manager) => manager,
        None => return eprintln!("{}", "Error initializing DKG manager.".red()),
    };

    // #8 Run background preprocessing for the DKG Manager.
    dkg_manager.run_preprocessing(&mut peer_manager).await;

    // #9 Construct blacklist directory.
    let mut blacklist_dir: BLIST_DIRECTORY = match BlacklistDirectory::new() {
        Some(blacklist_dir) => blacklist_dir,
        None => {
            eprintln!(
                "{}",
                "Unexpected error: Failed to create blaming directory.".red()
            );
            return;
        }
    };

    // #10 Construct CSession.
    let csession: CSESSION_CTX =
        CSessionCtx::construct(&dkg_manager, &peer_manager, &blacklist_dir);

    // #11 Run CSession.
    {
        let csession = Arc::clone(&csession);
        let _ = tokio::spawn(async move {
            csession.run().await;
        });
    }

    // #12 Run TCP server.
    {
        let nns_client = nns_client.clone();
        let dkg_manager = Arc::clone(&dkg_manager);
        let csession = Arc::clone(&csession);

        let _ = tokio::spawn(async move {
            let _ = tcp::server::run(mode, &nns_client, &keys, &dkg_manager, Some(csession)).await;
        });
    }

    // #13 Initialize CLI
    cli(&mut peer_manager, &mut dkg_manager, &mut blacklist_dir).await;
}

pub async fn cli(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &mut DKG_MANAGER,
    blacklist_dir: &mut BLIST_DIRECTORY,
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
            "blist" => ccli::blist::command(parts, blacklist_dir).await,
            _ => eprintln!("{}", format!("Unknown commmand.").yellow()),
        }
    }
}
