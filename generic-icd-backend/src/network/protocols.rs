


pub enum Protocol {
    TCP,
    UDP,
}

const TCP_HEADER_SIZE: u8 = 20;
const UDP_HEADER_SIZE: u8 = 8;
impl Protocol {
    fn header_size(&self) -> u8 {
        match self {
            Protocol::TCP => TCP_HEADER_SIZE,
            Protocol::UDP => UDP_HEADER_SIZE,
        }
    }
}
