type Bytes = Vec<u8>;

#[derive(Debug)]
pub enum SerializeError {
    KeyParseError,
    EntryTypeError,
}

pub trait Serialize {
    fn serialize(&self) -> Bytes;
    fn from_bytes(bytes: Bytes) -> Result<Self, SerializeError> where Self: Sized;
}