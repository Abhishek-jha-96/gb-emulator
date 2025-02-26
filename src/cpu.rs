use crate::memory::Memory;
use crate::opcodes::execute_opcode;

pub struct CPU {
    pub pc: usize, // Program Counter
    pub sp: u16,   // Stack Pointer
    pub a: u8,     // Register A
}

impl CPU {
    pub fn new() -> Self {
        CPU { pc: 0, sp: 0, a: 0 }
    }

    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) {
        execute_opcode(self, opcode, memory);
    }
}
