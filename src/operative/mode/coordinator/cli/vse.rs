use crate::{tcp_client::Request, vse_setup, PeerList, SignatoryDB, VSEDirectory};

// vse setup <no> print
// vse setup <no> run
// vse dir print
// vse dir fetch <peer> print
// vse dir fetch <peer> save
pub async fn command(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &mut VSEDirectory,
    parts: Vec<&str>,
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
                "run" => setup_run(operator_list, signatory_db, vse_directory, no).await,
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
                // Args len must be exactly 5.
                if parts.len() != 5 {
                    return eprintln!("Incorrect usage.");
                }

                let peer_key: [u8; 32] = match hex::decode(parts[3]) {
                    Ok(decoded) => {
                        if decoded.len() != 32 {
                            return eprintln!("Invalid <peer>.");
                        }
                        decoded.try_into().expect("")
                    }
                    Err(_) => return eprintln!("Invalid <peer>."),
                };

                match parts[4] {
                    "print" => dir_fetch_print(operator_list, peer_key).await,
                    "save" => {
                        dir_fetch_save(operator_list, peer_key, signatory_db, vse_directory).await
                    }
                    _ => return eprintln!("Incorrect usage."),
                }
            }
            _ => return eprintln!("Incorrect usage."),
        },
        _ => return eprintln!("Incorrect usage."),
    }
}

async fn setup_print(vse_directory: &VSEDirectory, no: u64) {
    let _vse_directory = vse_directory.lock().await;
    match _vse_directory.setup(no) {
        Some(setup) => setup.print(),
        None => eprintln!("Not found."),
    }
}

async fn setup_run(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &VSEDirectory,
    no: u64,
) {
    match vse_setup::run(operator_list, signatory_db, vse_directory, no).await {
        Some(setup) => {
            eprintln!("VSE protocol #{} run with success and saved.", no);
            setup.print();
        }
        None => return eprintln!("VSE protocol #{} failed.", no),
    };
}

async fn dir_print(vse_directory: &VSEDirectory) {
    let vse_directory_ = vse_directory.lock().await;
    vse_directory_.print().await;
}

async fn dir_fetch_print(operator_list: &PeerList, peer: [u8; 32]) {
    // Retrieve peer from list:
    let _operator_list = operator_list.lock().await;

    for operator in _operator_list.iter() {
        let lookup = {
            let _operator = operator.lock().await;
            _operator.nns_key() == peer
        };

        if lookup {
            let directory_ = match operator.retrieve_vse_directory().await {
                Ok(directory) => directory,
                Err(_) => return eprintln!("Error fetching directory."),
            };
            directory_.print().await;
        }
    }
}

async fn dir_fetch_save(
    operator_list: &PeerList,
    peer: [u8; 32],
    db: &SignatoryDB,
    vse_directory: &mut VSEDirectory,
) {
    // Retrieve peer from list:
    let _operator_list = operator_list.lock().await;

    for operator in _operator_list.iter() {
        let lookup = {
            let _operator = operator.lock().await;
            _operator.nns_key() == peer
        };

        if lookup {
            let new_directory = match operator.retrieve_vse_directory().await {
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
    }
}
