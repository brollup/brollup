use crate::{
    communicative::peer::peer::PEER, communicative::tcp::client::TCPClient,
    transmutive::noist::dkg::directory::DKG_DIRECTORY, transmutive::noist::manager::DKG_MANAGER,
};
use colored::Colorize;

// dkg dirs
// dkg dir new (not supported)
// dkg dir <height> info
// dkg dir <height> sign <msg> (not supported)
// dkg dir <height> sync

pub async fn dkg_command(parts: Vec<&str>, coordinator: &PEER, dkg_manager: &mut DKG_MANAGER) {
    match parts.get(1) {
        Some(part) => match part.to_owned() {
            "dir" => match parts.get(2) {
                Some(part) => match part.to_owned() {
                    "new" => return eprintln!("Not supported for operator."),
                    _ => {
                        let height = match part.to_owned().parse::<u64>() {
                            Ok(height) => height,
                            Err(_) => return eprintln!("Invalid <height>."),
                        };

                        match parts.get(3) {
                            Some(part) => match part.to_owned() {
                                "info" => dir_height_info(dkg_manager, height).await,
                                "sign" => return eprintln!("Not supported for operator."),
                                "sync" => dir_height_sync(coordinator, dkg_manager, height).await,

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

async fn dir_height_sync(coordinator: &PEER, dkg_manager: &DKG_MANAGER, height: u64) {
    {
        let _dkg_manager = dkg_manager.lock().await;
        if let Some(_) = _dkg_manager.directory_by_height(height) {
            return eprintln!("Directory already exists.");
        }
    }

    let (setup, sessions) = match coordinator.sync_dkg_dir(height).await {
        Ok(tuple) => tuple,
        Err(_) => return eprintln!("Failed to sync with peer."),
    };

    let dkg_directory: DKG_DIRECTORY = {
        let mut _dkg_manager = dkg_manager.lock().await;
        if !_dkg_manager.insert_setup(&setup) {
            return eprintln!("Failed to initialize new directory.");
        }
        match _dkg_manager.directory_by_height(height) {
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

async fn dir_height_info(dkg_manager: &DKG_MANAGER, height: u64) {
    let _dkg_manager = dkg_manager.lock().await;

    let dkg_directory: DKG_DIRECTORY = match _dkg_manager.directory_by_height(height) {
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
    println!("Nonce height : {}", _dkg_directory.nonce_height());
    println!("Index pick   : {}", index_pick);
    println!("Setup        : ");

    _dkg_directory.setup().print();
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
