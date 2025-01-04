use std::time::Instant;

use minifb::{ Window, WindowOptions, Key };

use crate::memory::*;

#[derive(Clone, Copy)]
pub struct MemPtrWrapper(pub *mut MEM);
unsafe impl Sync for MemPtrWrapper {}
unsafe impl Send for MemPtrWrapper {}

pub struct PPU {
    memory_pointer: MemPtrWrapper,
    framebuffer: Vec<u32>,
    window: Window,
    // should_close: bool,
}

impl PPU {
    pub fn new(memory_pointer: MemPtrWrapper) -> Self {
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
        let window = Window::new(
            "Rusted NES",
            256,
            240,
            window_options
        ).unwrap();
        return Self {
            memory_pointer,
            framebuffer: vec![0; 256*240],
            window,
        }
    }

    fn render_frame(&mut self) {
        // For now there's only minifb rendering
        // TODO: implement ImGUI rendering
        self.window
            .update_with_buffer(&[0; 256*240], 256, 240)
            .unwrap();
    }

    pub fn run(&mut self) {
        let mut fps_counter = Instant::now();
        let mut frame_counter = 0;
        const PIXEL:f64 = 186.2433862;
        const SCANLINE:f64 = 341.*PIXEL;
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) { // Frame start
            let frame_start = Instant::now();
            // TODO: process user input
            self.render_frame(); // We render frame at 0:0
            while frame_start.elapsed().as_nanos() < (241.*SCANLINE) as u128 {} // For 0:0 through 240:341 we wait till VBlank
            self.set_vblank(); // We set VBlank at 241:0
            while frame_start.elapsed().as_nanos() < (262.*SCANLINE) as u128 {} // For 241:0 through 261:341 we wait till VBlank
            self.clear_vblank(); // We clear VBlank at 262:0
            frame_counter += 1;
            if fps_counter.elapsed().as_millis()>=10000 {
                println!("frames per second {frame_counter}");
                println!("average frametime {:#}", fps_counter.elapsed().as_nanos()/frame_counter);
                // TODO: Since this would always be longer than exactly needed time we can calculate frametime deviation
                // and adjust to it in runtime. This would help in emulating games that depend on strict timings
                frame_counter = 0;
                fps_counter = Instant::now();
            }
        }
    }

    fn set_vblank(&self) {
        unsafe{(*self.memory_pointer.0).data[0x2002] |= 0b_1000_0000};
    }

    fn clear_vblank(&self) {
        unsafe{(*self.memory_pointer.0).data[0x2002] &= 0b_0111_1111};
    }
}