#[cfg(test)]
mod noist_tests {
    use brollup::into::IntoPointVec;
    use brollup::noist::dkg::package::DKGPackage;
    use brollup::noist::dkg::session::DKGSession;
    use brollup::{
        noist::setup::{keymap::VSEKeyMap, setup::VSESetup},
        schnorr::Authenticable,
    };

    #[test]
    fn noist_test() -> Result<(), String> {
        let signer_1_secret: [u8; 32] =
            hex::decode("396e7f3b89843e1e5610b1fdbaabf1b6a53066f43b22c529f839d69b6799ce8f")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_1_public: [u8; 32] =
            hex::decode("eae0001e445c4f748f91010c1fb6d5b99391e588e605dbbb6ca4e5d98e520cd7")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_2_secret: [u8; 32] =
            hex::decode("31dfea206f96e7b254e00fddb22baac233feb57d6ea98f3fe6929becad1eee78")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_2_public: [u8; 32] =
            hex::decode("25451c1c2d326a14e86c7921cb1467512c944801c4fc0f81f8bd89e85d3ab1f1")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_3_secret: [u8; 32] =
            hex::decode("38e2361ab771574909a9768670fa33406a311a2cae7d446359f09df18ac2cb83")
                .unwrap()
                .try_into()
                .unwrap();
        let signer_3_public: [u8; 32] =
            hex::decode("e8e5393d1873b616c12c6e2bee0c637f58dc5762dda654903c4dd1a72d762c34")
                .unwrap()
                .try_into()
                .unwrap();

        let full_list = vec![signer_1_public, signer_2_public, signer_3_public];

        let full_point_list = match full_list.into_point_vec() {
            Ok(list) => list,
            Err(_) => return Err(format!("full_point_list err.")),
        };

        // Signer 1 keymap.
        let signer_1_keymap =
            VSEKeyMap::new(signer_1_secret, &vec![signer_2_public, signer_3_public]).unwrap();

        if !signer_1_keymap.is_complete(&full_point_list) {
            return Err(format!("signer_1_keymap is not complete."));
        }

        let signer_1_auth_keymap = match Authenticable::new(signer_1_keymap, signer_1_secret) {
            Some(keymap) => keymap,
            None => return Err(format!("signer_1_auth_keymap err.")),
        };

        if !signer_1_auth_keymap.authenticate() {
            println!("signer_1_auth_keymap auth err.");
        }

        // Signer 2 keymap.
        let signer_2_keymap =
            VSEKeyMap::new(signer_2_secret, &vec![signer_1_public, signer_3_public]).unwrap();

        if !signer_2_keymap.is_complete(&full_point_list) {
            return Err(format!("signer_2_keymap is not complete."));
        }

        let signer_2_auth_keymap = match Authenticable::new(signer_2_keymap, signer_2_secret) {
            Some(keymap) => keymap,
            None => return Err(format!("signer_2_auth_keymap err.")),
        };

        if !signer_2_auth_keymap.authenticate() {
            println!("signer_2_auth_keymap auth err.");
        }

        // Signer 3 keymap.
        let signer_3_keymap =
            VSEKeyMap::new(signer_3_secret, &vec![signer_1_public, signer_2_public]).unwrap();

        if !signer_3_keymap.is_complete(&full_point_list) {
            return Err(format!("signer_3_keymap is not complete."));
        }

        let signer_3_auth_keymap = match Authenticable::new(signer_3_keymap, signer_3_secret) {
            Some(keymap) => keymap,
            None => return Err(format!("signer_3_auth_keymap err.")),
        };

        if !signer_3_auth_keymap.authenticate() {
            println!("signer_3_auth_keymap auth err.");
        }

        let mut vse_setup = match VSESetup::new(&full_list, 0) {
            Some(setup) => setup,
            None => return Err(format!("vse_setup err.")),
        };

        if !vse_setup.insert(signer_1_auth_keymap) {
            return Err(format!("signer_1_auth_keymap insert err."));
        };

        if !vse_setup.insert(signer_2_auth_keymap) {
            return Err(format!("signer_2_auth_keymap insert err."));
        };

        if !vse_setup.insert(signer_3_auth_keymap) {
            return Err(format!("signer_3_auth_keymap insert err."));
        };

        if !vse_setup.validate() {
            return Err(format!("vse_setup validate err."));
        }

        let package_1 = match DKGPackage::new(signer_1_secret, &full_list) {
            Some(package) => package,
            None => return Err(format!("err creating package_1.")),
        };

        if !package_1.is_complete(&full_list) {
            return Err(format!("package_1 is_complete failed."));
        }

        if !package_1.vss_verify() {
            return Err(format!("package_1 vss_verify failed."));
        }

        if !package_1.vse_verify(&vse_setup) {
            return Err(format!("package_1 vse_verify failed."));
        }

        let package_2 = match DKGPackage::new(signer_2_secret, &full_list) {
            Some(package) => package,
            None => return Err(format!("err creating package_2.")),
        };

        if !package_2.is_complete(&full_list) {
            return Err(format!("package_2 is_complete failed."));
        }

        if !package_2.vss_verify() {
            return Err(format!("package_2 vss_verify failed."));
        }

        if !package_2.vse_verify(&vse_setup) {
            return Err(format!("package_2 vse_verify failed."));
        }

        let package_3 = match DKGPackage::new(signer_3_secret, &full_list) {
            Some(package) => package,
            None => return Err(format!("err creating package_3.")),
        };

        if !package_3.is_complete(&full_list) {
            return Err(format!("package_3 is_complete failed."));
        }

        if !package_3.vss_verify() {
            return Err(format!("package_3 vss_verify failed."));
        }

        if !package_3.vse_verify(&vse_setup) {
            return Err(format!("package_3 vse_verify failed."));
        }

        let mut session = match DKGSession::new(0, &full_list) {
            Some(session) => session,
            None => return Err(format!("session construction failed.")),
        };

        if !session.insert(&package_1, &vse_setup) {
            return Err(format!("session package_1 insertion failed."));
        }

        if !session.insert(&package_2, &vse_setup) {
            return Err(format!("session package_2 insertion failed."));
        }

        if !session.insert(&package_3, &vse_setup) {
            return Err(format!("session package_3 insertion failed."));
        }

        if !session.is_above_threshold() {
            return Err(format!("session threshold is not met (2-of-3)."));
        }

        println!("is_full: {}", session.is_full());
        println!("is_above_threshold: {}", session.is_above_threshold());

        Ok(())
    }
}
