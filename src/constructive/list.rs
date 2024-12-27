pub trait ListCodec {
    fn encode_list(&self) -> Vec<u8>;
    fn decode_list(encoded: &Vec<u8>) -> Option<Self>
    where
        Self: Sized;
}

// Implement ListCodec for Vec<Vec<u8>>
impl ListCodec for Vec<Vec<u8>> {
    fn encode_list(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        for vec in self {
            let len = vec.len() as u32;
            encoded.extend_from_slice(&len.to_le_bytes());
            encoded.extend_from_slice(vec);
        }

        encoded
    }

    fn decode_list(encoded: &Vec<u8>) -> Option<Self> {
        let mut decoded: Vec<Vec<u8>> = Vec::new();
        let mut i = 0;

        while i < encoded.len() {
            if i + 4 > encoded.len() {
                return None;
            }

            let len = u32::from_le_bytes([
                encoded[i],
                encoded[i + 1],
                encoded[i + 2],
                encoded[i + 3],
            ]) as usize;
            i += 4;

            if i + len > encoded.len() {
                return None;
            }

            let vec = encoded[i..i + len].to_vec();
            decoded.push(vec);

            i += len;
        }

        Some(decoded)
    }
}

// Implement ListCodec for Vec<[u8; 32]>
impl ListCodec for Vec<[u8; 32]> {
    fn encode_list(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        for array in self {
            encoded.extend_from_slice(array);
        }

        encoded
    }

    fn decode_list(encoded: &Vec<u8>) -> Option<Self> {
        if encoded.len() % 32 != 0 {
            return None;
        }

        let mut decoded: Vec<[u8; 32]> = Vec::new();
        let mut i = 0;

        while i < encoded.len() {
            let mut array = [0u8; 32];
            array.copy_from_slice(&encoded[i..i + 32]);
            decoded.push(array);

            i += 32;
        }

        Some(decoded)
    }
}
