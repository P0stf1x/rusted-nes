use minifb::{ Window, WindowOptions };

use crate::pixel_processor::tile::PixelPalette;
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
            let oam_palette_id = 4 + (self.oam_data[oam_sprite_id*4+2] & 0b_0000_0011) as usize;
            let palette = PixelPalette::get_by_id(&self.ppu_memory, oam_palette_id);
            let tile = Tile::get(&self.ppu_memory, oam_tile_id as usize, self.fg_plane);
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

    pub(super) fn render_bg_slice(&mut self, line: usize, slice: usize) {
        for i in 0..8 {
            let x = slice * 8 + i;
            let (base_addr, tile_offset) = get_actual_nametable_addr_and_tile_offset(x + self.x_offset, line + self.y_offset, self.nametable_address);
            let tile_pattern_id = self.ppu_memory.read(base_addr + tile_offset, 1);
            let pixel_index = Tile::get_at(&self.ppu_memory, (x + self.x_offset) & 0b_0000_0111, (line + self.y_offset) & 0b_0000_0111, tile_pattern_id, self.bg_plane);
            let color_palette = PixelPalette::get_from_addr_and_offset(&self.ppu_memory, base_addr, tile_offset);

            self.main_framebuffer[x + line*256] = match pixel_index {
                tile::PixelPaletteColorIndex::Background => color_palette.background,
                tile::PixelPaletteColorIndex::Color1 => color_palette.color1,
                tile::PixelPaletteColorIndex::Color2 => color_palette.color2,
                tile::PixelPaletteColorIndex::Color3 => color_palette.color3,
            };
        }
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
