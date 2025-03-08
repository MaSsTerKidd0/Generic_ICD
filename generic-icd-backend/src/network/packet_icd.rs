use bincode::{Encode, Decode};
use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub struct PacketICD {
    time_stamp: u32,
    tail: u32,
    station: u32,
    payload: Vec<u8>,
}
const config: Configuration<LittleEndian, Fixint, NoLimit> = bincode::config::legacy();
impl PacketICD{
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, config)
    }

    // Convert bytes back to struct
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (decoded, _): (PacketICD, usize) = bincode::decode_from_slice(bytes, config)?;
        Ok(decoded)
    }
}
