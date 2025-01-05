pub fn overlay_sprite(screen: &mut [u32], sprite: &[u32], offset_x: usize, offset_y: usize, screen_width: usize, sprite_width: usize) {
    for y in 0..sprite.len()/sprite_width {
        for x in 0..sprite_width {
            let screen_offset = (offset_y+y)*screen_width + (offset_x+x);
            let sprite_offset = y*sprite_width + x;
            screen[screen_offset] = sprite[sprite_offset];
        }
    }
}
