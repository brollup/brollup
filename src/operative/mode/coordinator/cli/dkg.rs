use crate::{signatoryops::SignatoryOps, DKG_DIRECTORY, DKG_MANAGER, PEER_MANAGER};
use colored::Colorize;

// dkg setup run
// dkg setup <no>
// dkg setups
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &mut DKG_MANAGER,
) {
    if parts.len() < 3 {
        return eprintln!("Incorrect usage.");
    }

    match parts[1] {
        "setup" => match parts[2] {
            "run" => setup_run(peer_manager, dkg_manager).await,
            _ => {
                let no = match parts[2].parse::<u64>() {
                    Ok(no) => no,
                    Err(_) => return eprintln!("Invalid <no>."),
                };
                setup_no_print(dkg_manager, no).await;
            }
        },

        "setups" => setup_all_print(dkg_manager).await,
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn setup_no_print(dkg_manager: &DKG_MANAGER, no: u64) {
    let _dkg_manager = dkg_manager.lock().await;

    let dkg_directory: DKG_DIRECTORY = match _dkg_manager.directory(no) {
        Some(directory) => directory,
        None => return eprintln!("Setup not found."),
    };

    let _dkg_directory = dkg_directory.lock().await;
    _dkg_directory.setup().print();
}

async fn setup_run(peer_manager: &mut PEER_MANAGER, dkg_manager: &DKG_MANAGER) {
    match dkg_manager.coordinate_new_setup(peer_manager).await {
        Ok(setup_height) => {
            println!(
                "{}",
                format!("DKG protocol #{} run with success and saved.", setup_height).green()
            );
        }
        Err(err) => return eprintln!("{}", format!("DKG protocol failed: {:?}", err).red()),
    };
}

async fn setup_all_print(dkg_manager: &DKG_MANAGER) {
    let dirs = {
        let _dkg_manager = dkg_manager.lock().await;
        _dkg_manager.directories().clone()
    };

    for (setup_no, _) in dirs {
        println!("Setup no: {}", setup_no);
    }
}
