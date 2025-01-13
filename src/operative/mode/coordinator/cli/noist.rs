use crate::{vse_setup, NOIST_MANAGER, PEER_LIST};
use colored::Colorize;

// noist setup <no> run
// noist setup <no> print
// noist setups
pub async fn command(
    parts: Vec<&str>,
    operator_list: &PEER_LIST,
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
                "run" => setup_no_run(operator_list, noist_manager, no).await,
                _ => return eprintln!("Incorrect usage."),
            }
        }
        "setups" => setup_all_print(noist_manager).await,
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn setup_no_print(noist_manager: &NOIST_MANAGER, no: u64) {
    let _noist_manager = noist_manager.lock().await;

    let directory = match _noist_manager.directory(no) {
        Some(directory) => directory,
        None => return eprintln!("Setup not found."),
    };

    directory.setup().print();
}

async fn setup_no_run(operator_list: &PEER_LIST, noist_manager: &NOIST_MANAGER, no: u64) {
    match vse_setup::run(operator_list, noist_manager, no).await {
        Some(setup) => {
            println!(
                "{}",
                format!("VSE protocol #{} run with success and saved.", no).green()
            );
            setup.print();
        }
        None => return eprintln!("{}", format!("VSE protocol #{} failed.", no).red()),
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
