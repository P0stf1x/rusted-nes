#![allow(dead_code)]

use std::fs::File;

pub static mut VBLANK_READS_VALUE: u32 = 0;

pub const MEMORY_SIZE: usize = 0x10000;

pub struct MemoryRegion {
    region_address: usize,
    region_size: usize,
}

impl MemoryRegion {
    pub fn intersects_region(&self, region: &MemoryRegion) -> bool {
        let region1_start = self.region_address;
        let region1_end = self.region_address + self.region_size - 1;
        let region2_start = region.region_address;
        let region2_end = region.region_address + region.region_size - 1;

        region1_start <= region2_end && region1_end >= region2_start
    }

    pub fn inside_region(&self, address: usize) -> bool {
        let region_start = self.region_address;
        let region_end = self.region_address + self.region_size - 1;

        address >= region_start && address <= region_end
    }
}

#[cfg(test)]
mod memory_region_test {
    use super::*;

    fn test_intersection(addr1: usize, addr2: usize, size1: usize, size2: usize, expected_result: bool) {
        let a: MemoryRegion = MemoryRegion { region_address: addr1, region_size: size1 };
        let b: MemoryRegion = MemoryRegion { region_address: addr2, region_size: size2 };
        
        assert_eq!(a.intersects_region(&b), expected_result);
    }

    #[test]
    fn not_intersects_on_left() {
        test_intersection(0x0000, 0x0004, 2, 4, false);
    }

    #[test]
    fn not_intersects_on_left_touches() {
        test_intersection(0x0000, 0x0004, 4, 4, false);
    }

    #[test]
    fn intersects_on_left() {
        test_intersection(0x0002, 0x0004, 4, 4, true);
    }

    #[test]
    fn intersects_same() {
        test_intersection(0x0004, 0x0004, 4, 4, true);
    }

    #[test]
    fn intersects_smaller() {
        test_intersection(0x0005, 0x0004, 2, 4, true);
    }

    #[test]
    fn intersects_bigger() {
        test_intersection(0x0003, 0x0004, 6, 4, true);
    }

    #[test]
    fn intersects_on_right() {
        test_intersection(0x0006, 0x0004, 4, 4, true);
    }

    #[test]
    fn not_intersects_on_right_touches() {
        test_intersection(0x0008, 0x0004, 4, 4, false);
    }

    #[test]
    fn not_intersects_on_right() {
        test_intersection(0x000A, 0x0004, 2, 4, false);
    }

    #[test]
    fn not_inside_on_left() {
        let a: MemoryRegion = MemoryRegion { region_address: 0x0004, region_size: 4 };
        assert_eq!(a.inside_region(0x0002), false);
    }

    #[test]
    fn not_inside_on_left_touches() {
        let a: MemoryRegion = MemoryRegion { region_address: 0x0004, region_size: 4 };
        assert_eq!(a.inside_region(0x0003), false);
    }

    #[test]
    fn inside() {
        let a: MemoryRegion = MemoryRegion { region_address: 0x0004, region_size: 4 };
        assert_eq!(a.inside_region(0x0004), true);
        assert_eq!(a.inside_region(0x0005), true);
        assert_eq!(a.inside_region(0x0006), true);
        assert_eq!(a.inside_region(0x0007), true);
    }

    #[test]
    fn not_inside_on_right_touches() {
        let a: MemoryRegion = MemoryRegion { region_address: 0x0004, region_size: 4 };
        assert_eq!(a.inside_region(0x0008), false);
    }

    #[test]
    fn not_inside_on_right() {
        let a: MemoryRegion = MemoryRegion { region_address: 0x0004, region_size: 4 };
        assert_eq!(a.inside_region(0x0009), false);
    }
}

pub struct MemoryMirror {
    physical_memory: MemoryRegion,
    mirrored_memory: MemoryRegion,
}

pub struct MEM {
    pub data: Vec<u8>,
    mirroring: Vec<MemoryMirror>,
}

// Read/Write
impl MEM {
    pub fn read(&mut self, address: usize, size: usize) -> usize {
        let mut val: usize = 0;
        for i in 0..size {
            let mirrored_address = self.get_mirrored_address(address+i);
            val += (self.data[mirrored_address] as usize) << 8*i
        }
        return val;
    }

    pub fn write(&mut self, address: usize, data: u8) {
        // TODO: ADD SUPPORT FOR ROM
        let mirrored_address = self.get_mirrored_address(address);
        self.data[mirrored_address] = data;
    }

    pub fn write_bulk(&mut self, address: usize, data: Vec<u8>) {
        for (i, byte) in data.iter().enumerate() {
            self.write(address + i, *byte);
        }
    }
}

#[cfg(test)]
mod read_write_test {
    use super::*;

    #[test]
    fn read_zero() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.data[0x0000], 0x00, "Couldn't prepare memory for test");
        assert_eq!(test_memory.read(0x0000, 1), 0x00);
    }

    #[test]
    fn read() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.data[0x0000] = 0xff;
        assert_eq!(test_memory.data[0x0000], 0xff, "Couldn't prepare memory for test");
        assert_eq!(test_memory.read(0x0000, 1), 0xff);
    }

    #[test]
    fn read_bulk() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.data[0x0000] = 0xff;
        test_memory.data[0x0001] = 0xff;
        assert_eq!(test_memory.data[0x0000], 0xff, "Couldn't prepare memory for test");
        assert_eq!(test_memory.data[0x0001], 0xff, "Couldn't prepare memory for test");
        assert_eq!(test_memory.read(0x0000, 2), 0xffff);
    }

    #[test]
    fn write() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.read(0x0000, 1), 0x00, "Couldn't prepare memory for test");
        
        test_memory.write(0x0000, 0xff);
        
        assert_eq!(test_memory.read(0x0000, 1), 0xff);
    }

    #[test]
    fn write_bulk() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.read(0x0000, 2), 0x0000, "Couldn't prepare memory for test");
        
        test_memory.write_bulk(0x0000, vec![0xff, 0xff]);
        
        assert_eq!(test_memory.read(0x0000, 2), 0xffff);
    }
}

// Constructors
impl MEM {
    pub fn new(memory_size: usize) -> Self {
        let data = vec![0u8; memory_size];
        Self{
            data,
            mirroring: vec![],
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

// Mirroring
impl MEM {
    fn push_mirrored_range(&mut self, new_mirror: MemoryMirror) -> Result<(), &'static str> {
        for mirror in &self.mirroring {
            if new_mirror.mirrored_memory.intersects_region(&mirror.mirrored_memory) {
                return Err("Memory already mirrored");
            }
        }
        self.mirroring.push(new_mirror);
        return Ok(());
    }

    pub fn load_mapper_mirroring(&mut self, mapper: File) {
        // TODO
        todo!();
    }

    fn is_mirrored(&self, address: usize) -> Option<&MemoryMirror> {
        for mirror in &self.mirroring {
            if mirror.mirrored_memory.inside_region(address) {
                return Some(mirror);
            }
        }
        return None;
    }

    fn get_mirrored_address(&self, address: usize) -> usize {
        // TODO: Room for efficiency improvements
        let mut mirrored_address = address;
        if let Some(mirror) = self.is_mirrored(address) {
            let physical_start = mirror.physical_memory.region_address;
            let mirrored_start = mirror.mirrored_memory.region_address;
            mirrored_address = address - mirrored_start - physical_start;
        }
        return mirrored_address;
    }
}

pub const PRG_ROM_ADDR: usize = 0x8000;
pub const PRG_ROM_ENTRY_ADDR: usize = 0xC000;

#[cfg(test)]
mod mirroring_tests {
    use super::*;

    #[test]
    fn test_memory_no_mirroring() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.data[0x0000], 0x00);
        assert_eq!(test_memory.read(0x0000, 1), 0x00);
        assert_eq!(test_memory.data[0x0001], 0x00);
        assert_eq!(test_memory.read(0x0001, 1), 0x00);

        test_memory.write(0x0001, 0xCD);

        assert_eq!(test_memory.data[0x0000], 0x00);
        assert_eq!(test_memory.read(0x0000, 1), 0x00);
        assert_eq!(test_memory.data[0x0001], 0xCD);
        assert_eq!(test_memory.read(0x0001, 1), 0xCD);

        test_memory.write(0x0001, 0xAB);

        assert_eq!(test_memory.data[0x0000], 0x00);
        assert_eq!(test_memory.read(0x0000, 1), 0x00);
        assert_eq!(test_memory.data[0x0001], 0xAB);
        assert_eq!(test_memory.read(0x0001, 1), 0xAB);
    }
    
    #[test]
    fn test_memory_mirroring() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.write(0x0001, 0xCD);

        assert_eq!(test_memory.read(0x0000, 1), 0x00, "Couldn't prepare memory for test");
        assert_eq!(test_memory.read(0x0001, 1), 0xCD, "Couldn't prepare memory for test");

        test_memory.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0000,
                region_size: 1,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0001,
                region_size: 1,
            },
        }).unwrap();

        test_memory.write(0x0001, 0xAB);

        assert_eq!(test_memory.data[0x0000], 0xAB);
        assert_eq!(test_memory.read(0x0000, 1), 0xAB);
        assert_eq!(test_memory.data[0x0001], 0xCD);
        assert_eq!(test_memory.read(0x0001, 1), 0xAB);
    }

    #[test]
    fn test_memory_mirroring_check() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0000,
                region_size: 1,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0001,
                region_size: 1,
            },
        }).unwrap();

        assert_eq!(test_memory.is_mirrored(0x0000).is_some(), false);
        assert_eq!(test_memory.is_mirrored(0x0001).is_some(), true);
    }

    #[test]
    fn get_mirrored_address() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0000,
                region_size: 1,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0001,
                region_size: 1,
            },
        }).unwrap();

        assert_eq!(test_memory.get_mirrored_address(0x0000), 0x0000);
        assert_eq!(test_memory.get_mirrored_address(0x0001), 0x0000);
    }
}