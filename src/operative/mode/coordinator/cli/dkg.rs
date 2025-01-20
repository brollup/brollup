use crate::{
    dkgops::DKGOps, peer::PeerKind, peer_manager::PeerManagerExt, tcp::client::TCPClient,
    DKG_DIRECTORY, DKG_MANAGER, PEER, PEER_MANAGER,
};
use colored::Colorize;
use std::time::{Duration, Instant};

// dkg dirs
// dkg dir new
// dkg dir <height> info
// dkg dir <height> sign <msg>
// dkg dir <height> sync <key>

pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &mut DKG_MANAGER,
) {
    match parts.get(1) {
        Some(part) => match part.to_owned() {
            "dir" => match parts.get(2) {
                Some(part) => match part.to_owned() {
                    "new" => dir_new_run(peer_manager, dkg_manager).await,
                    _ => {
                        let height = match part.to_owned().parse::<u64>() {
                            Ok(height) => height,
                            Err(_) => return eprintln!("Invalid <height>."),
                        };

                        match parts.get(3) {
                            Some(part) => match part.to_owned() {
                                "info" => dir_height_info(dkg_manager, height).await,
                                "sign" => match parts.get(4) {
                                    Some(part) => {
                                        let msg: [u8; 32] = match hex::decode(part.to_owned()) {
                                            Ok(bytes) => match bytes.try_into() {
                                                Ok(bytes) => bytes,
                                                Err(_) => return eprintln!("Invalid <msg>."),
                                            },
                                            Err(_) => return eprintln!("Invalid <msg>."),
                                        };

                                        dir_height_sign(peer_manager, dkg_manager, height, msg)
                                            .await;
                                    }
                                    None => return eprintln!("<msg> is missing."),
                                },
                                "sync" => match parts.get(4) {
                                    Some(part) => {
                                        let key: [u8; 32] = match hex::decode(part.to_owned()) {
                                            Ok(bytes) => match bytes.try_into() {
                                                Ok(bytes) => bytes,
                                                Err(_) => return eprintln!("Invalid <key>."),
                                            },
                                            Err(_) => return eprintln!("Invalid <key>."),
                                        };

                                        dir_height_sync(peer_manager, dkg_manager, height, key)
                                            .await;
                                    }
                                    None => return eprintln!("<key> is missing."),
                                },
                                _ => return eprintln!("Incorrect usage."),
                            },
                            None => return eprintln!("Incorrect usage."),
                        }
                    }
                },
                None => return eprintln!("Incorrect usage."),
            },

            "dirs" => dirs_print(dkg_manager).await,
            _ => return eprintln!("Incorrect usage."),
        },
        None => return eprintln!("Incorrect usage."),
    }
}

async fn dir_height_sync(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
    height: u64,
    peer_key: [u8; 32],
) {
    {
        let _dkg_manager = dkg_manager.lock().await;
        if let Some(_) = _dkg_manager.directory(height) {
            return eprintln!("Directory already exists.");
        }
    }

    let peer: PEER = {
        peer_manager
            .add_peers(PeerKind::Operator, &vec![peer_key])
            .await;

        let _peer_manager = peer_manager.lock().await;

        match _peer_manager.retrieve_peer(peer_key) {
            Some(peer) => peer,
            None => return eprintln!("Peer not found."),
        }
    };

    let (setup, sessions) = match peer.sync_dkg_dir(height).await {
        Ok(tuple) => tuple,
        Err(_) => return eprintln!("Failed to sync with peer."),
    };

    let dkg_directory: DKG_DIRECTORY = {
        let mut _dkg_manager = dkg_manager.lock().await;
        if !_dkg_manager.insert_setup(&setup) {
            return eprintln!("Failed to initialize new directory.");
        }
        match _dkg_manager.directory(height) {
            Some(dir) => dir,
            None => return eprintln!("Failed to return the new directory."),
        }
    };

    for session in sessions {
        let mut _dkg_directory = dkg_directory.lock().await;
        if !_dkg_directory.insert_session_filled(&session) {
            return eprintln!("insert_session_filled err.");
        }
    }

    println!(
        "{}",
        format!(
            "Succesfully syncronized DKG directory at height {}.",
            height
        )
        .green()
    );
}

async fn dir_height_sign(
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
    height: u64,
    msg: [u8; 32],
) {
    let start = Instant::now();

    match dkg_manager.sign(peer_manager, height, vec![msg]).await {
        Ok(sig) => {
            println!("Sig: {}", hex::encode(sig[0]).green());

            let elapsed: Duration = start.elapsed();
            println!("{}ms", elapsed.as_millis());

            tokio::time::sleep(Duration::from_millis(2_250)).await;
        }

        Err(err) => return eprintln!("Error signing: {:?}", err),
    }
}

async fn dir_height_info(dkg_manager: &DKG_MANAGER, height: u64) {
    let _dkg_manager = dkg_manager.lock().await;

    let dkg_directory: DKG_DIRECTORY = match _dkg_manager.directory(height) {
        Some(directory) => directory,
        None => return eprintln!("Setup not found."),
    };

    let _dkg_directory = dkg_directory.lock().await;

    let group_key = match _dkg_directory.group_key() {
        Some(point) => hex::encode(point.serialize_xonly()),
        None => "-".to_string(),
    };

    let index_pick = match _dkg_directory.pick_index() {
        Some(pick) => pick.to_string(),
        None => "-".to_string(),
    };

    println!("Group key    : {}", group_key);
    println!("DKG packages : {}", _dkg_directory.available_sessions());
    println!("Index height : {}", _dkg_directory.index_height());
    println!("Index pick   : {}", index_pick);
    println!("Setup        : ");

    _dkg_directory.setup().print();
}

async fn dir_new_run(peer_manager: &mut PEER_MANAGER, dkg_manager: &DKG_MANAGER) {
    match dkg_manager.new_setup(peer_manager).await {
        Ok(setup_height) => {
            println!(
                "{}",
                format!("DKG protocol #{} run with success and saved.", setup_height).green()
            );
        }
        Err(err) => return eprintln!("{}", format!("DKG protocol failed: {:?}", err).red()),
    };
}

async fn dirs_print(dkg_manager: &DKG_MANAGER) {
    let dirs = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.directories().clone()
    };

    match dirs.len() {
        0 => println!("None."),
        _ => {
            for (dir_height, _) in dirs {
                println!("DKG dir #{}", dir_height);
            }
        }
    }
}
