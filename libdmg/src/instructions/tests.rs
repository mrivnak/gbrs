use crate::{registers::Registers, data::SplitByte};

use super::*;

#[test]
fn test_get_x8_n8() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr = 0x0000;
    let data = 0xAA;
    mem.write(addr, data);

    let n8 = get_x8(InstructionTarget::N8(addr), &mut reg, &mut mem);

    assert_eq!(data, n8);
}

#[test]
fn test_get_x8_reg() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let data = 0xAA;
    reg.set_reg8(Register::B, data);

    let b = get_x8(InstructionTarget::B, &mut reg, &mut mem);

    assert_eq!(data, b);
}

#[test]
fn test_get_x8_addr_hl() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr: Address = 0x0000;
    let addr_bytes = addr.split_byte();
    let data = 0xAA;
    
    reg.set_reg8(Register::H, addr_bytes.1);
    reg.set_reg8(Register::L, addr_bytes.0);
    mem.write(addr, data);

    let addr_hl = get_x8(InstructionTarget::N8(reg.get_reg16(RegisterPair::HL)), &mut reg, &mut mem);

    assert_eq!(data, addr_hl);
}

#[test]
fn test_set_x8_n8() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr = 0x0000;
    let data = 0xAA;

    set_x8(InstructionTarget::N8(addr), &mut reg, &mut mem, data);

    let n8 = mem.read(addr);

    assert_eq!(data, n8);
}

#[test]
fn test_set_x8_reg() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let data = 0xAA;

    set_x8(InstructionTarget::B, &mut reg, &mut mem, data);

    let u8 = reg.get_reg8(Register::B);

    assert_eq!(data, u8);
}

#[test]
fn test_set_x8_addr_hl() {
    let mut mem = MemoryBus::create();
    let mut reg = Registers::create();

    let addr: Address = 0x0000;
    let addr_bytes = addr.split_byte();
    let data = 0xAA;
    
    reg.set_reg8(Register::H, addr_bytes.0);
    reg.set_reg8(Register::L, addr_bytes.1);

    set_x8(InstructionTarget::N8(reg.get_reg16(RegisterPair::HL)), &mut reg, &mut mem, data);

    let addr_hl = mem.read(addr);

    assert_eq!(data, addr_hl);
}

#[test]
fn test_0x00_NOP() {
    let base_mem = MemoryBus::create();
    let mut out_mem = base_mem.clone();
    let mut base_reg = Registers::create();
    let mut out_reg = base_reg.clone();

    base_reg.pc += 1;
    let _ = execute_instruction(0x00, &mut out_reg, &mut out_mem);

    assert_eq!(base_mem, out_mem);
    assert_eq!(base_reg, out_reg);
}

#[test]
fn test_0x01_LD_BC_u16() {
    let base_mem = MemoryBus::create();
    let mut out_mem = base_mem.clone();
    let base_reg = Registers::create();
    let mut out_reg = base_reg.clone();

    // Populate the memory with a test value
    let addr = out_reg.pc + 1;
    let data: u16 = 0x1234;
    let split_data = (((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8);
    out_mem.write(addr, split_data.1);
    out_mem.write(addr + 1, split_data.0);

    let _ = execute_instruction(0x01, &mut out_reg, &mut out_mem);

    assert_eq!(data, out_reg.get_reg16(RegisterPair::BC));
}