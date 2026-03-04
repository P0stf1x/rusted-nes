use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_asl(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Acc       => self.execute_asl_acc(memory),
            MemoryMode::ZeroPage  => self.execute_asl_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_asl_zpgx(memory),
            MemoryMode::Absolute  => self.execute_asl_abs(memory),
            MemoryMode::AbsoluteX => self.execute_asl_absx(memory),
            _                     => panic!("No {:?} memory mode for ASL", mode)
        }
    }
}

macro_rules! asl {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let value = $instruction.value.unwrap();
        $cpu.C = (value & 0b_1000_0000) != 0;
        let result = value << 1;
        $cpu.Z = result == 0;
        $cpu.N = (result & 0b_1000_0000) != 0;
        if $instruction.mode == MemoryMode::Acc {
            $cpu.store_a(result); // potential for abstraction, but since there's only 4 instructions with acc memory mode i dont think its necessary
        } else {
            $memory.write($instruction.memory_address.unwrap() as usize, result);
        }
    }}
}

// ASL IMPL
impl CPU {
    fn execute_asl_acc(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_acc(&self, memory);
        inst.log(&self, "ASL");
        asl!(self, inst, memory);
        self.increment_pc(1);
    }

    fn execute_asl_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "ASL");
        asl!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_asl_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "ASL");
        asl!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_asl_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "ASL");
        asl!(self, inst, memory);
        self.increment_pc(3);
    }

    fn execute_asl_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "ASL");
        asl!(self, inst, memory);
        self.increment_pc(3);
    }
}

#[cfg(test)]
mod asl_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    use std::num::Wrapping;
    use proptest::prelude::*;

    struct ExpectedResult {
        pub result: u8,
        pub result_carry: bool,
        pub result_zero: bool,
        pub result_negative: bool,
    }

    fn get_result_from_value(value: u8) -> ExpectedResult {
        ExpectedResult {
            result: value << 1,
            result_carry: (value & 0b_1000_0000) != 0,
            result_zero: (value << 1) == 0,
            result_negative: ((value << 1) & 0b_1000_0000) != 0,
        }
    }

    proptest! {
        #[test]
        fn test_asl_acc(value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(value);
            memory.write(0x0000, 0x0A);

            assert_eq!(test_cpu.get_a(), value);
            let r = get_result_from_value(value);

            test_cpu.execute_asl_acc(&mut memory);

            assert_eq!(test_cpu.get_a(), r.result);
            assert_eq!(test_cpu.C, r.result_carry);
            assert_eq!(test_cpu.Z, r.result_zero);
            assert_eq!(test_cpu.N, r.result_negative);
        }
    }

    proptest! {
        #[test]
        fn test_asl_zeropage(low_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0100, vec![0x06, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(memory.read(low_byte as usize, 1), 0x00);
            memory.write(low_byte as usize, value);
            assert_eq!(memory.read(low_byte as usize, 1) as u8, value);
            let r = get_result_from_value(value);

            test_cpu.execute_asl_zpg(&mut memory);

            assert_eq!(memory.read(low_byte as usize, 1) as u8, r.result);
            assert_eq!(test_cpu.C, r.result_carry);
            assert_eq!(test_cpu.Z, r.result_zero);
            assert_eq!(test_cpu.N, r.result_negative);
        }
    }

    proptest! {
        #[test]
        fn test_asl_zeropage_x(low_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_x(x_value);
            memory.write_bulk(0x0100, vec![0x16, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_x(), x_value);
            let result_zpg_address = (Wrapping::<u8>(low_byte) + Wrapping::<u8>(x_value)).0;
            assert_eq!(memory.read(result_zpg_address as usize, 1), 0x00);
            memory.write(result_zpg_address as usize, value);
            let r = get_result_from_value(value);

            test_cpu.execute_asl_zpgx(&mut memory);

            assert_eq!(memory.read(result_zpg_address as usize, 1) as u8, r.result);
            assert_eq!(test_cpu.C, r.result_carry);
            assert_eq!(test_cpu.Z, r.result_zero);
            assert_eq!(test_cpu.N, r.result_negative);
        }
    }

    proptest! {
        #[test]
        fn test_asl_absolute(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            if high_byte == 0x00 {
                memory.write_bulk(0x0100, vec![0x0E, low_byte, high_byte]);
                test_cpu.store_pc(0x0100);
            } else {
                memory.write_bulk(0x0000, vec![0x0E, low_byte, high_byte]);
            }

            let result_address = ((high_byte as usize) << 8) + low_byte as usize;
            assert_eq!(memory.read(result_address, 1) as u8, 0);
            memory.write(result_address, value);
            assert_eq!(memory.read(result_address, 1) as u8, value);
            let r = get_result_from_value(value);

            test_cpu.execute_asl_abs(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1) as u8, r.result);
            assert_eq!(test_cpu.C, r.result_carry);
            assert_eq!(test_cpu.Z, r.result_zero);
            assert_eq!(test_cpu.N, r.result_negative);
        }
    }

    proptest! {
        #[test]
        fn test_asl_absolute_x(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_x(x_value);
            if high_byte == 0x00 {
                memory.write_bulk(0x0100, vec![0x1E, low_byte, high_byte]);
                test_cpu.store_pc(0x0100);
            } else {
                memory.write_bulk(0x0000, vec![0x1E, low_byte, high_byte]);
            }

            assert_eq!(test_cpu.get_x(), x_value);
            let absolute_address = ((high_byte as u16) << 8) + low_byte as u16;
            let result_address = (Wrapping::<u16>(absolute_address) + Wrapping::<u16>(x_value as u16)).0;
            assert_eq!(memory.read(result_address as usize, 1) as u8, 0);
            memory.write(result_address as usize, value);
            assert_eq!(memory.read(result_address as usize, 1) as u8, value);
            let r = get_result_from_value(value);

            test_cpu.execute_asl_absx(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1) as u8, r.result);
            assert_eq!(test_cpu.C, r.result_carry);
            assert_eq!(test_cpu.Z, r.result_zero);
            assert_eq!(test_cpu.N, r.result_negative);
        }
    }
}
