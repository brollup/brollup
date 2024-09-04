use bit_vec::BitVec;

pub trait CompactPayloadEncoding {
    fn to_cpe(&self) -> BitVec;
}

pub trait CommonIndex {
    fn from_u8_common_index(common_index: &u8) -> BitVec;
    fn to_u8_common_index(&self) -> u8;
}

impl CommonIndex for BitVec {
    fn from_u8_common_index(common_index: &u8) -> BitVec {
        let mut bit_vec = BitVec::new();

        // 3-bit common index encoding
        match common_index {
            0 => {
                // 0b000
                bit_vec.push(false);
                bit_vec.push(false);
                bit_vec.push(false);
            }
            1 => {
                // 0b001
                bit_vec.push(false);
                bit_vec.push(false);
                bit_vec.push(true);
            }
            2 => {
                // 0b010
                bit_vec.push(false);
                bit_vec.push(true);
                bit_vec.push(false);
            }
            3 => {
                // 0b011
                bit_vec.push(false);
                bit_vec.push(true);
                bit_vec.push(true);
            }
            4 => {
                // 0b100
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(false);
            }
            5 => {
                // 0b101
                bit_vec.push(true);
                bit_vec.push(false);
                bit_vec.push(true);
            }
            6 => {
                // 0b110
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(false);
            }
            7 => {
                // 0b111
                bit_vec.push(true);
                bit_vec.push(true);
                bit_vec.push(true);
            }
            _ => panic!("Common index must be 3-bits-long."),
        }
        bit_vec
    }

    fn to_u8_common_index(&self) -> u8 {
        let mut bit_vec = BitVec::new();
        bit_vec.extend(self);

        // Pad the remaining five significant bits with zeros.
        bit_vec.insert(0, false);
        bit_vec.insert(0, false);
        bit_vec.insert(0, false);
        bit_vec.insert(0, false);
        bit_vec.insert(0, false);

        bit_vec.to_bytes()[0]
    }
}
