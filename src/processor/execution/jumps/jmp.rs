use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_jmp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Absolute  => self.execute_jmp_abs(memory),
            MemoryMode::Indirect  => self.execute_jmp_indirect(memory),
            _                     => panic!("No {:?} memory mode for JMP", mode)
        }
    }
}

macro_rules! jmp {
    ($cpu:ident, $instruction:ident) => {{
        let memory_address = $instruction.memory_address.unwrap() as u16;
        $cpu.store_pc(memory_address);
    }}
}

impl CPU {
    fn execute_jmp_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "JMP");
        jmp!(self, inst);
    }

    fn execute_jmp_indirect(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_indirect(&self, memory);
        inst.log(&self, "JMP");
        jmp!(self, inst);
    }
}

#[cfg(test)]
mod jmp_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    static JMP_ABS: u8 = 0x4C;
    static JMP_INDIRECT: u8 = 0x6C;

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_jmp_abs(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![JMP_ABS, low_byte, high_byte]);

            assert_eq!(test_cpu.PC.0, 0x0000);

            test_cpu.execute_jmp_abs(&mut memory);

            prop_assert_eq!(test_cpu.PC.0, ((high_byte as u16) << 8) + low_byte as u16);
        }

        #[test]
        fn test_jmp_indirect(indirect_low_byte in 0x03u8..=0xFE, indirect_high_byte in 0x00u8..=0xFF, target_low_byte in 0x00u8..=0xFF, target_high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![JMP_INDIRECT, indirect_low_byte, indirect_high_byte]);
            memory.write_bulk(((indirect_high_byte as usize) << 8) + indirect_low_byte as usize, vec![target_low_byte, target_high_byte]);

            assert_eq!(test_cpu.PC.0, 0x0000);

            test_cpu.execute_jmp_indirect(&mut memory);

            prop_assert_eq!(test_cpu.PC.0, ((target_high_byte as u16) << 8) + target_low_byte as u16);
        }

        #[test]
        fn test_jmp_indirect_page_bound(indirect_low_byte in 0xFFu8..=0xFF, indirect_high_byte in 0x01u8..=0xFF, target_low_byte in 0x00u8..=0xFF, target_high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![JMP_INDIRECT, indirect_low_byte, indirect_high_byte]);
            memory.write((indirect_high_byte as usize) << 8, target_high_byte);
            memory.write(((indirect_high_byte as usize) << 8) + indirect_low_byte as usize, target_low_byte);

            assert_eq!(test_cpu.PC.0, 0x0000);

            test_cpu.execute_jmp_indirect(&mut memory);

            prop_assert_eq!(test_cpu.PC.0, ((target_high_byte as u16) << 8) + target_low_byte as u16);
        }
    }
}
