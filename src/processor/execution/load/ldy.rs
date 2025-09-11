use crate::logging::Logger;
use crate::processor::instruction::Instruction;
use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_ldy(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldy_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldy_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ldy_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ldy_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ldy_absx(memory),
            _                     => panic!("No {:?} memory mode for LDY", mode)
        }
    }
}

macro_rules! ldy {
    ($cpu:ident, $instruction:ident) => {{
        let value = $instruction.value.unwrap();
        $cpu.store_y(value);
        $cpu.Z = $cpu.get_y() == 0;
        $cpu.N = $cpu.get_y() & 0b_1000_0000 != 0;
    }}
}

// LDY IMPL
impl CPU {
    fn execute_ldy_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "LDY");
        ldy!(self, inst);
        self.increment_pc(2);
    }
    
    fn execute_ldy_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "LDY");
        ldy!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ldy_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "LDY");
        ldy!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ldy_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "LDY");
        ldy!(self, inst);
        self.increment_pc(3);
    }

    fn execute_ldy_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "LDY");
        ldy!(self, inst);
        self.increment_pc(3);
    }
}

#[cfg(test)]
mod ldy_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldy_immediate() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_immediate_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }

    #[test]
    fn test_lny_immediate_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);

        test_cpu.Y = Wrapping(0x42u8);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }

    #[test]
    fn test_lny_zeropage() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }

    #[test]
    fn test_lny_zeropage_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }

    #[test]
    fn test_lny_zeropage_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);

        test_cpu.Y = Wrapping(0x69u8);
        assert_eq!(test_cpu.Y.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
}
