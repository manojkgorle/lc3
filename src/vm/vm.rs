use crate::vm::registers::Registers;
use crate::MEMORY_SIZE;
pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn read_memory(&self, address: usize) -> u16 {
        self.memory[address]
    }
}
