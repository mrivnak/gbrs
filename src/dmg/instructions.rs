use core::panic;

use super::data::Address;
use super::memory::MemoryBus;
use super::registers::{Register, RegisterPair, Registers};

struct Instruction {
    src: Option<InstructionTarget>,
    dest: Option<InstructionTarget>,
    address: Option<Address>,
    operation: Operation,
    op_size: Option<OpSize>,
    cycles: u8,
}

// 16 and 8-bit targets could be moved to a separate enum to have some compile-time checking of instructions
#[allow(non_camel_case_types)]
enum InstructionTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    N8,
    N16,
    ADD_HL,
}

enum Operation {
    NOP,
    LD,
}

enum OpSize {
    EIGHT,
    SIXTEEN,
}

fn get_instruction(code: u16, reg: &Registers) -> Instruction {
    match code {
        0x00 => Instruction {
            src: None,
            dest: None,
            address: None,
            operation: Operation::NOP,
            op_size: None,
            cycles: 4,
        },
        0x01 => Instruction {
            src: Some(InstructionTarget::N16),
            dest: Some(InstructionTarget::BC),
            address: Some(reg.pc + 1),
            operation: Operation::LD,
            op_size: Some(OpSize::SIXTEEN),
            cycles: 12,
        },
        _ => panic!("Unsupported instruction: {code}"),
    }
}

pub fn execute_instruction(code: u16, reg: &mut Registers, mem: &mut MemoryBus) -> u8 {
    let instruction = get_instruction(code, reg);

    match instruction.operation {
        Operation::NOP => {}
        Operation::LD => {
            todo!()
        }
    }

    instruction.cycles
}

fn get_x8(
    target: InstructionTarget,
    addr: Address,
    reg: &mut Registers,
    mem: &mut MemoryBus,
) -> u8 {
    match target {
        InstructionTarget::A => reg.get_reg8(Register::A),
        InstructionTarget::B => reg.get_reg8(Register::B),
        InstructionTarget::C => reg.get_reg8(Register::C),
        InstructionTarget::D => reg.get_reg8(Register::D),
        InstructionTarget::E => reg.get_reg8(Register::E),
        InstructionTarget::H => reg.get_reg8(Register::H),
        InstructionTarget::L => reg.get_reg8(Register::L),
        InstructionTarget::N8 => mem.read(addr),
        InstructionTarget::ADD_HL => mem.read(reg.get_reg16(RegisterPair::HL)),
        _ => panic!("Unsupported target for 8-bit value read"),
    }
}

fn get_x16(
    target: InstructionTarget,
    addr: Address,
    reg: &mut Registers,
    mem: &mut MemoryBus,
) -> u16 {
    match target {
        InstructionTarget::BC => reg.get_reg16(RegisterPair::BC),
        InstructionTarget::DE => reg.get_reg16(RegisterPair::DE),
        InstructionTarget::HL => reg.get_reg16(RegisterPair::HL),
        InstructionTarget::N16 => ((mem.read(addr + 1) as u16) << 8) | mem.read(addr) as u16,
        _ => panic!("Unsupported target for 16-bit value read"),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
