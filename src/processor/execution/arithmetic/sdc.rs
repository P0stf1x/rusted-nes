use std::num::Wrapping;

use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_sbc(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_sbc_imm(memory),
            MemoryMode::ZeroPage  => self.execute_sbc_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sbc_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sbc_abs(memory),
            MemoryMode::AbsoluteX => self.execute_sbc_absx(memory),
            MemoryMode::AbsoluteY => self.execute_sbc_absy(memory),
            MemoryMode::IndirectX => self.execute_sbc_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_sbc_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for SBC", mode)
        }
    }
}

macro_rules! sbc {
    ($cpu:ident, $instruction:ident) => {{
        let carry = ($cpu.get_a() as i16 - $instruction.value.unwrap() as i16 -1 + $cpu.C as i16) < 0;
        let prev_a = $cpu.get_a();
        $cpu.store_a((Wrapping::<u8>($cpu.get_a()) - Wrapping::<u8>($instruction.value.unwrap()) - Wrapping::<u8>(1) + Wrapping::<u8>($cpu.C as u8)).0);
        $cpu.C = carry;
        $cpu.V = ($cpu.get_a() & 0b_1000_0000) != (prev_a & 0b_1000_0000);
        $cpu.Z = $cpu.get_a() == 0;
        $cpu.N = $cpu.get_a() & 0b_1000_0000 != 0;
    }}
}

impl CPU {
    fn execute_sbc_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(2);
    }

    fn execute_sbc_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(2);
    }

    fn execute_sbc_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(2);
    }

    fn execute_sbc_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(3);
    }

    fn execute_sbc_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(3);
    }

    fn execute_sbc_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absy(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(3);
    }

    fn execute_sbc_indirect_x(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_x(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(2);
    }

    fn execute_sbc_indirect_y(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_y(&self, memory);
        inst.log(&self, "SBC");
        sbc!(self, inst);
        self.increment_pc(2);
    }
}
