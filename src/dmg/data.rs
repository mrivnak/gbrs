pub type Address = u16;

pub trait SplitByte {
    fn split_byte(&self) -> (u8, u8);
}

impl SplitByte for Address {
    fn split_byte(&self) -> (u8, u8) {
        (((self & 0xFF00) >> 8) as u8, (self & 0xFF) as u8)
    }
}