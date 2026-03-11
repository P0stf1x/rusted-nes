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
    nametable_address: usize,
    ppu_addr_high_byte: bool,
    ppu_addr: usize,
    x_offset: usize,
    y_offset: usize,
    controller_state: u8,
    oam_data: [u8; 256],
    oam_addr: usize,
    fg_plane: bool,
    rendered_nametable: Vec<u32>
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
            nametable_address: 0x2000,
            ppu_addr_high_byte: true,
            ppu_addr: 0x0000,
            x_offset: 0,
            y_offset: 0,
            controller_state: 0,
            oam_data: [0; 256],
            oam_addr: 0,
            fg_plane: false,
            rendered_nametable: vec![0u32; 32*30*4],
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
                self.render_frame();
                if self.pattern_table_window.is_open() { self.render_pattern_table(); } // If pattern table is open - we also render it
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
