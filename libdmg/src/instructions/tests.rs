use crate::registers::{Flag, Registers};

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
    let addr_bytes = addr.to_le_bytes();
    let data = 0xAA;

    reg.set_reg8(Register::H, addr_bytes[0]);
    reg.set_reg8(Register::L, addr_bytes[1]);
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
    let addr_bytes = addr.to_le_bytes();
    let data = 0xAA;

    reg.set_reg8(Register::H, addr_bytes[0]);
    reg.set_reg8(Register::L, addr_bytes[1]);

    set_x8(
        &InstructionTarget::Ref(Box::new(InstructionTarget::HL)),
        &mut reg,
        &mut mem,
        data,
    );

    let addr_hl = mem.read(addr);

    assert_eq!(data, addr_hl);
}

#[test]
fn test_get_x16_n16() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr = 0x0000;
    let data: u16 = 0xAABB;
    let bytes = data.to_le_bytes();
    mem.write(addr, bytes[0]);
    mem.write(addr + 1, bytes[1]);

    let n16 = get_x16(&InstructionTarget::N16(addr), &mut reg, &mut mem);

    assert_eq!(data, n16);
}

#[test]
fn test_get_x16_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data: u16 = 0xAABB;
    reg.set_reg16(RegisterPair::BC, data);

    let bc = get_x16(&InstructionTarget::BC, &mut reg, &mut mem);

    assert_eq!(data, bc);
}

#[test]
fn test_get_x16_ref() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr: Address = 0x0000;
    let data: u16 = 0xAABB;
    let bytes = data.to_le_bytes();

    reg.set_reg16(RegisterPair::HL, addr);
    mem.write(addr, bytes[0]);
    mem.write(addr + 1, bytes[1]);

    let result = get_x16(
        &InstructionTarget::Ref(Box::new(InstructionTarget::HL)),
        &mut reg,
        &mut mem,
    );

    assert_eq!(data, result);
}

#[test]
fn test_set_x16_n16() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr = 0x0000;
    let data: u16 = 0xAABB;

    set_x16(&InstructionTarget::N16(addr), &mut reg, &mut mem, data);

    let n16 = u16::from_le_bytes([mem.read(addr), mem.read(addr + 1)]);

    assert_eq!(data, n16);
}

#[test]
fn test_set_x16_reg() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data: u16 = 0xAABB;

    set_x16(&InstructionTarget::BC, &mut reg, &mut mem, data);

    let n16 = u16::from_be_bytes([reg.get_reg8(Register::B), reg.get_reg8(Register::C)]);

    assert_eq!(data, n16);
}

#[test]
fn test_set_x16_ref() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr: Address = 0x0000;
    let addr_bytes = addr.to_le_bytes();
    let data: u16 = 0xAABB;

    reg.set_reg8(Register::H, addr_bytes[0]);
    reg.set_reg8(Register::L, addr_bytes[1]);

    set_x16(
        &InstructionTarget::Ref(Box::new(InstructionTarget::HL)),
        &mut reg,
        &mut mem,
        data,
    );

    let result = u16::from_le_bytes([mem.read(addr), mem.read(addr + 1)]);

    assert_eq!(data, result);
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
    let split_addr = addr.to_be_bytes();
    reg.set_reg8(Register::B, split_addr[0]);
    reg.set_reg8(Register::C, split_addr[1]);

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

#[test]
fn test_0x08_LD_u16_SP() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr: u16 = 0x1000;
    let addr_bytes = addr.to_le_bytes();
    let data = 0x1122;

    reg.sp = data;
    let pc = reg.pc;
    mem.write(pc + 1, addr_bytes[0]);
    mem.write(pc + 2, addr_bytes[1]);

    let _ = execute_instruction(0x08, &mut reg, &mut mem);

    assert_eq!(mem.read(addr), data as u8);
    assert_eq!(mem.read(addr + 1), (data >> 8) as u8);
}

#[test]
fn test_0x09_ADD_HL_BC() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let bc = 0x0800;
    let hl = 0x0800;

    reg.set_reg16(RegisterPair::BC, bc);
    reg.set_reg16(RegisterPair::HL, hl);

    let _ = execute_instruction(0x09, &mut reg, &mut mem);

    let result = reg.get_reg16(RegisterPair::HL);

    assert_eq!(result, hl + bc);
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let bc = 0xFFFF;
    let hl = 0x0001;

    reg.set_reg16(RegisterPair::BC, bc);
    reg.set_reg16(RegisterPair::HL, hl);

    let _ = execute_instruction(0x09, &mut reg, &mut mem);

    let result = reg.get_reg16(RegisterPair::HL);

    assert_eq!(result, hl.wrapping_add(bc));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(true, reg.get_flag(Flag::Carry));
}

#[test]
fn test_0x0A_LD_A_BC() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let addr = 0x0101;
    let data = 0x42;

    mem.write(addr, data);
    reg.set_reg16(RegisterPair::BC, addr);

    let _ = execute_instruction(0x0A, &mut reg, &mut mem);

    assert_eq!(data, reg.get_reg8(Register::A));
}

#[test]
fn test_0x0B_DEC_BC() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x1234;
    reg.set_reg16(RegisterPair::BC, data);

    let _ = execute_instruction(0x0B, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg16(RegisterPair::BC));
}

#[test]
fn test_0x0C_INC_C() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x12;
    reg.set_reg8(Register::C, data);

    let _ = execute_instruction(0x0C, &mut reg, &mut mem);

    assert_eq!(data + 1, reg.get_reg8(Register::C));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x1F;
    reg.set_reg8(Register::C, data);

    let _ = execute_instruction(0x0C, &mut reg, &mut mem);

    assert_eq!(data + 1, reg.get_reg8(Register::C));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));
}

#[test]
fn test_0x0D_DEC_C() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x20;
    reg.set_reg8(Register::C, data);

    let _ = execute_instruction(0x0D, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::C));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x10;
    reg.set_reg8(Register::C, data);

    let _ = execute_instruction(0x0D, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::C));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(true, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));

    let data = 0x01;
    reg.set_reg8(Register::C, data);

    let _ = execute_instruction(0x0D, &mut reg, &mut mem);

    assert_eq!(data - 1, reg.get_reg8(Register::C));
    assert_eq!(true, reg.get_flag(Flag::Zero));
    assert_eq!(true, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(false, reg.get_flag(Flag::Carry));
}

#[test]
fn test_0x0E_LD_C_u8() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0x12;
    mem.write(reg.pc + 1, data);

    let _ = execute_instruction(0x0E, &mut reg, &mut mem);

    assert_eq!(data, reg.get_reg8(Register::C));
}

#[test]
fn test_0x0F_RRCA() {
    let mut mem = MemoryBus::default();
    let mut reg = Registers::default();

    let data = 0b1011_1011;
    let expected = 0b1101_1101;
    reg.set_reg8(Register::A, data);

    reg.set_flag(Flag::Zero, true);
    reg.set_flag(Flag::Subtract, true);
    reg.set_flag(Flag::HalfCarry, true);

    let _ = execute_instruction(0x0F, &mut reg, &mut mem);

    assert_eq!(expected, reg.get_reg8(Register::A));
    assert_eq!(false, reg.get_flag(Flag::Zero));
    assert_eq!(false, reg.get_flag(Flag::Subtract));
    assert_eq!(false, reg.get_flag(Flag::HalfCarry));
    assert_eq!(true, reg.get_flag(Flag::Carry));
}
