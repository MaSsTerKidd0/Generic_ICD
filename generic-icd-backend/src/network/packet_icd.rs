use bincode::{Encode, Decode};
use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};


const CONFIG: Configuration<LittleEndian, Fixint, NoLimit> = bincode::config::legacy();

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub struct PacketICD {
    time_stamp: u32,
    tail: u32,
    station: u32,
    payload: Vec<u8>,
}


pub trait BinarySerializable: Sized {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::error::EncodeError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError>;
}

impl BinarySerializable for PacketICD {
    fn to_bytes(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, CONFIG)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (decoded, _): (PacketICD, usize) = bincode::decode_from_slice(bytes, CONFIG)?;
        Ok(decoded)
    }
}
