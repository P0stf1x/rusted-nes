use std::num::Wrapping;

use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_cpy(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_cpy_imm(memory),
            MemoryMode::ZeroPage  => self.execute_cpy_zpg(memory),
            MemoryMode::Absolute  => self.execute_cpy_abs(memory),
            _                     => panic!("No {:?} memory mode for CPY", mode)
        }
    }
}

macro_rules! cpy {
    ($cpu:ident, $instruction:ident) => {{
        $cpu.C = $cpu.get_y() >= $instruction.value.unwrap();
        $cpu.Z = $cpu.get_y() == $instruction.value.unwrap();
        $cpu.N = (Wrapping::<u8>($cpu.get_y()) - Wrapping::<u8>($instruction.value.unwrap())).0 & 0b_1000_0000 != 0;
    }}
}

impl CPU {
    fn execute_cpy_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "CPY");
        cpy!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cpy_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "CPY");
        cpy!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cpy_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "CPY");
        cpy!(self, inst);
        self.increment_pc(3);
    }
}
