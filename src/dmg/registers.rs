use std::mem::zeroed;

#[derive(Copy, Clone, Debug)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_BIT: u8 = 7;
const SUBTRACT_BIT: u8 = 6;
const HALF_CARRY_BIT: u8 = 5;
const CARRY_BIT: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_BIT
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_BIT
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_BIT
            | (if flag.carry { 1 } else { 0 }) << CARRY_BIT
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte & (0b1 << ZERO_BIT)) != 0;
        let subtract = (byte & (0b1 << SUBTRACT_BIT)) != 0;
        let half_carry = (byte & (0b1 << HALF_CARRY_BIT)) != 0;
        let carry = (byte & (0b1 << CARRY_BIT)) != 0;

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
    pub fn get_reg8(&self, reg: Register) -> u8 {
        todo!()
    }

    pub fn get_reg16(&self, reg_1: Register, reg_2: Register) -> u16 {
        todo!()
    }

    pub fn set_reg8(&self,reg: Register, value: u8) {
        todo!()
    }

    pub fn set_reg16(&self, reg_1: Register, reg_2: Register, value: u16) {
        todo!()
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
mod tests {
    use super::*;

    #[test]
    fn test_get_reg8() {
        let test_value: u8 = 0x14;

        let reg: Registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false
            }
        };

        let result = reg.get_reg8(Register::A);
    }
}
