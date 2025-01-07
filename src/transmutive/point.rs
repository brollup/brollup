use secp::Point;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)]
pub struct SecpPoint(Point);

impl SecpPoint {
    pub fn new(point: Point) -> Self {
        Self(point)
    }

    pub fn inner(&self) -> &Point {
        &self.0
    }
}

// Implement `Eq` for `SecpPoint`
impl Eq for SecpPoint {}

// Implement `Hash` for `SecpPoint`
impl Hash for SecpPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.serialize().hash(state);
    }
}

impl Serialize for SecpPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serialized = self.0.serialize();
        serializer.serialize_bytes(&serialized)
    }
}

impl<'de> Deserialize<'de> for SecpPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        Point::from_slice(&bytes)
            .map(SecpPoint)
            .map_err(serde::de::Error::custom)
    }
}
