use std::{ process::exit, sync::mpsc::{Receiver, Sender, channel}, time::Instant };

use minifb::{ Window, Key };

use crate::memory::*;
use ppu_memory::PPU_MEM;

pub mod tile;
pub mod rendering;
pub mod helper;
mod memory_events_processor;

#[derive(Clone, Copy)]
pub struct MemPtrWrapper(pub *mut MEM);
unsafe impl Sync for MemPtrWrapper {}
unsafe impl Send for MemPtrWrapper {}
#[derive(Clone, Copy)]
pub struct CPUPtrWrapper(pub *mut crate::CPU);
unsafe impl Sync for CPUPtrWrapper {}
unsafe impl Send for CPUPtrWrapper {}

#[derive(Clone, Copy)]
pub struct PPUVramAddr(u16);

impl PPUVramAddr {
    pub fn set_all(&mut self, value: u16) {
        self.0 = value & 0b0111_1111_1111_1111;
    }

    pub fn get_all(&self) -> u16 {
        return self.0;
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.0 &= !0x1F;
        self.0 += (value & 0x1F) as u16;
    }

    pub fn get_coarse_x(&self) -> u8 {
        return (self.0 & 0x1F) as u8;
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.0 &= !(0x1F << 5);
        self.0 += ((value & 0x1F) as u16) << 5;
    }

    pub fn get_coarse_y(&self) -> u8 {
        return ((self.0 >> 5) & 0x1F) as u8;
    }

    pub fn set_nametable_h(&mut self, value: bool) {
        if value {
            self.0 |= 0b_0000_0100_0000_0000;
        } else {
            self.0 &= 0b_1111_1011_1111_1111;
        }
    }

    pub fn get_nametable_h(&self) -> bool {
        return self.0 & 0b_0000_0100_0000_0000 != 0;
    }

    pub fn set_nametable_v(&mut self, value: bool) {
        if value {
            self.0 |= 0b_0000_1000_0000_0000;
        } else {
            self.0 &= 0b_1111_0111_1111_1111;
        }
    }

    pub fn get_nametable_v(&self) -> bool {
        return self.0 & 0b_0000_1000_0000_0000 != 0;
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.0 &= !(0x7 << 12);
        self.0 += ((value & 0x7) as u16) << 12;
    }

    pub fn get_fine_y(&self) -> u8 {
        return ((self.0 >> 12) & 0x7) as u8;
    }

    pub fn increment_x(&mut self) {
        self.set_coarse_x(self.get_coarse_x() + 1);
        if self.get_coarse_x() == 0 {
            self.set_nametable_h(!self.get_nametable_h());
        }
    }

    pub fn increment_y(&mut self) {
        self.set_fine_y(self.get_fine_y() + 1);
        if self.get_fine_y() == 0 {
            let mut y = self.get_coarse_y();
            if y == 29 {
                y = 0;
                self.set_nametable_v(!self.get_nametable_v());
            } else if y == 31 {
                y = 0;
            } else {
                y += 1
            }
            self.set_coarse_y(y);
        }
    }
}

pub struct PPU {
    memory_pointer: MemPtrWrapper,
    #[allow(unused)]
    framebuffer: Vec<u32>,
    main_window: Window,
    pattern_table_window: Window,
    ppu_memory: PPU_MEM,
    memory_events_rx: Receiver<MemoryEvent>,
    cpu_pointer: CPUPtrWrapper,
    dot: f64,
    is_closed: bool,
    main_framebuffer: Vec<u32>,
    pattern_table_framebuffer: Vec<u32>,
    nmi_enabled: bool,
    bg_plane: bool,
    ppudata_write_down: bool,
    ppu_addr_high_byte: bool,
    vram_v: PPUVramAddr,
    vram_t: PPUVramAddr,
    fine_x: u8,
    controller_state: u8,
    oam_data: [u8; 256],
    oam_addr: usize,
    fg_plane: bool,
    frame_start: Instant,
    fg_rendering: bool,
    bg_rendering: bool,
}

impl PPU {
    pub fn new(memory_pointer: MemPtrWrapper, ppu_memory: PPU_MEM, cpu_pointer: CPUPtrWrapper) -> (Self, Sender<MemoryEvent>) {
        let (tx, memory_events_rx): (Sender<MemoryEvent>, Receiver<MemoryEvent>) = channel();
        return (Self {
            memory_pointer,
            framebuffer: vec![0; 256*240],
            main_window: Self::create_main_window(),
            pattern_table_window: Self::create_pattern_window(),
            ppu_memory,
            memory_events_rx,
            cpu_pointer,
            dot: 0.,
            is_closed: false,
            main_framebuffer: vec![0; 256*240],
            pattern_table_framebuffer: vec![0; 256*128],
            nmi_enabled: true,
            bg_plane: false,
            ppudata_write_down: false,
            ppu_addr_high_byte: true,
            vram_v: PPUVramAddr(0),
            vram_t: PPUVramAddr(0),
            fine_x: 0,
            controller_state: 0,
            oam_data: [0; 256],
            oam_addr: 0,
            fg_plane: false,
            frame_start: Instant::now(),
            fg_rendering: false,
            bg_rendering: false,
        },
        tx)
    }

    fn get_line_dot(&self) -> (usize, usize) {
        return (self.dot.div_euclid(341.) as usize, self.dot.rem_euclid(341.) as usize);
    }

    pub fn tick(&mut self) {
        if !self.is_closed {
            if self.main_window.is_key_down(Key::Escape) { self.is_closed = true; exit(0); };
            if self.pattern_table_window.is_key_down(Key::Escape) { self.is_closed = true; exit(0); };

            self.process_memory_events();

            if self.dot > 89341.5 {
                self.dot -= 89341.5;
                if self.fg_rendering {
                    self.render_oam();
                }
                self.display_frame();
                self.wait_for_next_frame();
                if self.pattern_table_window.is_open() { self.render_pattern_table(); } // If pattern table is open - we also render it
            }

            if self.bg_rendering {
                let (line, dot) = self.get_line_dot();
                if (line < 240 || line == 261) && 0 < dot && dot <= 256 {
                    if dot.rem_euclid(8) == 0 /* last dot of 8 long slice starting from 1 */ {
                            if line != 261 { // line -1/261 is not used
                                self.render_current_vram_slice();
                            }
                            self.vram_v.increment_x();
                            if dot == 256 {
                                self.vram_v.increment_y();
                            }
                    }
                }
            }

            if self.bg_rendering || self.fg_rendering {
                match self.get_line_dot() {
                    (0..240 | 261, 257) => {
                        // Copying horizontal vram_t to vram_v
                        self.vram_v.set_coarse_x(self.vram_t.get_coarse_x());
                        self.vram_v.set_nametable_h(self.vram_t.get_nametable_h());
                    },
                    _ => (),
                }
            }

            if self.bg_rendering || self.fg_rendering {
                match self.get_line_dot() {
                    (261, 280..=304) => {
                        // Copying vertical vram_t to vram_v
                        self.vram_v.set_coarse_y(self.vram_t.get_coarse_y());
                        self.vram_v.set_nametable_v(self.vram_t.get_nametable_v());
                        self.vram_v.set_fine_y(self.vram_t.get_fine_y());
                    }
                    _ => (),
                }
            }

            if self.get_line_dot() == (241, 1) {
                self.set_vblank();
                if self.nmi_enabled {
                    self.call_nmi();
                }
            }

            if self.get_line_dot() == (261, 1) {
                self.clear_vblank();
                self.clear_sprite_0_hit();
                self.clear_sprite_overflow();
            }

            self.dot += 1.;
        }
    }

    fn call_nmi(&self) {
        unsafe{(*self.cpu_pointer.0).nmi(&mut *self.memory_pointer.0);};
    }

    fn set_vblank(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] |= 0b_1000_0000};
    }

    fn clear_vblank(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] &= 0b_0111_1111};
    }

    fn set_sprite_0_hit(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] |= 0b_0100_0000};
    }

    fn clear_sprite_0_hit(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] &= 0b_1011_1111};
    }

    fn set_sprite_overflow(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] |= 0b_0010_0000};
    }

    fn clear_sprite_overflow(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] &= 0b_1101_1111};
    }

    fn get_controller_state(&self) -> u8 {
        let mut value = 0x00;
        if self.main_window.is_key_down(Key::Right) { value += 0b_1000_0000 } // DPAD
        if self.main_window.is_key_down(Key::Left) { value += 0b_0100_0000 }
        if self.main_window.is_key_down(Key::Down) { value += 0b_0010_0000 }
        if self.main_window.is_key_down(Key::Up) { value += 0b_0001_0000 }

        if self.main_window.is_key_down(Key::V) { value += 0b_0000_1000 } // Start
        if self.main_window.is_key_down(Key::C) { value += 0b_0000_0100 } // Select
        if self.main_window.is_key_down(Key::X) { value += 0b_0000_0010 } // B
        if self.main_window.is_key_down(Key::Z) { value += 0b_0000_0001 } // A
        return value;
    }
}
