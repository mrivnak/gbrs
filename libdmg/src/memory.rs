use std::vec;

use super::data::Address;

const MEM_SIZE: usize = 0x10000;

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryBus {
    memory: Vec<u8>,
}

impl MemoryBus {
    pub fn write(&mut self, addr: Address, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read(&self, addr: Address) -> u8 {
        self.memory[addr as usize]
    }
}

impl Default for MemoryBus {
    fn default() -> MemoryBus {
        MemoryBus {
            memory: vec![0; MEM_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut mem = MemoryBus::default();

        let addresses = vec![0x0000, 0x00FF, 0xFFFF];
        let values = vec![0x0F, 0xF0, 0xAA];

        for addr in &addresses {
            for val in &values {
                mem.write(*addr, *val);
                assert_eq!(*val, mem.memory[*addr as usize])
            }
        }
    }

    #[test]
    fn test_read() {
        let mut mem = MemoryBus::default();

        let addresses = vec![0x0000, 0x00FF, 0xFFFF];
        let values = vec![0x0F, 0xF0, 0xAA];

        for addr in &addresses {
            for val in &values {
                mem.memory[*addr as usize] = *val;
                let result = mem.read(*addr);
                assert_eq!(result, *val)
            }
        }
    }
}
