use super::registers::{Register, Registers};

fn foo() {
    let _ = Registers::get_reg8(Register::A);
}
