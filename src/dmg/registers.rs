#[derive(Copy, Clone, Debug)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << 7
            | (if flag.subtract { 1 } else { 0 }) << 6
            | (if flag.half_carry { 1 } else { 0 }) << 5
            | (if flag.carry { 1 } else { 0 }) << 4
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte & 0b10000000) != 0;
        let subtract = (byte & 0b01000000) != 0;
        let half_carry = (byte & 0b00100000) != 0;
        let carry = (byte & 0b00010000) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: FlagsRegister,
}

impl Registers {
    pub fn get_reg8(reg: Register) -> u8 {
        todo!();
    }

    pub fn get_reg16(reg_1: Register, reg_2: Register) -> u16 {
        todo!();
    }

    pub fn set_reg8(reg: Register, value: u8) {
        todo!();
    }

    pub fn set_reg16(reg_1: Register, reg_2: Register, value: u16) {
        todo!();
    }

    pub fn get_zero(&self) -> bool {
        self.flags.zero
    }

    pub fn get_subtract(&self) -> bool {
        self.flags.subtract
    }

    pub fn get_half_carry(&self) -> bool {
        self.flags.half_carry
    }

    pub fn get_carry(&self) -> bool {
        self.flags.carry
    }

    pub fn set_zero(&mut self, value: bool) {
        self.flags.zero = value;
    }

    pub fn set_subtract(&mut self, value: bool) {
        self.flags.subtract = value;
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.flags.half_carry = value;
    }

    pub fn set_carry(&mut self, value: bool) {
        self.flags.carry = value;
    }

    pub fn get_flags(&mut self) -> u8 {
        self.flags.into()
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flags = value.into();
    }
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[cfg(test)]
mod tests {}
