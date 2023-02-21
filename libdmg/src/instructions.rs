use core::panic;

use super::data::Address;
use super::memory::MemoryBus;
use super::registers::{Register, RegisterPair, Registers};

struct Instruction {
    src: Option<InstructionTarget>,
    dest: Option<InstructionTarget>,
    operation: Operation,
    cycles: u8,
    length: u8,
}

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
    N8(Address),
    N16(Address),
}

enum Operation {
    NOP,
    LD,
}

enum OpSize {
    EIGHT,
    SIXTEEN,
}

fn get_op_size(target: &InstructionTarget) -> OpSize {
    match target {
        InstructionTarget::A => OpSize::EIGHT,
        InstructionTarget::B => OpSize::EIGHT,
        InstructionTarget::C => OpSize::EIGHT,
        InstructionTarget::D => OpSize::EIGHT,
        InstructionTarget::E => OpSize::EIGHT,
        InstructionTarget::H => OpSize::EIGHT,
        InstructionTarget::L => OpSize::EIGHT,
        InstructionTarget::BC => OpSize::SIXTEEN,
        InstructionTarget::DE => OpSize::SIXTEEN,
        InstructionTarget::HL => OpSize::SIXTEEN,
        InstructionTarget::N8(_) => OpSize::EIGHT,
        InstructionTarget::N16(_) => OpSize::SIXTEEN,
    }
}

fn get_instruction(code: u16, reg: &Registers) -> Instruction {
    match code {
        0x00 => Instruction { // NOP
            src: None,
            dest: None,
            operation: Operation::NOP,
            cycles: 4,
            length: 1
        },
        0x01 => Instruction { // LD BC, u16
            src: Some(InstructionTarget::N16(reg.pc + 1)),
            dest: Some(InstructionTarget::BC),
            operation: Operation::LD,
            cycles: 12,
            length: 3
        },
        0x02 => Instruction { // LD (BC), A
            src: Some(InstructionTarget::A),
            dest: Some(InstructionTarget::N8(reg.get_reg16(RegisterPair::BC))),
            operation: Operation::LD,
            cycles: 4,
            length: 1
        },
        _ => panic!("Unsupported instruction: {code}"),
    }
}

pub fn execute_instruction(code: u16, reg: &mut Registers, mem: &mut MemoryBus) -> u8 {
    let instr = get_instruction(code, reg);

    match instr.operation {
        Operation::NOP => {}
        Operation::LD => {
            let src = instr.src.unwrap();
            let dest = instr.dest.unwrap();
            
            match get_op_size(&src) {
                OpSize::EIGHT => {
                    let data = get_x8(src, reg, mem);
                    set_x8(dest, reg, mem, data);
                }
                OpSize::SIXTEEN => {
                    let data = get_x16(src, reg, mem);
                    set_x16(dest, reg, mem, data);
                }
            }
        }
    }

    // Increment the program counter
    reg.pc += instr.length as u16;

    instr.cycles
}

fn get_x8(
    target: InstructionTarget,
    reg: &mut Registers,
    mem: &mut MemoryBus
) -> u8 {
    match target {
        InstructionTarget::A => reg.get_reg8(Register::A),
        InstructionTarget::B => reg.get_reg8(Register::B),
        InstructionTarget::C => reg.get_reg8(Register::C),
        InstructionTarget::D => reg.get_reg8(Register::D),
        InstructionTarget::E => reg.get_reg8(Register::E),
        InstructionTarget::H => reg.get_reg8(Register::H),
        InstructionTarget::L => reg.get_reg8(Register::L),
        InstructionTarget::N8(addr) => mem.read(addr),
        _ => panic!("Unsupported target for 8-bit value read"),
    }
}

fn get_x16(
    target: InstructionTarget,
    reg: &mut Registers,
    mem: &mut MemoryBus
) -> u16 {
    match target {
        InstructionTarget::BC => reg.get_reg16(RegisterPair::BC),
        InstructionTarget::DE => reg.get_reg16(RegisterPair::DE),
        InstructionTarget::HL => reg.get_reg16(RegisterPair::HL),
        InstructionTarget::N16(addr) => {
            ((mem.read(addr + 1) as u16) << 8) | mem.read(addr) as u16
        }
        _ => panic!("Unsupported target for 16-bit value read"),
    }
}

fn set_x8(
    target: InstructionTarget,
    reg: &mut Registers,
    mem: &mut MemoryBus,
    value: u8
) {
    match target {
        InstructionTarget::A => reg.set_reg8(Register::A, value),
        InstructionTarget::B => reg.set_reg8(Register::B, value),
        InstructionTarget::C => reg.set_reg8(Register::C, value),
        InstructionTarget::D => reg.set_reg8(Register::D, value),
        InstructionTarget::E => reg.set_reg8(Register::E, value),
        InstructionTarget::H => reg.set_reg8(Register::H, value),
        InstructionTarget::L => reg.set_reg8(Register::L, value),
        InstructionTarget::N8(addr) => mem.write(addr, value),
        _ => panic!("Unsupported target for 8-bit value write"),
    }
}

fn set_x16(
    target: InstructionTarget,
    reg: &mut Registers,
    mem: &mut MemoryBus,
    value: u16
) {
    match target {
        InstructionTarget::BC => reg.set_reg16(RegisterPair::BC, value),
        InstructionTarget::DE => reg.set_reg16(RegisterPair::DE, value),
        InstructionTarget::HL => reg.set_reg16(RegisterPair::HL, value),
        InstructionTarget::N16(addr) => {
            mem.write(addr + 1, (value >> 8) as u8);
            mem.write(addr, value as u8);
        }
        _ => panic!("Unsupported target for 16-bit value write"),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
