use crate::{Peer, SignatoryDB, VSEDirectory};

// vse
pub async fn command(
    parts: Vec<&str>,
    signatory_db: &SignatoryDB,
    vse_directory: &VSEDirectory,
    coordinator: &Peer,
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
                    None => eprintln!("VSE directory not available."),
                }
            }
            Err(_) => eprintln!("Invalid <no>."),
        },
        _ => {
            eprintln!("Invalid command.")
        }
    }
}
