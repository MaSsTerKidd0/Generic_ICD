use pcap::{Capture, Error};


pub struct PcapHandler;
impl PcapHandler {
     pub fn read_pcap(pcap_file: &str) -> Result<Vec<Vec<u8>>, Error> {
          let mut captured_packets: Vec<Vec<u8>> = Vec::new();
          let mut cap = Capture::from_file(pcap_file)?;

          while let Ok(packet) = cap.next_packet() {
               captured_packets.push(packet.data.to_vec()); // Convert to owned Vec<u8>
          }

          Ok(captured_packets)
     }
}