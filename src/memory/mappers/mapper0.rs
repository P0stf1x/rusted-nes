use crate::{ ines::iNESData, memory::ppu_memory::PPU_MEM, MemoryMirror, MemoryRegion, MEM };

use super::check_if_correct_mapper;

pub fn map(data: iNESData) -> (MEM, PPU_MEM) {
    check_if_correct_mapper(0, data.header.mapper_number);

    let mut memory = MEM::new(0x10000);
    let mut ppu_memory = MEM::new(0x4000);
    match data.header.prg_rom_size {
        1 => {
            memory.write_bulk(0x8000, data.prg_rom[0x0000..0x3FFF].to_vec());
            memory.push_mirrored_range(
                MemoryMirror {
                    physical_memory: MemoryRegion {
                        region_address: 0x8000,
                        region_size: 0x4000,
                    },
                    mirrored_memory: MemoryRegion {
                        region_address: 0xC000,
                        region_size: 0x4000
                    }
                }
            ).unwrap();
        },
        2 => memory.write_bulk(0x8000, data.prg_rom[0x0000..0x7FFF].to_vec()),
        _ => panic!("Used mapper 0 but PRG ROM size is not [1-2]")
    }
    if data.chr_rom.len() != 0x2000 {panic!("chr_rom length is not 0x2000")};
    ppu_memory.write_bulk(0x0000, data.chr_rom);
    ppu_memory.push_write_protected_region(
        crate::WriteProtectedRegion {
            protected_memory: MemoryRegion {
                region_address: 0x0000,
                region_size: 0x8000
            }
        }
    );
    return (memory, ppu_memory);
}
