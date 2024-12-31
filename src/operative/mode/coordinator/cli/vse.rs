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
