use crate::logging::Logger;
use crate::processor::instruction::Instruction;
use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_lda(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_lda_imm(memory),
            MemoryMode::ZeroPage  => self.execute_lda_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_lda_zpgx(memory),
            MemoryMode::Absolute  => self.execute_lda_abs(memory),
            MemoryMode::AbsoluteX => self.execute_lda_absx(memory),
            MemoryMode::AbsoluteY => self.execute_lda_absy(memory),
            MemoryMode::IndirectX => self.execute_lda_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_lda_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for LDA", mode)
        }
    }
}

macro_rules! lda {
    ($cpu:ident, $instruction:ident) => {{
        let value = $instruction.value.unwrap();
        $cpu.store_a(value);
        $cpu.Z = $cpu.get_a() == 0;
        $cpu.N = $cpu.get_a() & 0b_1000_0000 != 0;
    }}
}

// LDA IMPL
impl CPU {
    fn execute_lda_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(2);
    }

    fn execute_lda_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(2);
    }
    
    fn execute_lda_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(2);
    }

    fn execute_lda_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(3);
    }

    fn execute_lda_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(3);
    }

    fn execute_lda_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absy(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(3);
    }

    fn execute_lda_indirect_x(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_x(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(2);
    }

    fn execute_lda_indirect_y(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_y(&self, memory);
        inst.log(&self, "LDA");
        lda!(self, inst);
        self.increment_pc(2);
    }
}
