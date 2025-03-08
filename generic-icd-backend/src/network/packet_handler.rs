use pcap::Packet;

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
    fn parse_from_bytes(){

    }
    fn parse_to_bytes(){

    }
    pub fn strip_ethernet_header_from_packets(_packets: Vec<Vec<u8>>) -> Vec<Vec<u8>>{
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