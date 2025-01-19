use crate::{DKG_DIRECTORY, DKG_MANAGER};

// dkg dir new (not supported)
// dkg dir <height> info
// dkg dir <height> sign <msg> (not supported)
// dkg dirs
pub async fn command(parts: Vec<&str>, dkg_manager: &mut DKG_MANAGER) {
    match parts.get(1) {
        Some(part) => match part.to_owned() {
            "dir" => match parts.get(2) {
                Some(part) => match part.to_owned() {
                    "new" => return eprintln!("Not available for the operator."),
                    _ => {
                        let height = match part.to_owned().parse::<u64>() {
                            Ok(height) => height,
                            Err(_) => return eprintln!("Invalid <height>."),
                        };

                        match parts.get(3) {
                            Some(part) => match part.to_owned() {
                                "info" => dir_height_info(dkg_manager, height).await,
                                "sign" => return eprintln!("Not available for the operator."),
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
    println!("Avb sessions : {}", _dkg_directory.available_sessions());
    println!("Index height : {}", _dkg_directory.index_height());
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
