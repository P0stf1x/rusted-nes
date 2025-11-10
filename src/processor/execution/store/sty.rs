use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_sty(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_sty_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sty_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sty_abs(memory),
            _                     => panic!("No {:?} memory mode for STY", mode)
        }
    }
}

macro_rules! sty {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let value = $cpu.get_y();
        $memory.write($instruction.memory_address.unwrap() as usize, value);
    }}
}

// STY IMPL
impl CPU {
    fn execute_sty_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "STY");
        sty!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_sty_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "STY");
        sty!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_sty_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "STY");
        sty!(self, inst, memory);
        self.increment_pc(3);
    }
}

#[cfg(test)]
mod sty_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    use std::num::Wrapping;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sty_zeropage(low_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_y(value);
            memory.write_bulk(0x0100, vec![0x84, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_y(), value);
            assert_eq!(memory.read(low_byte as usize, 1), 0x00);

            test_cpu.execute_sty_zpg(&mut memory);

            assert_eq!(memory.read(low_byte as usize, 1), value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sty_zeropage_x(low_byte in 0x00u8..=0xFF, y_value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_y(y_value);
            test_cpu.store_x(x_value);
            memory.write_bulk(0x0100, vec![0x94, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_y(), y_value);
            assert_eq!(test_cpu.get_x(), x_value);
            let result_zpg_address = (Wrapping::<u8>(low_byte) + Wrapping::<u8>(x_value)).0;
            assert_eq!(memory.read(result_zpg_address as usize, 1), 0x00);

            test_cpu.execute_sty_zpgx(&mut memory);

            assert_eq!(memory.read(result_zpg_address as usize, 1), y_value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sty_absolute(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_y(value);
            memory.write_bulk(0x00, vec![0x8C, low_byte, high_byte]);

            assert_eq!(test_cpu.get_y(), value);
            let result_address = ((high_byte as usize) << 8) + low_byte as usize;
            if result_address > 0x0002 { // if address is in 0x0000..=0x0002 then there's no need to check since we just overwrote it
                assert_eq!(memory.read(result_address as usize, 1), 0x00);
            }

            test_cpu.execute_sty_abs(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), value as usize);
        }
    }
}
