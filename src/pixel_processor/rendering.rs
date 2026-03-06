use minifb::{ Window, WindowOptions };

use crate::{memory::ppu_memory::PPU_MEM, pixel_processor::tile::PixelPalette};

use super::{ helper::overlay_sprite, tile::{self, Tile}, PPU };

fn render_oam(screen: &mut [u32], oam_data: &[u8], screen_width: usize, ppu_memory: &PPU_MEM, plane: bool) {
    for oam_sprite_id in 0..64 {
        let oam_sprite_x = oam_data[oam_sprite_id*4+3] as usize;
        let oam_sprite_y = oam_data[oam_sprite_id*4+0] as usize;
        // let oam_tile_plane = oam_data[oam_sprite_id*4+1] & 0x01 == 0; // only in 8x16
        // let oam_tile_id = oam_data[oam_sprite_id*4+1] >> 1;
        let oam_tile_id = oam_data[oam_sprite_id*4+1];
        let oam_palette_id = 4 + (oam_data[oam_sprite_id*4+2] & 0b_0000_0011) as usize;
        let palette = PixelPalette::get_by_id(ppu_memory, oam_palette_id);
        let tile = Tile::get(ppu_memory, oam_tile_id as usize, plane);
        let sprite = tile.rendered(palette);
        for y in 0..8 {     // assume sprite is always 8x8
            for x in 0..8 {
                let screen_offset = 239.min(oam_sprite_y+y)*screen_width + 255.min(oam_sprite_x+x);
                let sprite_offset = y*8 + x;
                screen[screen_offset] = sprite[sprite_offset];
            }
        }
    }
}

impl PPU {
    pub(super) fn render_frame(&mut self) {
        // For now there's only minifb rendering
        // TODO: implement ImGUI rendering
        for i in 0..32*30 { // Each byte is 8x8 sprite index so in turn we fill 256x240 pixels
            // TODO: It renders only first plane right now
            let (tile, tile_palette) = tile::get_tile_and_palette(&self.ppu_memory, i, self.pattern_table_bit_plane, self.x_offset, self.y_offset);
            overlay_sprite(&mut self.main_framebuffer, &tile.rendered(tile_palette), (i%32)*8, (i/32)*8, 256);
            render_oam(&mut self.main_framebuffer, &self.oam_data, 256, &self.ppu_memory, self.fg_plane);
        }
        self.main_window
            .update_with_buffer(&self.main_framebuffer, 256, 240)
            .unwrap();
    }

    pub(super) fn render_pattern_table(&mut self) {
        for bit_plane in 0..=1 {
            for y in 0..16 {
                for x in 0..16 {
                    let tile_palette = tile::PixelPalette::get_sample_palette();
                    let tile = &Tile::get(&self.ppu_memory, x+y*8, bit_plane!=0).rendered(tile_palette);
                    overlay_sprite(&mut self.pattern_table_framebuffer, tile, x*8+bit_plane*128, y*8, 256);
                }
            }
        }
        self.pattern_table_window.update_with_buffer(&self.pattern_table_framebuffer, 256, 128).unwrap();
    }

    pub(super) fn create_main_window() -> Window {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false, // crash on macos
            none: false, //?
        };
        let main_window = Window::new(
            "Rusted NES",
            256,
            240,
            window_options
        ).unwrap();
        return main_window;
    }

    pub(super) fn create_pattern_window() -> Window {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false, // crash on macos
            none: false, //?
        };
        let pattern_table_window = Window::new(
            "Pattern table",
            256,
            128,
            window_options
        ).unwrap();
        return pattern_table_window;
    }
}
