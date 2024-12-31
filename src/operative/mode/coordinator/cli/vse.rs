use crate::{vse_setup, PeerList, SignatoryDB, VSEDirectory};

// vse setup <no> print
// vse setup <no> run
// vse dir print
// vse dir fetch <peer> print
// vse dir fetch <peer> save
pub async fn command(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &VSEDirectory,
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
                    "save" => dir_fetch_save(operator_list, peer_key, vse_directory).await,
                    _ => return eprintln!("Incorrect usage."),
                }
            }
            _ => return eprintln!("Incorrect usage."),
        },
        _ => return eprintln!("Incorrect usage."),
    }

    match parts.len() {
        2 => match parts[1].parse::<u64>() {
            Ok(no) => {
                let directory_ = {
                    let mut _vse_directory = vse_directory.lock().await;
                    (*_vse_directory).clone()
                };

                match directory_.setup(no) {
                    Some(setup) => {
                        setup.print();
                    }
                    None => {
                        match vse_setup::run(operator_list, signatory_db, vse_directory, no).await {
                            Some(setup) => {
                                eprintln!("VSE protocol run with success.");
                                setup.print();
                            }
                            None => return eprintln!("VSE protocol failed."),
                        };
                    }
                }
            }
            Err(_) => eprintln!("Invalid <no>."),
        },
        _ => {
            eprintln!("Invalid command.")
        }
    }
}

async fn setup_print(vse_directory: &VSEDirectory, no: u64) {}

async fn setup_run(
    operator_list: &PeerList,
    signatory_db: &SignatoryDB,
    vse_directory: &VSEDirectory,
    no: u64,
) {
}

async fn dir_print(vse_directory: &VSEDirectory) {}

async fn dir_fetch_print(operator_list: &PeerList, peer: [u8; 32]) {}

async fn dir_fetch_save(operator_list: &PeerList, peer: [u8; 32], vse_directory: &VSEDirectory) {}
