use crate::{ MEM, MemoryMirror, MemoryRegion, ines::iNESData, memory::{ines::NametableLayout, ppu_memory::PPU_MEM} };

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
    match data.header.nametable_layout {
        NametableLayout::horizontal => {
            ppu_memory.push_mirrored_range(MemoryMirror {
                physical_memory: MemoryRegion { region_address: 0x2000, region_size: 0x0800 },
                mirrored_memory: MemoryRegion { region_address: 0x2800, region_size: 0x0800 },
            }).unwrap();
        },
        NametableLayout::vertical_or_mapper => {
            ppu_memory.push_mirrored_range(MemoryMirror {
                physical_memory: MemoryRegion { region_address: 0x2000, region_size: 0x0400 },
                mirrored_memory: MemoryRegion { region_address: 0x2400, region_size: 0x0400 },
            }).unwrap();
            ppu_memory.push_mirrored_range(MemoryMirror {
                physical_memory: MemoryRegion { region_address: 0x2800, region_size: 0x0400 },
                mirrored_memory: MemoryRegion { region_address: 0x2C00, region_size: 0x0400 },
            }).unwrap();
        }
    }
    return (memory, ppu_memory);
}
