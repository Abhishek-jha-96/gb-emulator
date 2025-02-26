mod cpu;
mod memory;
mod emulator;
mod opcodes;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    emulator.run();
}