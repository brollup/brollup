#[cfg(test)]
mod vse_tests {
    use brollup::noist_vse::{Directory, KeyMap};

    #[test]
    fn vse_directory() -> Result<(), String> {
        let signer_1: [u8; 32] =
            hex::decode("45d4884c4c96b45728611b97a792c5a79e683fc0e7f82dd55c62101203970a03")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_2: [u8; 32] =
            hex::decode("5d2206732a7656624a694ed9ff38083974ed4b2492ae8ddfb5ccc9d407b786bb")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_3: [u8; 32] =
            hex::decode("35c5bee3bb81d7c4938af505ddf21bfea9929120ec2afa9274a11bef8d0b3c0a")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_list = vec![signer_1, signer_2, signer_3];

        let signer_1_2_vse_key: [u8; 32] =
            hex::decode("c64ed1b803bbce14cb9a2d2de01923a926dbb2858ae120685b3adba20966b5fa")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_1_3_vse_key: [u8; 32] =
            hex::decode("4c87f08ebef8cf44353b5407f6bc10a79174f5072c518256e1671579358db2ed")
                .unwrap()
                .try_into()
                .unwrap();

        let signer_2_3_vse_key: [u8; 32] =
            hex::decode("bffb5c0a04bbef154a65b9f4fcc916f06e7ae81e0880c19617ff41eba7f8a6d0")
                .unwrap()
                .try_into()
                .unwrap();

        let mut signer_1_keymap = KeyMap::new(signer_1);
        signer_1_keymap.insert(signer_2, signer_1_2_vse_key);
        signer_1_keymap.insert(signer_3, signer_1_3_vse_key);

        if !signer_1_keymap.is_complete(&signer_list) {
            return Err("signer_1_keymap is not complete.".to_string());
        }

        let mut signer_2_keymap = KeyMap::new(signer_2);
        signer_2_keymap.insert(signer_1, signer_1_2_vse_key);
        signer_2_keymap.insert(signer_3, signer_2_3_vse_key);

        if !signer_2_keymap.is_complete(&signer_list) {
            return Err("signer_2_keymap is not complete.".to_string());
        }

        let mut signer_3_keymap = KeyMap::new(signer_3);
        signer_3_keymap.insert(signer_1, signer_1_3_vse_key);
        signer_3_keymap.insert(signer_2, signer_2_3_vse_key);

        if !signer_3_keymap.is_complete(&signer_list) {
            return Err("signer_3_keymap is not complete.".to_string());
        }

        let mut directory = Directory::new(&signer_list);

        if !directory.insert(signer_1_keymap) {
            return Err("Failed to insert signer_1_keymap".to_string());
        }

        if !directory.insert(signer_2_keymap) {
            return Err("Failed to insert signer_2_keymap".to_string());
        }

        if !directory.insert(signer_3_keymap) {
            return Err("Failed to insert signer_3_keymap".to_string());
        }

        if !directory.is_complete() {
            return Err("Directory is not yet complete.".to_string());
        }

        if !directory.validate() {
            return Err("Directory is not valid.".to_string());
        }

        let signer_1_2_vse_key_ = directory
            .vse_key(signer_1, signer_2)
            .ok_or("No signer_1_2_vse_key_ returned.".to_string())?;

        if signer_1_2_vse_key_ != signer_1_2_vse_key {
            return Err("Invalid signer_1_2_vse_key_.".to_string());
        }

        let signer_1_3_vse_key_ = directory
            .vse_key(signer_1, signer_3)
            .ok_or("No signer_1_3_vse_key_ returned.".to_string())?;

        if signer_1_3_vse_key_ != signer_1_3_vse_key {
            return Err("Invalid signer_1_3_vse_key_.".to_string());
        }

        let signer_2_3_vse_key_ = directory
            .vse_key(signer_2, signer_3)
            .ok_or("No signer_2_3_vse_key_ returned.".to_string())?;

        if signer_2_3_vse_key_ != signer_2_3_vse_key {
            return Err("Invalid signer_2_3_vse_key_.".to_string());
        }

        Ok(())
    }
}