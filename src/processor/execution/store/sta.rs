use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_sta(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_sta_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sta_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sta_abs(memory),
            MemoryMode::AbsoluteX => self.execute_sta_absx(memory),
            MemoryMode::AbsoluteY => self.execute_sta_absy(memory),
            MemoryMode::IndirectX => self.execute_sta_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_sta_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for STA", mode)
        }
    }
}

macro_rules! sta {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let value = $cpu.get_a();
        $memory.write($instruction.memory_address.unwrap() as usize, value);
    }}
}

// STA IMPL
impl CPU {
    fn execute_sta_zpg(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpg(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_sta_zpgx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_zpgx(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_sta_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(3);
    }

    fn execute_sta_absx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absx(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(3);
    }

    fn execute_sta_absy(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_absy(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(3);
    }

    fn execute_sta_indirect_x(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_x(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(2);
    }

    fn execute_sta_indirect_y(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect_y(&self, memory);
        inst.log(&self, "STA");
        sta!(self, inst, memory);
        self.increment_pc(2);
    }
}

#[cfg(test)]
mod sta_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    use std::num::Wrapping;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sta_zeropage(low_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(value);
            memory.write_bulk(0x0100, vec![0x85, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_a(), value);
            assert_eq!(memory.read(low_byte as usize, 1), 0x00);

            test_cpu.execute_sta_zpg(&mut memory);

            assert_eq!(memory.read(low_byte as usize, 1), value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_zeropage_x(low_byte in 0x00u8..=0xFF, a_value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(a_value);
            test_cpu.store_x(x_value);
            memory.write_bulk(0x0100, vec![0x95, low_byte]);
            test_cpu.store_pc(0x0100);

            assert_eq!(test_cpu.get_a(), a_value);
            assert_eq!(test_cpu.get_x(), x_value);
            let result_zpg_address = (Wrapping::<u8>(low_byte) + Wrapping::<u8>(x_value)).0;
            assert_eq!(memory.read(result_zpg_address as usize, 1), 0x00);

            test_cpu.execute_sta_zpgx(&mut memory);

            assert_eq!(memory.read(result_zpg_address as usize, 1), a_value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_absolute(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(value);
            memory.write_bulk(0x00, vec![0x8D, low_byte, high_byte]);

            assert_eq!(test_cpu.get_a(), value);
            let result_address = ((high_byte as usize) << 8) + low_byte as usize;
            if result_address > 0x0002 { // if address is in 0x0000..=0x0002 then there's no need to check since we just overwrote it
                assert_eq!(memory.read(result_address as usize, 1), 0x00);
            }

            test_cpu.execute_sta_abs(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_absolute_x(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, a_value in 0x00u8..=0xFF, x_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(a_value);
            test_cpu.store_x(x_value);
            memory.write_bulk(0x00, vec![0x9D, low_byte, high_byte]);

            assert_eq!(test_cpu.get_a(), a_value);
            assert_eq!(test_cpu.get_x(), x_value);
            let absolute_address = ((high_byte as u16) << 8) + low_byte as u16;
            let result_address = (Wrapping::<u16>(absolute_address) + Wrapping::<u16>(x_value as u16)).0;
            if result_address > 0x0002 { // if address is in 0x0000..=0x0002 then there's no need to check since we just overwrote it
                assert_eq!(memory.read(result_address as usize, 1), 0x00);
            }

            test_cpu.execute_sta_absx(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), a_value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_absolute_y(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF, a_value in 0x00u8..=0xFF, y_value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(a_value);
            test_cpu.store_y(y_value);
            memory.write_bulk(0x00, vec![0x99, low_byte, high_byte]);

            assert_eq!(test_cpu.get_a(), a_value);
            assert_eq!(test_cpu.get_y(), y_value);
            let absolute_address = ((high_byte as u16) << 8) + low_byte as u16;
            let result_address = (Wrapping::<u16>(absolute_address) + Wrapping::<u16>(y_value as u16)).0;
            if result_address > 0x0002 { // if address is in 0x0000..=0x0002 then there's no need to check since we just overwrote it
                assert_eq!(memory.read(result_address as usize, 1), 0x00);
            }

            test_cpu.execute_sta_absy(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), a_value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_indirect_x(table_addr in 0x00u8..=0xFF, table_offset in 0x00u8..=0xFF, jmp_low_byte in 0x00u8..=0xFF, jmp_high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(value);
            test_cpu.store_x(table_offset);
            test_cpu.store_pc(0x0100);
            memory.write_bulk(0x0100, vec![0x81, table_addr]);

            assert_eq!(test_cpu.get_a(), value);
            assert_eq!(test_cpu.get_x(), table_offset);
            let jmp_low_byte_addr = (Wrapping::<u8>(table_addr) + Wrapping::<u8>(table_offset)).0;
            let jmp_high_byte_addr = (Wrapping::<u8>(table_addr) + Wrapping::<u8>(table_offset) + Wrapping::<u8>(1)).0;
            memory.write(jmp_low_byte_addr as usize, jmp_low_byte);
            memory.write(jmp_high_byte_addr as usize, jmp_high_byte);
            let result_address = ((jmp_high_byte as usize) << 8) + (jmp_low_byte as usize);
            // if it passed all the previous tests, memory at addr should be zero, and checking that explicitly is hard so no check

            test_cpu.execute_sta_indirect_x(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), value as usize);
        }
    }

    proptest! {
        #[test]
        fn test_sta_indirect_y(table_addr in 0x00u8..=0xFF, jmp_offset in 0x00u8..=0xFF, jmp_low_byte in 0x00u8..=0xFF, jmp_high_byte in 0x00u8..=0xFF, value in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_a(value);
            test_cpu.store_y(jmp_offset);
            test_cpu.store_pc(0x0100);
            memory.write_bulk(0x0100, vec![0x81, table_addr]);

            assert_eq!(test_cpu.get_a(), value);
            assert_eq!(test_cpu.get_y(), jmp_offset);
            let jmp_high_byte_addr = (Wrapping::<u8>(table_addr) + Wrapping::<u8>(1)).0;
            memory.write(table_addr as usize, jmp_low_byte);
            memory.write(jmp_high_byte_addr as usize, jmp_high_byte);
            let result_address = (Wrapping::<u16>(((jmp_high_byte as u16) << 8) + (jmp_low_byte as u16)) + Wrapping::<u16>(jmp_offset as u16)).0 as usize;
            // if it passed all the previous tests, memory at addr should be zero, and checking that explicitly is hard so no check

            test_cpu.execute_sta_indirect_y(&mut memory);

            assert_eq!(memory.read(result_address as usize, 1), value as usize);
        }
    }
}
