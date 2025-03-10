mod network;
use std::io::Read;
use network::pcap_handler::PcapHandler;
use network::packet_handler::PacketHandler;
use crate::network::PacketICD;

fn main() {
    let packets = PcapHandler::read_pcap("example.pcap").unwrap();
    let processed_packets = PacketHandler::strip_ethernet_header_from_packets(packets, None);
    for packet in processed_packets {
        println!("{:?}", packet.as_slice());
        println!("{:?}", PacketHandler::parse_from_bytes::<PacketICD>(packet.as_slice()));
    }


}

