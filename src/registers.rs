struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte & 0b10000000) != 0;
        let subtract = (byte & 0b01000000) != 0;
        let half_carry = (byte & 0b00100000) != 0;
        let carry = (b & 0b00010000) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flags: FlagRegister,
}

impl Registers {
    fn get_reg(reg: Register) -> u8 {
		todo!();
	}

	fn get_reg(reg_1: Register, reg_2: Register) -> u16 {
        todo!();
	}

	fn set_reg(reg: Register, value: u8) {
        todo!();
	}

	fn set_reg(reg_1: Register, reg_2: Register, value: u16) {
        todo!();
	}

    fn get_zero() -> bool {
        &self.flags.zero
    }

    fn get_subtract() -> bool {
        &self.flags.subtract
    }

    fn get_half_carry() -> bool {
        &self.flags.half_carry
    }

    fn get_carry() -> bool {
        &self.flags.carry
    }
}

enum Register {
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
    
}