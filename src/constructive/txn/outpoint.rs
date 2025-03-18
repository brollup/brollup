use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Outpoint {
    #[serde(
        serialize_with = "serialize_prev",
        deserialize_with = "deserialize_prev"
    )]
    prev: [u8; 32],
    vout: u32,
}

impl Outpoint {
    pub fn new(prev: [u8; 32], vout: u32) -> Outpoint {
        Outpoint { prev, vout }
    }
    pub fn prev(&self) -> [u8; 32] {
        self.prev
    }

    pub fn vout(&self) -> u32 {
        self.vout
    }

    pub fn vout_bytes(&self) -> [u8; 4] {
        self.vout.to_be_bytes()
    }

    pub fn bytes(&self) -> [u8; 36] {
        let mut bytes: [u8; 36] = [0; 36];
        bytes[..32].copy_from_slice(&self.prev);
        bytes[32..36].copy_from_slice(&self.vout_bytes());
        bytes
    }
}

// Custom function to serialize `prev` as a hex string
fn serialize_prev<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&hex::encode(bytes))
}

fn deserialize_prev<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
    Ok(bytes.try_into().unwrap())
}
