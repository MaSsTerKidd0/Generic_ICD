mod network;
use std::io::Read;
use network::pcap_handler::PcapHandler;
use network::packet_handler::PacketHandler;


fn main() {
    let packets = PcapHandler::read_pcap("example.pcap").unwrap();
    let processed_packets = PacketHandler::process_packets(packets);
    for packet in processed_packets {
        println!("{:?}", packet.as_slice());
    }
    

}

