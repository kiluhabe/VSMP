pub enum Pin {
    Rst,
    Dc,
    Cs,
    Busy,
    Bcm,
}

impl Pin {
    pub fn value(&self) -> u8 {
        match *self {
            Pin::Rst => 17,
            Pin::Dc => 25,
            Pin::Cs => 8,
            Pin::Busy => 24,
            Pin::Bcm => 18,
        }
    }
}
