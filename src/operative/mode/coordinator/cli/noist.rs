use crate::{baked, noist_protocol, DKG_DIRECTORY, NOIST_MANAGER, PEER_MANAGER};
use colored::Colorize;

// noist setup <no> run
// noist setup <no> print
// noist setups
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    noist_manager: &mut NOIST_MANAGER,
) {
    if parts.len() < 3 {
        return eprintln!("Incorrect usage.");
    }

    match parts[1] {
        "setup" => {
            // Args len must be exactly 4.
            if parts.len() != 4 {
                return eprintln!("Incorrect usage.");
            }

            let no = match parts[2].parse::<u64>() {
                Ok(no) => no,
                Err(_) => return eprintln!("Invalid <no>."),
            };

            match parts[3] {
                "print" => setup_no_print(noist_manager, no).await,
                "run" => setup_no_run(peer_manager, noist_manager, no).await,
                _ => return eprintln!("Incorrect usage."),
            }
        }
        "setups" => setup_all_print(noist_manager).await,
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn setup_no_print(noist_manager: &NOIST_MANAGER, no: u64) {
    let _noist_manager = noist_manager.lock().await;

    let dkg_directory: DKG_DIRECTORY = match _noist_manager.directory(no) {
        Some(directory) => directory,
        None => return eprintln!("Setup not found."),
    };

    let _dkg_directory = dkg_directory.lock().await;
    _dkg_directory.setup().print();
}

async fn setup_no_run(peer_manager: &mut PEER_MANAGER, noist_manager: &NOIST_MANAGER, setup_no: u64) {
    let signatory_set = baked::OPERATOR_SET.to_vec();
    match noist_protocol::setup::run_setup(peer_manager, noist_manager, setup_no, &signatory_set)
        .await
    {
        Some(setup) => {
            println!(
                "{}",
                format!("VSE protocol #{} run with success and saved.", setup_no).green()
            );
            setup.print();
        }
        None => return eprintln!("{}", format!("VSE protocol #{} failed.", setup_no).red()),
    };
}

async fn setup_all_print(noist_manager: &NOIST_MANAGER) {
    let dirs = {
        let _noist_manager = noist_manager.lock().await;
        _noist_manager.directories().clone()
    };

    for (setup_no, _) in dirs {
        println!("Setup no: {}", setup_no);
    }
}
