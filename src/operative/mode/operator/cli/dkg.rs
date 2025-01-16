use crate::{DKG_DIRECTORY, DKG_MANAGER};

// dkg dir <no>
// dkg dirs
pub async fn command(parts: Vec<&str>, dkg_manager: &mut DKG_MANAGER) {
    if parts.len() < 3 {
        return eprintln!("Incorrect usage.");
    }

    match parts[1] {
        "dir" => {
            let no = match parts[2].parse::<u64>() {
                Ok(no) => no,
                Err(_) => return eprintln!("Invalid <no>."),
            };
            dir_no_print(dkg_manager, no).await;
        }

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
    _dkg_directory.setup().print();
}

async fn dirs_print(dkg_manager: &DKG_MANAGER) {
    let dirs = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.directories().clone()
    };

    for (dir_height, _) in dirs {
        println!("Dir height: {}", dir_height);
    }
}
