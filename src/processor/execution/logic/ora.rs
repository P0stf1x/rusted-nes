use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_ora(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ora_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ora_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ora_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ora_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ora_absx(memory),
            MemoryMode::AbsoluteY => self.execute_ora_absy(memory),
            MemoryMode::IndirectX => self.execute_ora_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_ora_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for ORA", mode)
        }
    }
}

macro_rules! ora {
    ($cpu:ident, $instruction:ident) => {{
        $cpu.store_a($cpu.get_a() | $instruction.value.unwrap());
        $cpu.Z = $cpu.get_a() == 0;
        $cpu.N = $cpu.get_a() & 0b_1000_0000 != 0;
    }}
}

impl CPU {
    fn execute_ora_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ora_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ora_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ora_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(3);
    }

    fn execute_ora_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(3);
    }

    fn execute_ora_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absy(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(3);
    }

    fn execute_ora_indirect_x(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_x(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ora_indirect_y(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_y(&self, memory);
        inst.log(&self, "ORA");
        ora!(self, inst);
        self.increment_pc(2);
    }
}
