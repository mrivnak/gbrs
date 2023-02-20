use std::ops::Add;

use crate::{registers::Registers, data::SplitByte};

use super::*;

#[test]
fn test_get_x8_n8() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr = 0x0000;
    let data = 0xAA;
    mem.write(addr, data);

    let n8 = get_x8(InstructionTarget::N8, Some(addr), &mut reg, &mut mem);

    assert_eq!(data, n8);
}

#[test]
fn test_get_x8_reg() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let data = 0xAA;
    reg.set_reg8(Register::B, data);

    let b = get_x8(InstructionTarget::B, None, &mut reg, &mut mem);

    assert_eq!(data, b);
}

#[test]
fn test_get_x8_addr_hl() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr: Address = 0x0000;
    let addr_bytes = addr.split_byte();
    let data = 0xAA;
    
    reg.set_reg8(Register::H, addr_bytes.0);
    reg.set_reg8(Register::L, addr_bytes.1);
    mem.write(addr, data);

    let addr_hl = get_x8(InstructionTarget::ADDR_HL, None, &mut reg, &mut mem);

    assert_eq!(data, addr_hl);
}

#[test]
fn test_NOP() {
    let base_mem = MemoryBus::create();
    let mut out_mem = base_mem.clone();
    let base_reg = Registers::create();
    let mut out_reg = base_reg.clone();

    execute_instruction(0x00, &mut out_reg, &mut out_mem);

    assert_eq!(base_mem, out_mem);
    assert_eq!(base_reg, out_reg);
}
