use std::num::Wrapping;

use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_inc(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_inc_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_inc_zpgx(memory),
            MemoryMode::Absolute  => self.execute_inc_abs(memory),
            MemoryMode::AbsoluteX => self.execute_inc_absx(memory),
            _                     => panic!("No {:?} memory mode for INC", mode)
        }
    }
}

macro_rules! inc {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let result = (Wrapping::<u8>($instruction.value.unwrap()) + Wrapping::<u8>(1)).0;
        $memory.write($instruction.memory_address.unwrap() as usize, result);
        $cpu.Z = result == 0;
        $cpu.N = result & 0b_1000_0000 != 0;
    }}
}

impl CPU {
    fn execute_inc_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "INC");
        inc!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_inc_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "INC");
        inc!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_inc_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "INC");
        inc!(self, inst, memory);
        self.increment_pc(3);
    }

    fn execute_inc_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "INC");
        inc!(self, inst, memory);
        self.increment_pc(3);
    }
}
