use crate::{ines::iNESData, MemoryMirror, MemoryRegion, MEM};

use super::check_if_correct_mapper;

pub fn map(data: iNESData) -> MEM {
    check_if_correct_mapper(0, data.header.mapper_number);

    let mut memory = MEM::new(0xFFFF);
    match data.header.prg_rom_size {
        1 => {
            memory.write_bulk(0x8000, data.prg_rom[0x0000..0x3FFF].to_vec());
            _ = memory.push_mirrored_range(
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
            );
        },
        2 => memory.write_bulk(0x8000, data.prg_rom[0x0000..0x7FFF].to_vec()),
        _ => panic!("Used mapper 0 but PRG ROM size is not [1-2]")
    }
    // TODO: add ram (or rather write protection to non ram memory)
    return memory;
}