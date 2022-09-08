const MEM_SIZE: usize = 0x10000;

#[derive(Clone, Debug, PartialEq)]
pub struct MemoryBus {
    memory: Vec<u8>,
}

impl MemoryBus {
    pub fn create() -> MemoryBus {
        MemoryBus { memory: Vec::with_capacity(MEM_SIZE)}
    }
}