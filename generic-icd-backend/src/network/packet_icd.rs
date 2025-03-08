//stracture represetnt network icd 
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct PacketICD{
    time_stamp: u32,
    tail: u32,
    station: u32,
    payload: Vec<u8>
}


impl PacketICD{
    fn serialize(&self)->Vec<u8>{
        let mut vec = Vec::new();
        
    }
    fn deserialize(icd: &PacketICD)->PacketICD{
        
    }
}
