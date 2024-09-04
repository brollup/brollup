pub enum ParseError {
    ParseError32,
    ParseError33,
    ParseError64,
    ParseError65
}

pub trait IntoByteArray {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError>;
    fn into_byte_array_33(&self) -> Result<[u8; 33], ParseError>;
    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError>;
    fn into_byte_array_65(&self) -> Result<[u8; 65], ParseError>;
}

impl IntoByteArray for Vec<u8> {
    fn into_byte_array_32(&self) -> Result<[u8; 32], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(32);
        vec.extend(self);
        let bytes_32: [u8; 32] = vec.try_into().map_err(|_| ParseError::ParseError32)?;

        Ok(bytes_32)
    }

    fn into_byte_array_33(&self) -> Result<[u8; 33], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(33);
        vec.extend(self);
        let bytes_33: [u8; 33] = vec.try_into().map_err(|_| ParseError::ParseError33)?;

        Ok(bytes_33)
    }

    fn into_byte_array_64(&self) -> Result<[u8; 64], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(64);
        vec.extend(self);
        let bytes_64: [u8; 64] = vec.try_into().map_err(|_| ParseError::ParseError64)?;

        Ok(bytes_64)
    }

    fn into_byte_array_65(&self) -> Result<[u8; 65], ParseError> {
        let mut vec = Vec::<u8>::with_capacity(65);
        vec.extend(self);
        let bytes_65: [u8; 65] = vec.try_into().map_err(|_| ParseError::ParseError65)?;

        Ok(bytes_65)
    }
}
