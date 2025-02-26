use crate::cpu::CPU;
use crate::memory::Memory;

pub struct Emulator {
    cpu: CPU,
    memory: Memory,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.read_byte(self.cpu.pc);
            self.cpu.execute(opcode, &mut self.memory);
        }
    }
}
