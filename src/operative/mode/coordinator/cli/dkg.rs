use crate::{dkgops::DKGOps, DKG_DIRECTORY, DKG_MANAGER, PEER_MANAGER};
use colored::Colorize;

// dkg dir new
// dkg dir <no>
// dkg dirs
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &mut DKG_MANAGER,
) {
    if parts.len() < 2 {
        return eprintln!("Incorrect usage.");
    }

    match parts[1] {
        "dir" => match parts[2] {
            "new" => dir_new_run(peer_manager, dkg_manager).await,
            _ => {
                let no = match parts[2].parse::<u64>() {
                    Ok(no) => no,
                    Err(_) => return eprintln!("Invalid <no>."),
                };
                dir_no_print(dkg_manager, no).await;
            }
        },

        "dirs" => dirs_print(dkg_manager).await,
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn dir_no_print(dkg_manager: &DKG_MANAGER, no: u64) {
    let _dkg_manager = dkg_manager.lock().await;

    let dkg_directory: DKG_DIRECTORY = match _dkg_manager.directory(no) {
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
    println!("Avb sessions : {}", _dkg_directory.available_sessions());
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

    println!("Printing DKG dirs..");
    for (dir_height, _) in dirs {
        println!("Dir height: {}", dir_height);
    }
}
