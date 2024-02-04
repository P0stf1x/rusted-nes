#![allow(dead_code)]

pub static mut VBLANK_READS_VALUE: u32 = 0;

pub const MEMORY_SIZE: usize = 0x10000;

pub struct MEM {
    pub data: Vec<u8>,
}

impl MEM {
    pub fn read(&mut self, address: usize, size: usize) -> usize {
        // TODO: ADD SUPPORT FOR MIRRORING
        let mut val: usize = 0;
        for i in 0..size {
            val += (self.data[address+i] as usize) << 8*i
        }
        return val;
    }

    pub fn write(&mut self, address: usize, data: u8) {
        // TODO: ADD SUPPORT FOR MIRRORING
        // TODO: ADD SUPPORT FOR ROM
        self.data[address] = data;
    }

    pub fn write_bulk(&mut self, address: usize, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write(address + i, *byte);
        }
    }

    pub fn new(memory_size: usize) -> Self {
        let data = vec![0u8; memory_size];
        Self{
            data
        }
    }

    // This just loads memory dump instead of loading rom files
    pub fn new_from(file_path: &String) -> Self {
        use std::fs;

        let data = fs::read(file_path)
        .expect("Should have been able to read the file");

        let mut memory = MEM::new(data.len());
        memory.data = data;

        return memory;
    }
}

pub const PRG_ROM_ADDR: usize = 0x8000;
pub const PRG_ROM_ENTRY_ADDR: usize = 0xC000;
