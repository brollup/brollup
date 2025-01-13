use crate::NOIST_MANAGER;
// noist setup <no> print
// noist setups
pub async fn command(parts: Vec<&str>, noist_manager: &mut NOIST_MANAGER) {
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

async fn setup_all_print(noist_manager: &NOIST_MANAGER) {
    let dirs = {
        let _noist_manager = noist_manager.lock().await;
        _noist_manager.directories().clone()
    };

    for (setup_no, _) in dirs {
        println!("Setup no: {}", setup_no);
    }
}
