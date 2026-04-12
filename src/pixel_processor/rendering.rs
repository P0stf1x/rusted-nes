use std::time::Instant;

use minifb::{ Window, WindowOptions };

use crate::pixel_processor::tile::{PixelPalette, PixelPaletteColorIndex};
use crate::pixel_processor::helper::get_actual_nametable_addr_and_tile_offset;

use super::{ helper::overlay_sprite, tile::{self, Tile}, PPU };

impl PPU {
    pub(super) fn display_frame(&mut self) {
        // For now there's only minifb rendering
        // TODO: implement ImGUI rendering
        self.main_window
            .update_with_buffer(&self.main_framebuffer, 256, 240)
            .unwrap();
    }

    pub(super) fn render_oam(&mut self) {
        for oam_sprite_id in 0..64 {
            let oam_sprite_x = self.oam_data[oam_sprite_id*4+3] as usize;
            let oam_sprite_y = self.oam_data[oam_sprite_id*4+0] as usize;
            // let oam_tile_plane = oam_data[oam_sprite_id*4+1] & 0x01 == 0; // only in 8x16
            // let oam_tile_id = oam_data[oam_sprite_id*4+1] >> 1;
            let oam_tile_id = self.oam_data[oam_sprite_id*4+1];
            let reverse_h = self.oam_data[oam_sprite_id*4+2] & 0b_0100_0000 != 0;
            let reverse_v = self.oam_data[oam_sprite_id*4+2] & 0b_1000_0000 != 0;
            let oam_palette_id = 4 + (self.oam_data[oam_sprite_id*4+2] & 0b_0000_0011) as usize;
            let palette = PixelPalette::get_by_id(&self.ppu_memory, oam_palette_id);
            let tile = Tile::get(&self.ppu_memory, oam_tile_id as usize, self.fg_plane, reverse_h, reverse_v);
            let sprite = tile.rendered(palette);
            for y in 0..8 {     // assume sprite is always 8x8
                for x in 0..8 {
                    let screen_offset = 239.min(oam_sprite_y+y)*256 + 255.min(oam_sprite_x+x);
                    let sprite_offset = y*8 + x;
                    self.main_framebuffer[screen_offset] = sprite[sprite_offset];
                }
            }
        }
    }

    pub(super) fn render_current_vram_slice(&mut self) {
        let slice = self.vram_v.get_coarse_x() as usize;
        let starting_slice = self.vram_t.get_coarse_x() as usize;
        let current_line = ((self.vram_v.get_coarse_y() << 3) + self.vram_v.get_fine_y()) as usize;
        let starting_line = ((self.vram_t.get_coarse_y() << 3) + self.vram_t.get_fine_y()) as usize;
        let line = current_line - starting_line;
        for i in 0..8 {
            let x = (slice - starting_slice) * 8 + i;
            let (pixel_index, color_palette) = self.get_bg_pixel_at(i, line);

            self.main_framebuffer[x + line*256] = match pixel_index {
                PixelPaletteColorIndex::Background => color_palette.background,
                PixelPaletteColorIndex::Color1 => color_palette.color1,
                PixelPaletteColorIndex::Color2 => color_palette.color2,
                PixelPaletteColorIndex::Color3 => color_palette.color3,
            };
        }
    }

    pub(super) fn get_bg_pixel_at(&self, fine_x: usize, fine_y: usize) -> (PixelPaletteColorIndex, PixelPalette) {
        let fine_dot = fine_x & 0b_0000_0111;
        let fine_line = fine_y & 0b_0000_0111;

        let mut incremented_vram = self.vram_v;
        let should_fetch_next = (self.fine_x as usize) + fine_dot > 7;
        if should_fetch_next { // because coarse x and nametable x are noncontiguous
            incremented_vram.increment_x();
        }

        let tile_address = 0x2000 + ((incremented_vram.get_all() as usize) & 0x0FFF);
        let tile_pattern_id = self.ppu_memory.read(tile_address, 1);
        let pixel_index = Tile::get_at(&self.ppu_memory, (fine_dot + (self.fine_x as usize)) & 0b_0000_0111, fine_line, tile_pattern_id, self.bg_plane, false, false);

        let palette_base = tile_address & 0b_1111_11_00000_00000;
        let palette_offset = tile_address & 0b_0000_00_11111_11111;
        let color_palette = PixelPalette::get_from_addr_and_offset(&self.ppu_memory, palette_base, palette_offset);

        return (pixel_index, color_palette);
    }

    pub(super) fn render_pattern_table(&mut self) {
        for bit_plane in 0..=1 {
            for y in 0..16 {
                for x in 0..16 {
                    let tile_palette = tile::PixelPalette::get_sample_palette();
                    let tile = &Tile::get(&self.ppu_memory, x+y*8, bit_plane!=0, false, false).rendered(tile_palette);
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

    pub(super) fn wait_for_next_frame(&mut self) {
        loop { // calling sleep() is not guaranteed to sleep exactly specified time, only AT LEAST specified time or more
            if self.frame_start.elapsed().as_nanos() > 16_666_666 {
                self.frame_start = Instant::now();
                break;
            }
        }
    }
}
