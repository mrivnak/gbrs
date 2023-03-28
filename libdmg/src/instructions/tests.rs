use crate::{
    data::SplitByte,
    registers::{Flag, Registers},
};

use super::*;

#[test]
fn test_modify_flags() {
    let mut reg = Registers::default();

    let instr = FlagInstruction {
        zero: FlagOperation::Dependent,
        subtract: FlagOperation::Unset,
        half_carry: FlagOperation::Dependent,
        carry: FlagOperation::Unmodified,
    };
    let results = FlagResults {
        zero: Some(FlagResult::Unset),
        subtract: Some(FlagResult::Unset),
        half_carry: Some(FlagResult::Set),
        carry: None,
    };

    reg.set_flag(Flag::Zero, true);
    reg.set_flag(Flag::Subtract, true);

    modify_flags(&mut reg, instr, results);

    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));
}

#[test]
fn test_get_x8_n8() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr = 0x0000;
    let data = 0xAA;
    mem.write(addr, data);

    let n8 = get_x8(&InstructionTarget::N8(addr), &mut reg, &mut mem);

    assert_eq!(data, n8);
}

#[test]
fn test_get_x8_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0xAA;
    reg.set_reg8(Register::B, data);

    let b = get_x8(&InstructionTarget::B, &mut reg, &mut mem);

    assert_eq!(data, b);
}

#[test]
fn test_get_x8_addr_from_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr: Address = 0x0000;
    let addr_bytes = addr.split_byte();
    let data = 0xAA;

    reg.set_reg8(Register::H, addr_bytes.1);
    reg.set_reg8(Register::L, addr_bytes.0);
    mem.write(addr, data);

    let addr_hl = get_x8(
        &InstructionTarget::N8(reg.get_reg16(RegisterPair::HL)),
        &mut reg,
        &mut mem,
    );

    assert_eq!(data, addr_hl);
}

#[test]
fn test_set_x8_n8() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr = 0x0000;
    let data = 0xAA;

    set_x8(&InstructionTarget::N8(addr), &mut reg, &mut mem, data);

    let n8 = mem.read(addr);

    assert_eq!(data, n8);
}

#[test]
fn test_set_x8_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0xAA;

    set_x8(&InstructionTarget::B, &mut reg, &mut mem, data);

    let u8 = reg.get_reg8(Register::B);

    assert_eq!(data, u8);
}

#[test]
fn test_set_x8_addr_from_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr: Address = 0x0000;
    let addr_bytes = addr.split_byte();
    let data = 0xAA;

    reg.set_reg8(Register::H, addr_bytes.0);
    reg.set_reg8(Register::L, addr_bytes.1);

    set_x8(
        &InstructionTarget::N8(reg.get_reg16(RegisterPair::HL)),
        &mut reg,
        &mut mem,
        data,
    );

    let addr_hl = mem.read(addr);

    assert_eq!(data, addr_hl);
}

#[test]
fn test_0x00_NOP() {
    let base_mem = MemoryBus::default();
    let mut out_mem = base_mem.clone();
    let mut base_reg = Registers::default();
    let mut out_reg = base_reg.clone();

    base_reg.pc += 1;
    let _ = execute_instruction(0x00, &mut out_reg, &mut out_mem);

    assert_eq!(base_mem, out_mem);
    assert_eq!(base_reg, out_reg);
}

#[test]
fn test_0x01_LD_BC_u16() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    // Populate the memory with a test value
    let addr = reg.pc + 1;
    let data: u16 = 0x1234;
    let split_data = (((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8);
    mem.write(addr, split_data.1);
    mem.write(addr + 1, split_data.0);

    let _ = execute_instruction(0x01, &mut reg, &mut mem);

    assert_eq!(data, reg.get_reg16(RegisterPair::BC));
}

#[test]
fn test_0x02_LD_BC_A() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data: u8 = 0x12;
    reg.set_reg8(Register::A, data);

    let addr: Address = 0x1122;
    let split_addr = addr.split_byte();
    reg.set_reg8(Register::B, split_addr.0);
    reg.set_reg8(Register::C, split_addr.1);

    let _ = execute_instruction(0x02, &mut reg, &mut mem);

    assert_eq!(data, mem.read(addr));
}

#[test]
fn test_0x03_INC_BC() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x1234;
    reg.set_reg16(RegisterPair::BC, data);

    let _ = execute_instruction(0x03, &mut reg, &mut mem);

    assert_eq!(data + 1, reg.get_reg16(RegisterPair::BC));
}

#[test]
fn test_0x04_INC_B() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x12;
    reg.set_reg8(Register::B, data);

    let _ = execute_instruction(0x04, &mut reg, &mut mem);

    assert_eq!(data + 1, reg.get_reg8(Register::B));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x1F;
    reg.set_reg8(Register::B, data);

    let _ = execute_instruction(0x04, &mut reg, &mut mem);

    assert_eq!(data + 1, reg.get_reg8(Register::B));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));
}

#[test]
fn test_0x05_DEC_B() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x20;
    reg.set_reg8(Register::B, data);

    let _ = execute_instruction(0x05, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::B));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x10;
    reg.set_reg8(Register::B, data);

    let _ = execute_instruction(0x05, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::B));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x01;
    reg.set_reg8(Register::B, data);

    let _ = execute_instruction(0x05, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::B));
    assert_eq!(true, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));
}

#[test]
fn test_0x06_LD_B_u8() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x12;
    mem.write(reg.pc + 1, data);

    let _ = execute_instruction(0x06, &mut reg, &mut mem);

    assert_eq!(data, reg.get_reg8(Register::B));
}

#[test]
fn test_0x07_RLCA() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0b1011_1010;
    let expected = 0b0111_0101;
    reg.set_reg8(Register::A, data);

    reg.set_flag(Flag::Zero, true);
    reg.set_flag(Flag::Subtract, true);
    reg.set_flag(Flag::HalfCarry, true);

    let _ = execute_instruction(0x07, &mut reg, &mut mem);

    assert_eq!(expected, reg.get_reg8(Register::A));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(true, reg.get_flag(Flag::Carry));
}
