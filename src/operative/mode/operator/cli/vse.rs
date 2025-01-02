use crate::{tcp::client::Client, PEER, SIGNATORY_DB, VSE_DIRECTORY};

// vse setup <no> print
// vse dir print
// vse dir fetch print
// vse dir fetch save
pub async fn command(
    parts: Vec<&str>,
    coordinator: &PEER,
    signatory_db: &SIGNATORY_DB,
    vse_directory: &mut VSE_DIRECTORY,
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
                "print" => setup_print(vse_directory, no).await,
                _ => return eprintln!("Incorrect usage."),
            }
        }
        "dir" => match parts[2] {
            "print" => {
                // Args len must be exactly 3.
                if parts.len() != 3 {
                    return eprintln!("Incorrect usage.");
                }
                dir_print(vse_directory).await;
            }
            "fetch" => {
                // Args len must be exactly 4.
                if parts.len() != 4 {
                    return eprintln!("Incorrect usage.");
                }

                match parts[3] {
                    "print" => dir_fetch_print(coordinator).await,
                    "save" => dir_fetch_save(coordinator, signatory_db, vse_directory).await,
                    _ => return eprintln!("Incorrect usage."),
                }
            }
            _ => return eprintln!("Incorrect usage."),
        },
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn setup_print(vse_directory: &VSE_DIRECTORY, no: u64) {
    let _vse_directory = vse_directory.lock().await;
    match _vse_directory.setup(no) {
        Some(setup) => setup.print(),
        None => eprintln!("Not found."),
    }
}

async fn dir_print(vse_directory: &VSE_DIRECTORY) {
    let vse_directory_ = vse_directory.lock().await;
    vse_directory_.print().await;
}

async fn dir_fetch_print(coordinator: &PEER) {
    // Retrieve peer from list:

    let directory_ = match coordinator.retrieve_vse_directory().await {
        Ok(directory) => directory,
        Err(_) => return eprintln!("Error fetching directory."),
    };
    directory_.print().await;
}

async fn dir_fetch_save(coordinator: &PEER, db: &SIGNATORY_DB, vse_directory: &mut VSE_DIRECTORY) {
    let new_directory = match coordinator.retrieve_vse_directory().await {
        Ok(directory) => directory,
        Err(_) => return eprintln!("Error fetching directory."),
    };

    match new_directory.save(&db).await {
        true => {
            let mut _vse_directory = vse_directory.lock().await;
            *_vse_directory = new_directory;

            return eprintln!("Directory retrieved and saved.");
        }
        false => return eprintln!("Error saving directory."),
    }
}
