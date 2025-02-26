use crate::cpu::CPU;
use crate::memory::Memory;

pub fn execute_opcode(cpu: &mut CPU, opcode: u8, memory: &mut Memory) {
    match opcode {
        0x31 => {
            let low = memory.read_byte(cpu.pc + 1) as u16;
            let high = memory.read_byte(cpu.pc + 2) as u16;
            cpu.sp = (high << 8) | low;
            cpu.pc += 3;
        }
        0xAF => {
            cpu.a ^= cpu.a;
            cpu.pc += 1;
        }
        _ => panic!("Unknown opcode {:#04x} at address {:#06x}", opcode, cpu.pc),
    }
}
