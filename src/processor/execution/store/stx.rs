use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_stx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_stx_zpg(memory),
            MemoryMode::ZeroPageY => self.execute_stx_zpgy(memory),
            MemoryMode::Absolute  => self.execute_stx_abs(memory),
            _                     => panic!("No {:?} memory mode for STX", mode)
        }
    }
}

macro_rules! stx {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let value = $cpu.get_x();
        $memory.write($instruction.memory_address.unwrap() as usize, value);
    }}
}

// STX IMPL
impl CPU {
    fn execute_stx_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "STX");
        stx!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_stx_zpgy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgy(&self, memory);
        inst.log(&self, "STX");
        stx!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_stx_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "STX");
        stx!(self, inst, memory);
        self.increment_pc(3);
    }
}

#[cfg(test)]
mod stx_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    use std::num::Wrapping;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_stx_zeropage(low_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_x(value);
            memory.write_bulk(0x0100, vec![0x86, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_x(), value);
            assert_eq!(memory.read(low_byte as usize, 1), 0x00);

            test_cpu.execute_stx_zpg(&mut memory);

            assert_eq!(memory.read(low_byte as usize, 1), value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_stx_zeropage_y(low_byte in 0x00u8..=0xFF, y_value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_y(y_value);
            test_cpu.store_x(x_value);
            memory.write_bulk(0x0100, vec![0x96, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_y(), y_value);
            assert_eq!(test_cpu.get_x(), x_value);
            let result_zpg_address = (Wrapping::<u8>(low_byte) + Wrapping::<u8>(y_value)).0;
            assert_eq!(memory.read(result_zpg_address as usize, 1), 0x00);

            test_cpu.execute_stx_zpgy(&mut memory);

            assert_eq!(memory.read(result_zpg_address as usize, 1), x_value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_stx_absolute(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_x(value);
            memory.write_bulk(0x00, vec![0x8E, low_byte, high_byte]);

            assert_eq!(test_cpu.get_x(), value);
            let result_address = ((high_byte as usize) << 8) + low_byte as usize;
            if result_address > 0x0002 { // if address is in 0x0000..=0x0002 then there's no need to check since we just overwrote it
                assert_eq!(memory.read(result_address as usize, 1), 0x00);
            }

            test_cpu.execute_stx_abs(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), value as usize);
        }
    }
}
