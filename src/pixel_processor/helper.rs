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

pub fn get_tile_and_palette_addr(x: usize, y: usize, nametable_address: usize) -> (usize, usize) {
    let mut tile_base_offset = nametable_address;
    match nametable_address {
        0x2000 => {
            if x >= 32 { tile_base_offset += 0x400 };
            if y >= 30 { tile_base_offset += 0x800 };
        },
        0x2400 => {
            if x >= 32 { tile_base_offset -= 0x400 };
            if y >= 30 { tile_base_offset += 0x800 };
        },
        0x2800 => {
            if x >= 32 { tile_base_offset += 0x400 };
            if y >= 30 { tile_base_offset -= 0x800 };
        },
        0x2C00 => {
            if x >= 32 { tile_base_offset -= 0x400 };
            if y >= 30 { tile_base_offset -= 0x800 };
        },
        addr => panic!("Nametable address is not valid: 0x{:04X}", addr),
    }
    let palette_base_offset = tile_base_offset + 0x3C0;
    let local_x = x.rem_euclid(32);
    let local_y = y.rem_euclid(30);
    return (tile_base_offset + local_x + local_y*32, palette_base_offset);
}
