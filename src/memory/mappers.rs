use super::{ ines::iNESData, ppu_memory::PPU_MEM, MemoryMirror, MemoryRegion, MEM };

pub mod mapper0;

pub fn map(input: iNESData) -> (MEM, PPU_MEM) {
    add_write_protection_and_mirroring(
        match input.header.mapper_number {
            0 => mapper0::map(input),
            _ => unimplemented!() // TODO: implement other mappers
        }
    )
}

fn add_write_protection_and_mirroring((mut memory, mut ppu_memory): (MEM, PPU_MEM)) -> (MEM, PPU_MEM) {
    // PPU registers mirroring
    memory.push_mirrored_range(MemoryMirror {
        physical_memory: MemoryRegion { region_address: 0x2000, region_size: 0x0008 },
        mirrored_memory: MemoryRegion { region_address: 0x2008, region_size: 0x1FF8 }
    }).unwrap();

    // 2KB ram mirroring
    memory.push_mirrored_range(MemoryMirror {
        physical_memory: MemoryRegion { region_address: 0x0000, region_size: 0x0800 },
        mirrored_memory: MemoryRegion { region_address: 0x0800, region_size: 0x0800 }
    }).unwrap();
    
    // PPU palette ram indexes
    ppu_memory.push_mirrored_range(MemoryMirror {
        physical_memory: MemoryRegion { region_address: 0x3F00, region_size: 0x0020 },
        mirrored_memory: MemoryRegion { region_address: 0x3F20, region_size: 0x00E0 }
    }).unwrap();
    return (memory, ppu_memory);
}

fn check_if_correct_mapper(used_mapper: u16, expected_mapper: u16) {
    if used_mapper != expected_mapper {
        panic!("Wrong mapper used. Used mapper {used_mapper}, expected mapper {expected_mapper}")
    }
}
