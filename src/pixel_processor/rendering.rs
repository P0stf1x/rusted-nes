use minifb::{ Window, WindowOptions };

use super::{ helper::overlay_sprite, tile::Tile, PPU };

impl PPU {
    pub(super) fn render_frame(&mut self) {
        // For now there's only minifb rendering
        // TODO: implement ImGUI rendering
        self.main_window
            .update_with_buffer(&[0; 256*240], 256, 240)
            .unwrap();
    }

    pub(super) fn render_pattern_table(&mut self) {
        let mut pattern_screen = [0u32; 256*128];
        for bit_plane in 0..=1 {
            for y in 0..16 {
                for x in 0..16 {
                    let sprite = &Tile::get(&self.ppu_memory, x+y*8, bit_plane!=0).rendered();
                    overlay_sprite(&mut pattern_screen, sprite, x*8+bit_plane*128, y*8, 256, 8);
                }
            }
        }
        self.pattern_table_window.update_with_buffer(&pattern_screen, 256, 128).unwrap();
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
