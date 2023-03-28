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

#[derive(Default)]
struct FlagInstruction {
    zero: FlagOperation,
    subtract: FlagOperation,
    half_carry: FlagOperation,
    carry: FlagOperation,
}

#[derive(Default)]
struct FlagResults {
    zero: Option<FlagResult>,
    subtract: Option<FlagResult>,
    half_carry: Option<FlagResult>,
    carry: Option<FlagResult>,
}

#[derive(Default)]
enum FlagOperation {
    #[default]
    Unmodified,
    Dependent,
    Set,
    Unset,
}

enum FlagResult {
    Set,
    Unset,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    RXC(Direction),
}

enum Direction {
    Left,
    Right,
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

fn target_to_register(target: InstructionTarget) -> Register {
    match target {
        InstructionTarget::A => Register::A,
        InstructionTarget::B => Register::B,
        InstructionTarget::C => Register::C,
        InstructionTarget::D => Register::D,
        InstructionTarget::E => Register::E,
        InstructionTarget::H => Register::H,
        InstructionTarget::L => Register::L,
        _ => panic!("Invalid target: {:?}, cannot convert to register", target),
    }
}

fn target_to_register_pair(target: InstructionTarget) -> RegisterPair {
    match target {
        InstructionTarget::BC => RegisterPair::BC,
        InstructionTarget::DE => RegisterPair::DE,
        InstructionTarget::HL => RegisterPair::HL,
        _ => panic!(
            "Invalid target: {:?}, cannot convert to register pair",
            target
        ),
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
            flags: FlagInstruction::default(),
        },
        0x01 => Instruction {
            // LD BC, u16
            source: Some(InstructionTarget::N16(reg.pc + 1)),
            target: Some(InstructionTarget::BC),
            operation: Operation::LD,
            cycles: 12,
            length: 3,
            flags: FlagInstruction::default(),
        },
        0x02 => Instruction {
            // LD (BC), A
            source: Some(InstructionTarget::A),
            target: Some(InstructionTarget::N8(reg.get_reg16(RegisterPair::BC))),
            operation: Operation::LD,
            cycles: 4,
            length: 1,
            flags: FlagInstruction::default(),
        },
        0x03 => Instruction {
            // INC BC
            source: None,
            target: Some(InstructionTarget::BC),
            operation: Operation::INC,
            cycles: 8,
            length: 1,
            flags: FlagInstruction::default(),
        },
        0x04 => Instruction {
            // INC B
            source: None,
            target: Some(InstructionTarget::B),
            operation: Operation::INC,
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                zero: FlagOperation::Dependent,
                subtract: FlagOperation::Unset,
                half_carry: FlagOperation::Dependent,
                ..Default::default()
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
                zero: FlagOperation::Dependent,
                subtract: FlagOperation::Set,
                half_carry: FlagOperation::Dependent,
                ..Default::default()
            },
        },
        0x06 => Instruction {
            // LD B, u8
            source: Some(InstructionTarget::N8(reg.pc + 1)),
            target: Some(InstructionTarget::B),
            operation: Operation::LD,
            cycles: 8,
            length: 2,
            flags: FlagInstruction::default(),
        },
        0x07 => Instruction {
            // RLCA
            source: None,
            target: Some(InstructionTarget::A),
            operation: Operation::RXC(Direction::Left),
            cycles: 4,
            length: 1,
            flags: FlagInstruction {
                zero: FlagOperation::Unset,
                subtract: FlagOperation::Unset,
                half_carry: FlagOperation::Unset,
                carry: FlagOperation::Dependent,
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
            let mut results = FlagResults::default();

            match get_op_size(&target) {
                InstructionSize::Eight => {
                    let value = get_x8(&target, reg, mem);
                    set_x8(&target, reg, mem, value + 1);
                    results.zero = Some(FlagResult::Unset);
                    results.half_carry =
                        Some(match check_half_carry8(ArithmeticMode::Add, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        });
                }
                InstructionSize::Sixteen => {
                    let value = get_x16(&target, reg, mem);
                    set_x16(&target, reg, mem, value + 1);
                    results.zero = Some(FlagResult::Unset);
                    results.half_carry =
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
            let mut results = FlagResults::default();

            match get_op_size(&target) {
                InstructionSize::Eight => {
                    let value = get_x8(&target, reg, mem);
                    // TODO: check for integer overflow
                    set_x8(&target, reg, mem, value - 1);
                    results.zero = Some(match value - 1 {
                        0 => FlagResult::Set,
                        _ => FlagResult::Unset,
                    });
                    results.half_carry = Some(
                        match check_half_carry8(ArithmeticMode::Subtract, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        },
                    );
                }
                InstructionSize::Sixteen => {
                    let value = get_x16(&target, reg, mem);
                    set_x16(&target, reg, mem, value - 1);
                    results.zero = Some(match value - 1 {
                        0 => FlagResult::Set,
                        _ => FlagResult::Unset,
                    });
                    results.half_carry = Some(
                        match check_half_carry16(ArithmeticMode::Subtract, value, 1) {
                            true => FlagResult::Set,
                            false => FlagResult::Unset,
                        },
                    );
                }
            }

            modify_flags(reg, instr.flags, results)
        }
        Operation::RXC(direction) => {
            let mut results = FlagResults::default();
            match direction {
                Direction::Left => {
                    let register = target_to_register(instr.target.expect("No target provided"));
                    let value = reg.get_reg8(register.clone());
                    let bit7 = (value & 0x80) >> 7;

                    reg.set_reg8(register, (value << 1) + bit7);

                    results.carry = Some(match bit7 {
                        0 => FlagResult::Unset,
                        1 => FlagResult::Set,
                        _ => unreachable!(),
                    });
                }
                Direction::Right => {
                    todo!("RRC Not implemented");
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
    let zero = match (instr.zero, results.zero) {
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

    let subtract = match (instr.subtract, results.subtract) {
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

    let half_carry = match (instr.half_carry, results.half_carry) {
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

    let carry = match (instr.carry, results.carry) {
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
