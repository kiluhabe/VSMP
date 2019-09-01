pub enum PinNumber {
    RSTPin,
    DCPin,
    CSPin,
    BUSYPin,
    BCMPin
}

impl PinNumber {
    pub fn value(&self) -> u8 {
        match *self {
            PinNumber::RSTPin => 17,
            PinNumber::DCPin => 25,
            PinNumber::CSPin => 8,
            PinNumber::BUSYPin => 24,
            PinNumber::BCMPin => 18
        }
    }
}
