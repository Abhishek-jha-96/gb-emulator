pub struct Memory {
    high_ram: [u8; 127],
    video_ram: [u8; 8192],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            high_ram: [0; 127],
            video_ram: [0; 8192],
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            0xFF80..=0xFFFE => self.high_ram[address - 0xFF80],
            0x8000..=0x9FFF => self.video_ram[address - 0x8000],
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            0xFF80..=0xFFFE => self.high_ram[address - 0xFF80] = value,
            0x8000..=0x9FFF => self.video_ram[address - 0x8000] = value,
            _ => panic!("Unsupported memory address {:#06x}", address),
        }
    }
}
