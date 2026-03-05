use std::{ sync::mpsc::{channel, Receiver, Sender}, time::Instant };

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
    vram_address: usize,
    w: bool,
    cpu_pointer: CPUPtrWrapper,
    dot: f64,
    is_closed: bool,
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
            vram_address: 0,
            w: false,
            cpu_pointer,
            dot: 0.,
            is_closed: false,
        },
        tx)
    }

    fn get_line_dot(&self) -> (usize, usize) {
        return (self.dot.div_euclid(341.) as usize, self.dot.rem_euclid(341.) as usize);
    }

    pub fn tick(&mut self) {
        if !self.is_closed {
            if self.main_window.is_key_down(Key::Escape) { self.is_closed = true; return; };

            self.process_memory_events();

            if self.dot > 89341.5 {
                self.dot -= 89341.5;
                self.render_frame();
                if self.pattern_table_window.is_open() { self.render_pattern_table(); } // If pattern table is open - we also render it
            }

            if self.get_line_dot() == (241, 0) {
                self.set_vblank();
                self.call_nmi();
            }

            if self.get_line_dot() == (262, 0) {
                self.clear_vblank();
            }

            self.dot += 1.;
        }
    }

    fn call_nmi(&self) {
        unsafe{(*self.cpu_pointer.0).nmi((&mut *self.memory_pointer.0));};
    }

    fn set_vblank(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] |= 0b_1000_0000};
    }

    fn clear_vblank(&self) {
        unsafe{(&mut *self.memory_pointer.0).data[0x2002] &= 0b_0111_1111};
    }
}
