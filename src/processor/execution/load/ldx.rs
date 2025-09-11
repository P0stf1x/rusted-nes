use crate::logging::Logger;
use crate::processor::instruction::Instruction;
use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_ldx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldx_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldx_zpg(memory),
            MemoryMode::ZeroPageY => self.execute_ldx_zpgy(memory),
            MemoryMode::Absolute  => self.execute_ldx_abs(memory),
            MemoryMode::AbsoluteY => self.execute_ldx_absy(memory),
            _                     => panic!("No {:?} memory mode for LDX", mode)
        }
    }
}

macro_rules! ldx {
    ($cpu:ident, $instruction:ident) => {{
        let value = $instruction.value.unwrap();
        $cpu.store_x(value);
        $cpu.Z = $cpu.get_x() == 0;
        $cpu.N = $cpu.get_x() & 0b_1000_0000 != 0;
    }}
}

// LDX IMPL
impl CPU {
    fn execute_ldx_imm(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imm(&self, memory);
        inst.log(&self, "LDX");
        ldx!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ldx_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "LDX");
        ldx!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ldx_zpgy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgy(&self, memory);
        inst.log(&self, "LDX");
        ldx!(self, inst);
        self.increment_pc(2);
    }

    fn execute_ldx_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "LDX");
        ldx!(self, inst);
        self.increment_pc(3);
    }

    fn execute_ldx_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absy(&self, memory);
        inst.log(&self, "LDX");
        ldx!(self, inst);
        self.increment_pc(3);
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);

        test_cpu.X = Wrapping(0x42u8);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);

        test_cpu.X = Wrapping(0x69u8);
        assert_eq!(test_cpu.X.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
}
