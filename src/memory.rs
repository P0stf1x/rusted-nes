#![allow(dead_code)]

use std::fs::File;

use ppu_memory::PPU_MEM;

pub mod ines;
pub mod mappers;
pub mod ppu_memory;

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

pub struct WriteProtectedRegion {
    protected_memory: MemoryRegion,
}

pub struct MemoryMirror {
    physical_memory: MemoryRegion,
    mirrored_memory: MemoryRegion,
}

pub struct MEM {
    pub data: Vec<u8>,
    mirroring: Vec<MemoryMirror>,
    write_protection: Vec<WriteProtectedRegion>,
}

// Read/Write
impl MEM {
    pub fn read(&self, address: usize, size: usize) -> usize {
        let mut val: usize = 0;
        for i in 0..size {
            let mirrored_address = self.get_mirrored_address(address+i);
            val += (self.data[mirrored_address] as usize) << 8*i
        }
        return val;
    }

    pub fn write(&mut self, address: usize, data: u8) {
        if !self.is_protected(address) { // or should it check mirrored address?
            let mirrored_address = self.get_mirrored_address(address);
            self.data[mirrored_address] = data;
        }
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
            write_protection: vec![],
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

    pub fn new_from_ines(file_path: &String) -> (Self, PPU_MEM) {
        use std::fs;

        let data = fs::read(file_path)
        .expect("Should have been able to read the file");

        use ines::*;
        let parsed_ines = parse_file(&data);
        println!("{:#?}", parsed_ines.header);
        println!("prg_rom size: {}, {} blocks", parsed_ines.prg_rom.len(), parsed_ines.prg_rom.len()/(16*1024));
        println!("chr_rom size: {}, {} blocks", parsed_ines.chr_rom.len(), parsed_ines.chr_rom.len()/(8*1024));

        let (memory, ppu_memory) = mappers::map(parsed_ines);

        return (memory, ppu_memory);
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
            mirrored_address = (address - mirrored_start) + physical_start;
        }
        return mirrored_address;
    }
}

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
    fn test_memory_mirroring_non_single_range() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.write_bulk(0x0002, vec![0xEF, 0xBE, 0xAD, 0xDE]);

        assert_eq!(test_memory.read(0x0002, 4), 0xDEADBEEF, "Couldn't prepare memory for test 2");

        assert_eq!(test_memory.data[0x0005], 0xDE);
        assert_eq!(test_memory.read(0x0005, 1), 0xDE);
        assert_eq!(test_memory.data[0x0004], 0xAD);
        assert_eq!(test_memory.read(0x0004, 1), 0xAD);
        assert_eq!(test_memory.data[0x0003], 0xBE);
        assert_eq!(test_memory.read(0x0003, 1), 0xBE);
        assert_eq!(test_memory.data[0x0002], 0xEF);
        assert_eq!(test_memory.read(0x0002, 1), 0xEF);

        test_memory.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0002,
                region_size: 2,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0004,
                region_size: 2,
            },
        }).unwrap();

        assert_eq!(test_memory.data[0x0002..=0x0005], [0xEF, 0xBE, 0xAD, 0xDE]);
        assert_eq!(test_memory.read(0x0002, 2), 0xBEEF);
        assert_eq!(test_memory.read(0x0004, 2), 0xBEEF);

        test_memory.mirroring.pop();
        test_memory.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0004,
                region_size: 2,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0002,
                region_size: 2,
            },
        }).unwrap();

        assert_eq!(test_memory.data[0x0002..=0x0005], [0xEF, 0xBE, 0xAD, 0xDE]);
        assert_eq!(test_memory.read(0x0002, 2), 0xDEAD);
        assert_eq!(test_memory.read(0x0004, 2), 0xDEAD);
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

        let mut test_memory1: MEM = MEM::new(MEMORY_SIZE);

        test_memory1.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0002,
                region_size: 2,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0004,
                region_size: 2,
            },
        }).unwrap();
        test_memory1.push_mirrored_range(MemoryMirror{
            physical_memory: MemoryRegion{
                region_address: 0x0002,
                region_size: 2,
            },
            mirrored_memory: MemoryRegion{
                region_address: 0x0006,
                region_size: 4,
            },
        }).unwrap();

        assert_eq!(test_memory1.get_mirrored_address(0x0002), 0x0002);
        assert_eq!(test_memory1.get_mirrored_address(0x0003), 0x0003);
        assert_eq!(test_memory1.get_mirrored_address(0x0004), 0x0002);
        assert_eq!(test_memory1.get_mirrored_address(0x0005), 0x0003);
        assert_eq!(test_memory1.get_mirrored_address(0x0006), 0x0002);
        assert_eq!(test_memory1.get_mirrored_address(0x0007), 0x0003);
    }
}

// Write protection
impl MEM {
    fn push_write_protected_region(&mut self, new_region: WriteProtectedRegion) {
        for protected in &self.write_protection {
            if new_region.protected_memory.intersects_region(&protected.protected_memory) {
                // TODO: ideally it should combine them if they overlap since it could give speed up at runtime, although
                // it most likely wouldn't be called at runtime, and even then it easily could be optimised inside mapper
            }
        }
        self.write_protection.push(new_region);
    }

    fn is_protected(&self, address: usize) -> bool {
        for region in &self.write_protection {
            if region.protected_memory.inside_region(address) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod write_protection_tests {
    use super::*;

    #[test]
    fn test_memory_protection_check() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.is_protected(0xDEAD), false);

        test_memory.push_write_protected_region(WriteProtectedRegion {
            protected_memory: MemoryRegion {
                region_address: 0xBEEF,
                region_size: 1
            }
        });

        assert_eq!(test_memory.is_protected(0xDEAD), false);
        assert_eq!(test_memory.is_protected(0xBEEF), true);
    }

    #[test]
    fn test_memory_not_protected() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        assert_eq!(test_memory.data[0x0000], 0x00);
        test_memory.write(0x0000, 1);
        assert_eq!(test_memory.data[0x0000], 0x01);
    }

    #[test]
    fn test_memory_protected() {
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_memory.write(0x0000, 0xEF);

        assert_eq!(test_memory.read(0x0000, 1), 0xEF);

        test_memory.push_write_protected_region(WriteProtectedRegion {
            protected_memory: MemoryRegion {
                region_address: 0x0000,
                region_size: 1
            }
        });

        test_memory.write(0x0000, 0xBE);
        assert_eq!(test_memory.read(0x0000, 1), 0xEF);
    }
}
