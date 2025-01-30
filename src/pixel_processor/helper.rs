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
