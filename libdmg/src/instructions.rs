use core::panic;

use super::data::Address;
use super::flags::*;
use super::memory::MemoryBus;
use super::registers::{Flag, Register, RegisterPair, Registers};

struct Instruction {
    source: Option<InstructionTarget>,
    target: Option<InstructionTarget>,
    operation: Operation,
    cycles: u8,
    length: u8,
    flags: FlagInstruction,
}

struct FlagInstruction {
    Zero: FlagOperation,
    Subtract: FlagOperation,
    HalfCarry: FlagOperation,
    Carry: FlagOperation,
}

struct FlagResults {
    Zero: Option<FlagResult>,
    Subtract: Option<FlagResult>,
    HalfCarry: Option<FlagResult>,
    Carry: Option<FlagResult>,
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
    INC,
    DEC,
}

enum FlagOperation {
    Unmodified,
    Dependent,
    Set,
    Unset,
}

enum FlagResult {
    Set,
    Unset,
}

enum InstructionSize {
    Eight,
    Sixteen,
}

fn get_op_size(target: &InstructionTarget) -> InstructionSize {
    match target {
        InstructionTarget::A => InstructionSize::Eight,
        InstructionTarget::B => InstructionSize::Eight,
        InstructionTarget::C => InstructionSize::Eight,
        InstructionTarget::D => InstructionSize::Eight,
        InstructionTarget::E => InstructionSize::Eight,
        InstructionTarget::H => InstructionSize::Eight,
        InstructionTarget::L => InstructionSize::Eight,
        InstructionTarget::BC => InstructionSize::Sixteen,
        InstructionTarget::DE => InstructionSize::Sixteen,
        InstructionTarget::HL => InstructionSize::Sixteen,
        InstructionTarget::N8(_) => InstructionSize::Eight,
        InstructionTarget::N16(_) => InstructionSize::Sixteen,
    }
}

fn get_instruction(code: u16, reg: &Registers) -> Instruction {
    match code {
        0x00 => Instruction {
            // NOP
            source: None,
            target: None,
            operation: Operation::NOP,
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                Zero: FlagOperation::Unmodified,
                Subtract: FlagOperation::Unmodified,
                HalfCarry: FlagOperation::Unmodified,
                Carry: FlagOperation::Unmodified,
            },
        },
        0x01 => Instruction {
            // LD BC, u16
            source: Some(InstructionTarget::N16(reg.pc + 1)),
            target: Some(InstructionTarget::BC),
            operation: Operation::LD,
            cycles: 12,
            length: 3,
            flags: FlagInstruction {
                Zero: FlagOperation::Unmodified,
                Subtract: FlagOperation::Unmodified,
                HalfCarry: FlagOperation::Unmodified,
                Carry: FlagOperation::Unmodified,
            },
        },
        0x02 => Instruction {
            // LD (BC), A
            source: Some(InstructionTarget::A),
            target: Some(InstructionTarget::N8(reg.get_reg16(RegisterPair::BC))),
            operation: Operation::LD,
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                Zero: FlagOperation::Unmodified,
                Subtract: FlagOperation::Unmodified,
                HalfCarry: FlagOperation::Unmodified,
                Carry: FlagOperation::Unmodified,
            },
        },
        0x03 => Instruction {
            // INC BC
            source: None,
            target: Some(InstructionTarget::BC),
            operation: Operation::INC,
            cycles: 8,
            length: 1,
            flags: FlagInstruction {
                Zero: FlagOperation::Unmodified,
                Subtract: FlagOperation::Unmodified,
                HalfCarry: FlagOperation::Unmodified,
                Carry: FlagOperation::Unmodified,
            },
        },
        0x04 => Instruction {
            // INC B
            source: None,
            target: Some(InstructionTarget::B),
            operation: Operation::INC,
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                Zero: FlagOperation::Dependent,
                Subtract: FlagOperation::Unset,
                HalfCarry: FlagOperation::Dependent,
                Carry: FlagOperation::Unmodified,
            },
        },
        0x05 => Instruction {
            // DEC B
            source: None,
            target: Some(InstructionTarget::B),
            operation: Operation::DEC,
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                Zero: FlagOperation::Dependent,
                Subtract: FlagOperation::Set,
                HalfCarry: FlagOperation::Dependent,
                Carry: FlagOperation::Unmodified,
            },
        },
        _ => panic!("Unsupported instruction: {code}"),
    }
}

pub fn execute_instruction(code: u16, reg: &mut Registers, mem: &mut MemoryBus) -> u8 {
    let instr = get_instruction(code, reg);

    match instr.operation {
        Operation::NOP => {}
        Operation::LD => {
            let source = instr.source.unwrap();
            let target = instr.target.unwrap();

            match get_op_size(&source) {
                InstructionSize::Eight => {
                    let value = get_x8(&source, reg, mem);
                    set_x8(&target, reg, mem, value);
                }
                InstructionSize::Sixteen => {
                    let value = get_x16(&source, reg, mem);
                    set_x16(&target, reg, mem, value);
                }
            }
        }
        Operation::INC => {
            let target = instr.target.unwrap();
            let mut results = FlagResults {
                Zero: None,
                Subtract: None,
                HalfCarry: None,
                Carry: None,
            };

            match get_op_size(&target) {
                InstructionSize::Eight => {
                    let value = get_x8(&target, reg, mem);
                    set_x8(&target, reg, mem, value + 1);
                    results.Zero = Some(FlagResult::Unset);
                    results.HalfCarry =
                        Some(match check_half_carry8(ArithmeticMode::Add, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        });
                }
                InstructionSize::Sixteen => {
                    let value = get_x16(&target, reg, mem);
                    set_x16(&target, reg, mem, value + 1);
                    results.Zero = Some(FlagResult::Unset);
                    results.HalfCarry =
                        Some(match check_half_carry16(ArithmeticMode::Add, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        });
                }
            }

            modify_flags(reg, instr.flags, results)
        }
        Operation::DEC => {
            let target = instr.target.unwrap();
            let mut results = FlagResults {
                Zero: None,
                Subtract: None,
                HalfCarry: None,
                Carry: None,
            };

            match get_op_size(&target) {
                InstructionSize::Eight => {
                    let value = get_x8(&target, reg, mem);
                    // TODO: check for integer overflow
                    set_x8(&target, reg, mem, value - 1);
                    results.Zero = Some(match value - 1 {
                        0 => FlagResult::Set,
                        _ => FlagResult::Unset,
                    });
                    results.HalfCarry = Some(match check_half_carry8(ArithmeticMode::Subtract, value, 1) {
                        true => FlagResult::Set,
                        false => FlagResult::Unset,
                    });
                }
                InstructionSize::Sixteen => {
                    let value = get_x16(&target, reg, mem);
                    set_x16(&target, reg, mem, value - 1);
                    results.Zero = Some(match value - 1 {
                        0 => FlagResult::Set,
                        _ => FlagResult::Unset,
                    });
                    results.HalfCarry =
                        Some(match check_half_carry16(ArithmeticMode::Subtract, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        });
                }
            }

            modify_flags(reg, instr.flags, results)
        }
    }

    // Increment the program counter
    reg.pc += instr.length as u16;

    instr.cycles
}

fn modify_flags(reg: &mut Registers, instr: FlagInstruction, results: FlagResults) {
    let zero = match (instr.Zero, results.Zero) {
        (FlagOperation::Dependent, Some(FlagResult::Set)) => Some(true),
        (FlagOperation::Dependent, Some(FlagResult::Unset)) => Some(false),
        (FlagOperation::Dependent, None) => panic!("Dependent flag not provided result"),
        (FlagOperation::Unset, _) => Some(false),
        (FlagOperation::Set, _) => Some(true),
        (FlagOperation::Unmodified, _) => None,
    };
    match zero {
        Some(value) => reg.set_flag(Flag::Zero, value),
        None => {}
    }

    let subtract = match (instr.Subtract, results.Subtract) {
        (FlagOperation::Dependent, Some(FlagResult::Set)) => Some(true),
        (FlagOperation::Dependent, Some(FlagResult::Unset)) => Some(false),
        (FlagOperation::Dependent, None) => panic!("Dependent flag not provided result"),
        (FlagOperation::Set, _) => Some(true),
        (FlagOperation::Unset, _) => Some(false),
        (FlagOperation::Unmodified, _) => None,
    };
    match subtract {
        Some(value) => reg.set_flag(Flag::Subtract, value),
        None => {}
    }

    let half_carry = match (instr.HalfCarry, results.HalfCarry) {
        (FlagOperation::Dependent, Some(FlagResult::Set)) => Some(true),
        (FlagOperation::Dependent, Some(FlagResult::Unset)) => Some(false),
        (FlagOperation::Dependent, None) => panic!("Dependent flag not provided result"),
        (FlagOperation::Set, _) => Some(true),
        (FlagOperation::Unset, _) => Some(false),
        (FlagOperation::Unmodified, _) => None,
    };
    match half_carry {
        Some(value) => reg.set_flag(Flag::HalfCarry, value),
        None => {}
    }

    let carry = match (instr.Carry, results.Carry) {
        (FlagOperation::Dependent, Some(FlagResult::Set)) => Some(true),
        (FlagOperation::Dependent, Some(FlagResult::Unset)) => Some(false),
        (FlagOperation::Dependent, None) => panic!("Dependent flag not provided result"),
        (FlagOperation::Set, _) => Some(true),
        (FlagOperation::Unset, _) => Some(false),
        (FlagOperation::Unmodified, _) => None,
    };
    match carry {
        Some(value) => reg.set_flag(Flag::Carry, value),
        None => {}
    }
}

fn get_x8(target: &InstructionTarget, reg: &Registers, mem: &MemoryBus) -> u8 {
    match target {
        InstructionTarget::A => reg.get_reg8(Register::A),
        InstructionTarget::B => reg.get_reg8(Register::B),
        InstructionTarget::C => reg.get_reg8(Register::C),
        InstructionTarget::D => reg.get_reg8(Register::D),
        InstructionTarget::E => reg.get_reg8(Register::E),
        InstructionTarget::H => reg.get_reg8(Register::H),
        InstructionTarget::L => reg.get_reg8(Register::L),
        InstructionTarget::N8(addr) => mem.read(*addr),
        _ => panic!("Unsupported target for 8-bit value read"),
    }
}

fn get_x16(target: &InstructionTarget, reg: &Registers, mem: &MemoryBus) -> u16 {
    match target {
        InstructionTarget::BC => reg.get_reg16(RegisterPair::BC),
        InstructionTarget::DE => reg.get_reg16(RegisterPair::DE),
        InstructionTarget::HL => reg.get_reg16(RegisterPair::HL),
        InstructionTarget::N16(addr) => {
            ((mem.read((*addr) + 1) as u16) << 8) | mem.read(*addr) as u16
        }
        _ => panic!("Unsupported target for 16-bit value read"),
    }
}

fn set_x8(target: &InstructionTarget, reg: &mut Registers, mem: &mut MemoryBus, value: u8) {
    match target {
        InstructionTarget::A => reg.set_reg8(Register::A, value),
        InstructionTarget::B => reg.set_reg8(Register::B, value),
        InstructionTarget::C => reg.set_reg8(Register::C, value),
        InstructionTarget::D => reg.set_reg8(Register::D, value),
        InstructionTarget::E => reg.set_reg8(Register::E, value),
        InstructionTarget::H => reg.set_reg8(Register::H, value),
        InstructionTarget::L => reg.set_reg8(Register::L, value),
        InstructionTarget::N8(addr) => mem.write(*addr, value),
        _ => panic!("Unsupported target for 8-bit value write"),
    }
}

fn set_x16(target: &InstructionTarget, reg: &mut Registers, mem: &mut MemoryBus, value: u16) {
    match target {
        InstructionTarget::BC => reg.set_reg16(RegisterPair::BC, value),
        InstructionTarget::DE => reg.set_reg16(RegisterPair::DE, value),
        InstructionTarget::HL => reg.set_reg16(RegisterPair::HL, value),
        InstructionTarget::N16(addr) => {
            mem.write((*addr) + 1, (value >> 8) as u8);
            mem.write(*addr, value as u8);
        }
        _ => panic!("Unsupported target for 16-bit value write"),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
