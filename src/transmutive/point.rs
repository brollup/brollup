use secp::Point;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Copy, Debug, Clone)]
pub struct SerializablePoint(Point);

impl SerializablePoint {
    pub fn new(point: Point) -> Self {
        SerializablePoint(point)
    }
}

impl Deref for SerializablePoint {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement Serialize for SerializablePoint
impl Serialize for SerializablePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.serialize())
    }
}

// Implement Deserialize for SerializablePoint
impl<'de> Deserialize<'de> for SerializablePoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        let point = Point::from_slice(&bytes).map_err(serde::de::Error::custom)?;
        Ok(SerializablePoint(point))
    }
}

// Implement PartialEq and Eq
impl PartialEq for SerializablePoint {
    fn eq(&self, other: &Self) -> bool {
        self.0.serialize() == other.0.serialize()
    }
}

impl Eq for SerializablePoint {}

// Implement PartialOrd and Ord
impl PartialOrd for SerializablePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.serialize().partial_cmp(&other.0.serialize())
    }
}

impl Ord for SerializablePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.serialize().cmp(&other.0.serialize())
    }
}

impl Hash for SerializablePoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the serialized representation of the point
        self.0.serialize().hash(state);
    }
}
