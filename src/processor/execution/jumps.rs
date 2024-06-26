use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_jmp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Absolute  => self.execute_jmp_abs(memory),
            MemoryMode::Indirect  => self.execute_jmp_indirect(memory),
            _                     => panic!("No {:?} memory mode for JMP", mode)
        }
    }

    pub fn execute_jsr(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Absolute  => self.execute_jsr_abs(memory),
            _                     => panic!("No {:?} memory mode for JSR", mode)
        }
    }

    pub fn execute_rts(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_rts_imp(memory),
            _                     => panic!("No {:?} memory mode for RTS", mode)
        }
    }
}

// JMP IMPL
impl CPU {
    fn execute_jmp_abs(&mut self, memory: &mut MEM) {
        self.PC = Wrapped::word(memory.read((self.PC + 1).value as usize, 2) as isize);
    }

    fn execute_jmp_indirect(&mut self, memory: &mut MEM) {
        if memory.data[self.PC.value as usize + 1] != 0xFF {
            let jump_pointer = self.fetch_mem_address((self.PC + 1).value as u16, memory);
            self.PC = Wrapped::word(self.fetch_mem_address(jump_pointer, memory) as isize);
        } else {
            // To preserve hardware error present in original 6502
            let jump_pointer = self.fetch_mem_address((self.PC + 1).value as u16, memory);
            let lsb = memory.data[jump_pointer as usize];
            let msb = memory.data[(jump_pointer - 0xFF) as usize];
            self.PC = Wrapped::word((((msb as u16) << 8) + lsb as u16) as isize);
        }
    }
}

// JSR IMPL
impl CPU {
    #[allow(non_snake_case)]
    fn execute_jsr_abs(&mut self, memory: &mut MEM) {
        self.PC += 2;
        let PCH: u8 = ((self.PC.value as u16) >> 8) as u8;
        let PCL: u8 = self.PC.value as u8;
        self.push_stack(PCH, memory);
        self.push_stack(PCL, memory);
        self.PC = Wrapped::byte(memory.read((self.PC - 1).value as usize, 2) as isize);
    }
}

// RTS IMPL
impl CPU {
    #[allow(non_snake_case)]
    fn execute_rts_imp(&mut self, memory: &mut MEM) {
        let PCL: u16 = self.pull_stack(memory) as u16;
        let PCH: u16 = (self.pull_stack(memory) as u16) << 8;
        self.PC = Wrapped::word((PCH + PCL + 1) as isize);
    }
}

#[cfg(test)]
mod jmp_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_jmp_abs(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![0x4C, low_byte, high_byte]);

            assert_eq!(test_cpu.PC, 0x0000);
            
            test_cpu.execute_jmp_abs(&mut memory);

            prop_assert_eq!(test_cpu.PC, (((high_byte as u16) << 8) + low_byte as u16) as isize);
        }

        #[test]
        fn test_jmp_indirect(indirect_low_byte in 0x03u8..=0xFE, indirect_high_byte in 0x00u8..=0xFF, target_low_byte in 0x00u8..=0xFF, target_high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![0x6C, indirect_low_byte, indirect_high_byte]);
            memory.write_bulk(((indirect_high_byte as usize) << 8) + indirect_low_byte as usize, vec![target_low_byte, target_high_byte]);

            assert_eq!(test_cpu.PC, 0x0000);

            test_cpu.execute_jmp_indirect(&mut memory);

            prop_assert_eq!(test_cpu.PC, (((target_high_byte as u16) << 8) + target_low_byte as u16) as isize);
        }

        #[test]
        fn test_jmp_indirect_page_bound(indirect_low_byte in 0xFFu8..=0xFF, indirect_high_byte in 0x01u8..=0xFF, target_low_byte in 0x00u8..=0xFF, target_high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![0x6C, indirect_low_byte, indirect_high_byte]);
            memory.write((indirect_high_byte as usize) << 8, target_high_byte);
            memory.write(((indirect_high_byte as usize) << 8) + indirect_low_byte as usize, target_low_byte);

            assert_eq!(test_cpu.PC, 0x0000);
            
            test_cpu.execute_jmp_indirect(&mut memory);

            prop_assert_eq!(test_cpu.PC, (((target_high_byte as u16) << 8) + target_low_byte as u16) as isize);
        }
    }
}
