use super::instructions::{self, execute_instruction};
use super::memory::MemoryBus;
use super::registers::{Register, Registers};

pub struct CPU {
    memory: MemoryBus,
    registers: Registers,
}

impl CPU {
    pub fn create() -> CPU {
        CPU {
            memory: MemoryBus::create(),
            registers: Registers::create(),
        }
    }

    pub fn tick(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) {
        let cycles = instructions::execute_instruction(0x00, &mut self.registers, &mut self.memory);
    }
}
