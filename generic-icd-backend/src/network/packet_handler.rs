use pcap::Packet;
use crate::network::packet_icd::{BinarySerializable, PacketICD};
use crate::network::protocols::Protocol;

pub struct PacketHandler;

const ETHERNET_HEADER_LEN: usize = 34;
impl PacketHandler{
    ///extract ether payload of a packet
    /// 
    /// # Arguments
    /// * packet_byte_stream: Vec<u8> -> with ether headers
    fn strip_ethernet_header(_packet_byte_stream: &[u8]) -> Vec<u8>{
        return _packet_byte_stream[ETHERNET_HEADER_LEN..].to_vec();
    }
    pub fn parse_from_bytes<T: BinarySerializable>(_packet: &[u8]) -> Result<T, bincode::error::DecodeError> {
        T::from_bytes(_packet)
    }
    fn parse_to_bytes<T: BinarySerializable>(_packet: &T) -> Vec<u8> {
        _packet.to_bytes().unwrap()
    }
    pub fn strip_ethernet_header_from_packets(_packets: Vec<Vec<u8>>, _protocol :Option<Protocol>) -> Vec<Vec<u8>>{
        let mut packets_payload: Vec<Vec<u8>> = Vec::new();

        for packet in _packets{
            packets_payload.push(Self::strip_ethernet_header(packet.as_slice()));
        }

        return packets_payload;
    }
    pub fn parse_packets_from_bytes(){
        
    }

    pub fn parse_packets_to_bytes(){

    }




}