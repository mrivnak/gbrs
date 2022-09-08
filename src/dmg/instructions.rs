use super::data::Address;
use super::memory::MemoryBus;
use super::registers::{RegisterTarget, Registers};

struct Instruction {
    src_reg: Option<(RegisterTarget, Option<RegisterTarget>)>,
    src_addr: Option<Address>,
    dest_reg: Option<(RegisterTarget, Option<RegisterTarget>)>,
    dest_addr: Option<Address>,
    operation: Operation,
    cycles: u8,
}

enum Operation {
    NOP,
}

fn get_instruction(code: u16) -> Instruction {
    match code {
        0x00 => Instruction {
            src_reg: None,
            src_addr: None,
            dest_reg: None,
            dest_addr: None,
            operation: Operation::NOP,
            cycles: 4,
        },
        _ => panic!("Unsupported instruction: {code}"),
    }
}

pub fn execute_instruction(code: u16, reg: &mut Registers, mem: &mut MemoryBus) -> u8 {
    let instruction = get_instruction(code);

    match instruction.operation {
        Operation::NOP => {}
    }

    instruction.cycles
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
