pub enum ArithmeticMode {
    Add,
    Subtract,
}

pub fn check_carry8(mode: ArithmeticMode, a: u8, b: u8) -> bool {
    match mode {
        ArithmeticMode::Add => a as u16 + b as u16 > 0xFF,
        ArithmeticMode::Subtract => b > a,
    }
}

pub fn check_half_carry8(mode: ArithmeticMode, a: u8, b: u8) -> bool {
    match mode {
        ArithmeticMode::Add => (a & 0x0F) + (b & 0x0F) & 0x10 == 0x10,
        ArithmeticMode::Subtract => (a & 0x10 == 0x10) && (b & 0x0F > 0x0),
    }
}

pub fn check_carry16(mode: ArithmeticMode, a: u16, b: u16) -> bool {
    match mode {
        ArithmeticMode::Add => a as u32 + b as u32 > 0xFFFF,
        ArithmeticMode::Subtract => b > a,
    }
}

pub fn check_half_carry16(mode: ArithmeticMode, a: u16, b: u16) -> bool {
    match mode {
        ArithmeticMode::Add => (a & 0x0FFF) + (b & 0x0FFF) & 0x1000 == 0x1000,
        ArithmeticMode::Subtract => (a & 0x1000 == 0x1000) && (b & 0x0FFF > 0x0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_carry8() {
        // Addition
        assert_eq!(true, check_carry8(ArithmeticMode::Add, 0xFF, 0x01));
        assert_eq!(false, check_carry8(ArithmeticMode::Add, 0xFF, 0x00));
        assert_eq!(false, check_carry8(ArithmeticMode::Add, 0x00, 0x00));

        // Subtraction
        assert_eq!(true, check_carry8(ArithmeticMode::Subtract, 0x00, 0xFF));
        assert_eq!(false, check_carry8(ArithmeticMode::Subtract, 0xFF, 0x00));
    }

    #[test]
    fn test_check_half_carry8() {
        // Addition
        assert_eq!(
            true,
            check_half_carry8(ArithmeticMode::Add, 0b0000_1000, 0b0000_1000)
        );
        assert_eq!(
            false,
            check_half_carry8(ArithmeticMode::Add, 0b0000_1000, 0b0001_0000)
        );
        assert_eq!(
            true,
            check_half_carry8(ArithmeticMode::Add, 0b0000_1111, 0b0000_0001)
        );

        // Subtraction
        assert_eq!(
            true,
            check_half_carry8(ArithmeticMode::Subtract, 0b0001_0000, 0b0000_1000)
        );
        assert_eq!(
            false,
            check_half_carry8(ArithmeticMode::Subtract, 0b0000_1000, 0b0001_0000)
        );
        assert_eq!(
            true,
            check_half_carry8(ArithmeticMode::Subtract, 0b0001_0000, 0b0000_0001)
        );
        assert_eq!(
            true,
            check_half_carry8(ArithmeticMode::Subtract, 0b0001_0000, 0b0000_1000)
        );
    }

    #[test]
    fn test_check_carry16() {
        // Addition
        assert_eq!(true, check_carry16(ArithmeticMode::Add, 0xFFFF, 0x0001));
        assert_eq!(false, check_carry16(ArithmeticMode::Add, 0xFFFF, 0x0000));
        assert_eq!(false, check_carry16(ArithmeticMode::Add, 0x0000, 0x0000));

        // Subtraction
        assert_eq!(
            true,
            check_carry16(ArithmeticMode::Subtract, 0x0000, 0xFFFF)
        );
        assert_eq!(
            false,
            check_carry16(ArithmeticMode::Subtract, 0xFFFF, 0x0000)
        );
    }

    #[test]
    fn test_check_half_carry16() {
        // Addition
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Add,
                0b0000_1000_0000_0000,
                0b0000_1000_0000_0000
            )
        );
        assert_eq!(
            false,
            check_half_carry16(
                ArithmeticMode::Add,
                0b0000_1000_0000_0000,
                0b0001_0000_0000_0000
            )
        );
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Add,
                0b0000_1111_0000_0000,
                0b0000_0001_0000_0000
            )
        );
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Add,
                0b1111_1111_1111_1111,
                0b0000_0000_0000_0001
            )
        );

        // Subtraction
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Subtract,
                0b0001_0000_0000_0000,
                0b0000_1000_0000_0000
            )
        );
        assert_eq!(
            false,
            check_half_carry16(
                ArithmeticMode::Subtract,
                0b0000_1000_0000_0000,
                0b0001_0000_0000_0000
            )
        );
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Subtract,
                0b0001_0000_0000_0000,
                0b0000_0001_0000_0000
            )
        );
        assert_eq!(
            true,
            check_half_carry16(
                ArithmeticMode::Subtract,
                0b0001_0000_0000_0000,
                0b0000_1000_0000_0000
            )
        );
    }
}
