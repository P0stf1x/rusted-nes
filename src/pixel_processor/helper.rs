// TODO: I'm pretty sure this function is broken. Needs more testing
// TODO: Right now it doesn't support transparency, so it doesn't work with sprites, only with tiles
pub fn overlay_sprite(screen: &mut [u32], sprite: &[u32], offset_x: usize, offset_y: usize, screen_width: usize) {
    for y in 0..8 {     // assume sprite is always 8x8
        for x in 0..8 {
            let screen_offset = (offset_y+y)*screen_width + (offset_x+x);
            let sprite_offset = y*8 + x;
            screen[screen_offset] = sprite[sprite_offset];
        }
    }
}

pub fn get_actual_nametable_addr_and_tile_offset(x: usize, y: usize, nametable_address: usize) -> (usize, usize) {
    let mut tile_base_offset = nametable_address;
    let horizontal_offset = (x / (32*8)).rem_euclid(2) * 0x400;
    let vertical_offset = (y / (30*8)).rem_euclid(2) * 0x800;

    if nametable_address == 0x2000 || nametable_address == 0x2800 {
        tile_base_offset += horizontal_offset; // left half
    } else {
        tile_base_offset -= horizontal_offset; // right half
    }

    if nametable_address == 0x2000 || nametable_address == 0x2400 {
        tile_base_offset += vertical_offset; // upper half
    } else {
        tile_base_offset -= vertical_offset; // lower half
    }

    let tile_x = (x/8).rem_euclid(32);
    let tile_y = (y/8).rem_euclid(30);
    return (tile_base_offset, tile_x + tile_y*32);
}

pub fn reverse_bits(value: u8) -> u8 {
    #[cfg(target_arch = "aarch64")]
    {
        let mut result: u8 = 0;
        unsafe {
            std::arch::asm!(
                "rbit {res:x}, {val:x}",
                "lsr {res:x}, {res:x}, #56",
                options(nostack),
                val = in(reg) value,
                res = lateout(reg) result,
            );
        };
        return result;
    }

    #[cfg(not(target_arch = "aarch64"))]
    {
        ((((value as u64) * 0x0202020202) & 0x010884422010) % 1023) as u8
    }
}
