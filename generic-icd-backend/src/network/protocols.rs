


pub enum Protocol{
    TCP,
    UDP
}

impl Protocol {
    fn header_size(&self) -> u8{
        match self {
            Protocol::TCP => 0,
            Protocol::UDP => 1
        }
    }
}