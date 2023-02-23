#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: FlagsRegister,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn create() -> Registers {
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            flags: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        }
    }

    pub fn get_reg8(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => u8::from(self.flags),
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub fn get_reg16(&self, reg: RegisterPair) -> u16 {
        match reg {
            RegisterPair::BC => (self.b as u16) << 8 | self.c as u16,
            RegisterPair::DE => (self.d as u16) << 8 | self.e as u16,
            RegisterPair::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    pub fn set_reg8(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.flags = FlagsRegister::from(value),
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub fn set_reg16(&mut self, reg: RegisterPair, value: u16) {
        match reg {
            RegisterPair::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            RegisterPair::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            RegisterPair::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Zero => self.flags.zero,
            Flag::Subtract => self.flags.subtract,
            Flag::HalfCarry => self.flags.half_carry,
            Flag::Carry => self.flags.carry,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        match flag {
            Flag::Zero => self.flags.zero = value,
            Flag::Subtract => self.flags.subtract = value,
            Flag::HalfCarry => self.flags.half_carry = value,
            Flag::Carry => self.flags.carry = value,
        }
    }

    pub fn get_flags(&mut self) -> u8 {
        u8::from(self.flags)
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flags = FlagsRegister::from(value);
    }
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}
pub enum RegisterPair {
    BC,
    DE,
    HL,
}

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reg8() {
        let expected: u8 = 0x14;

        let reg: Registers = Registers {
            a: expected,
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_reg8(Register::A);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_reg16() {
        let expected: u16 = 0xFF00;
        let expected_high: u8 = ((expected & 0xFF00) >> 8) as u8;
        let expected_low: u8 = (expected & 0xFF) as u8;

        let reg: Registers = Registers {
            a: 0,
            b: expected_high,
            c: expected_low,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_reg16(RegisterPair::BC);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_set_reg8() {
        let expected: u8 = 0x14;

        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_reg8(Register::A, expected);
        let result = reg.a;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_set_reg16() {
        let expected: u16 = 0xFF00;
        let expected_high: u8 = ((expected & 0xFF00) >> 8) as u8;
        let expected_low: u8 = (expected & 0xFF) as u8;

        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_reg16(RegisterPair::BC, expected);
        let result_high = reg.b;
        let result_low = reg.c;

        assert_eq!(result_high, expected_high);
        assert_eq!(result_low, expected_low);
    }

    #[test]
    fn test_get_zero() {
        let reg: Registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_flag(Flag::Zero);

        assert!(result);
    }

    #[test]
    fn test_get_subtract() {
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
                subtract: true,
                half_carry: false,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_flag(Flag::Subtract);

        assert!(result);
    }

    #[test]
    fn test_get_half_carry() {
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
                half_carry: true,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_flag(Flag::HalfCarry);

        assert!(result);
    }

    #[test]
    fn test_get_carry() {
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
                carry: true,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_flag(Flag::Carry);

        assert!(result);
    }

    #[test]
    fn test_set_zero() {
        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_flag(Flag::Zero, true);

        assert!(reg.flags.zero);
    }

    #[test]
    fn test_set_subtract() {
        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_flag(Flag::Subtract, true);

        assert!(reg.flags.subtract);
    }

    #[test]
    fn test_set_half_carry() {
        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_flag(Flag::HalfCarry, true);

        assert!(reg.flags.half_carry);
    }

    #[test]
    fn test_set_carry() {
        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_flag(Flag::Carry, true);

        assert!(reg.flags.carry);
    }

    #[test]
    fn test_flags_from_u8() {
        let test: u8 = 0b11000000;

        let flags = FlagsRegister::from(test);

        assert!(flags.zero);
        assert!(flags.subtract);
        assert!(!flags.half_carry);
        assert!(!flags.carry);
    }

    #[test]
    fn test_u8_from_flags() {
        let expected: u8 = 0b11000000;
        let flags = FlagsRegister {
            zero: true,
            subtract: true,
            half_carry: false,
            carry: false,
        };

        let result = u8::from(flags);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_flags() {
        let expected: u8 = 0b11000000;
        let mut reg: Registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagsRegister {
                zero: true,
                subtract: true,
                half_carry: false,
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        let result = reg.get_flags();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_set_flags() {
        let test: u8 = 0b11000000;
        let mut reg: Registers = Registers {
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
                carry: false,
            },
            pc: 0x0000,
            sp: 0x0000,
        };

        reg.set_flags(test);

        assert!(reg.flags.zero);
        assert!(reg.flags.subtract);
        assert!(!reg.flags.half_carry);
        assert!(!reg.flags.carry);
    }
}
