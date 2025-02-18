use std::num::Wrapping;

use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_cmp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_cmp_imm(memory),
            MemoryMode::ZeroPage  => self.execute_cmp_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_cmp_zpgx(memory),
            MemoryMode::Absolute  => self.execute_cmp_abs(memory),
            MemoryMode::AbsoluteX => self.execute_cmp_absx(memory),
            MemoryMode::AbsoluteY => self.execute_cmp_absy(memory),
            MemoryMode::IndirectX => self.execute_cmp_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_cmp_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for CMP", mode)
        }
    }
}

macro_rules! cmp {
    ($cpu:ident, $instruction:ident) => {{
        $cpu.C = $cpu.get_a() >= $instruction.value.unwrap();
        $cpu.Z = $cpu.get_a() == $instruction.value.unwrap();
        $cpu.N = (Wrapping::<u8>($cpu.get_a()) - Wrapping::<u8>($instruction.value.unwrap())).0 & 0b_1000_0000 != 0;
    }}
}

impl CPU {
    fn execute_cmp_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cmp_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cmp_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cmp_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(3);
    }

    fn execute_cmp_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(3);
    }

    fn execute_cmp_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(3);
    }

    fn execute_cmp_indirect_x(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_x(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(2);
    }

    fn execute_cmp_indirect_y(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_y(&self, memory);
        inst.log(&self, "CMP");
        cmp!(self, inst);
        self.increment_pc(2);
    }
}
