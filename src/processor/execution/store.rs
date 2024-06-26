#![allow(unreachable_code)]
#![allow(unused_variables)]
use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_sta(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_sta_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sta_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sta_abs(memory),
            MemoryMode::AbsoluteX => self.execute_sta_absx(memory),
            MemoryMode::AbsoluteY => self.execute_sta_absy(memory),
            MemoryMode::IndirectX => self.execute_sta_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_sta_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for STA", mode)
        }
    }

    pub fn execute_stx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_stx_zpg(memory),
            MemoryMode::ZeroPageY => self.execute_stx_zpgy(memory),
            MemoryMode::Absolute  => self.execute_stx_abs(memory),
            _                     => panic!("No {:?} memory mode for STX", mode)
        }
    }

    pub fn execute_sty(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_sty_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sty_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sty_abs(memory),
            _                     => panic!("No {:?} memory mode for STY", mode)
        }
    }
}

// STA IMPL
impl CPU {
    fn execute_sta_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        memory.data[memory_address] = self.A.value as u8;
        self.PC += 2;
    }

    fn execute_sta_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        memory.data[memory_address.0 as usize] = self.A.value as u8;
        self.PC += 2;
    }

    fn execute_sta_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        memory.data[memory_address] = self.A.value as u8;
        self.PC += 3;
    }

    fn execute_sta_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u8);
        memory_address += self.X.value as u8;
        memory.data[memory_address.0 as usize] = self.A.value as u8;
        self.PC += 3;
    }

    fn execute_sta_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u8);
        memory_address += self.Y.value as u8;
        memory.data[memory_address.0 as usize] = self.A.value as u8;
        self.PC += 3;
    }

    fn execute_sta_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        memory.data[memory_address.0 as usize] = self.A.value as u8;
        self.PC += 2;
    }

    fn execute_sta_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read((self.PC + 1).value as usize, 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.value as u16;
        memory.data[memory_address.0 as usize] = self.A.value as u8;
        self.PC += 2;
    }
}

// STX IMPL
impl CPU {
    fn execute_stx_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        memory.data[memory_address] = self.X.value as u8;
        self.PC += 2;
    }

    fn execute_stx_zpgy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.Y.value as u8;
        memory.data[memory_address.0 as usize] = self.X.value as u8;
        self.PC += 2;
    }

    fn execute_stx_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        memory.data[memory_address] = self.X.value as u8;
        self.PC += 3;
    }
}

// STY IMPL
impl CPU {
    fn execute_sty_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        memory.data[memory_address] = self.Y.value as u8;
        self.PC += 2;
    }

    fn execute_sty_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        memory.data[memory_address.0 as usize] = self.Y.value as u8;
        self.PC += 2;
    }

    fn execute_sty_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        memory.data[memory_address] = self.Y.value as u8;
        self.PC += 3;
    }
}
