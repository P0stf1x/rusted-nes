use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_bit(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_bit_zpg(memory),
            MemoryMode::Absolute  => self.execute_bit_abs(memory),
            _                     => panic!("No {:?} memory mode for BIT", mode)
        }
    }
}

macro_rules! bit {
    ($cpu:ident, $instruction:ident) => {{
        $cpu.Z = ($cpu.A.0 & $instruction.value.unwrap()) == 0;
        $cpu.V = ($instruction.value.unwrap() & 0b_0100_0000) != 0;
        $cpu.N = ($instruction.value.unwrap() & 0b_1000_0000) != 0;
    }}
}

impl CPU {
    fn execute_bit_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "BIT");
        bit!(self, inst);
        self.increment_pc(2);
    }

    fn execute_bit_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "BIT");
        bit!(self, inst);
        self.increment_pc(3);
    }
}
