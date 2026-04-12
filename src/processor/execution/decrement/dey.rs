use std::num::Wrapping;

use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::CPU;
use crate::processor::instruction::Instruction;

impl CPU {
    pub fn execute_dey(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_dey_imp(memory),
            _                     => panic!("No {:?} memory mode for DEY", mode)
        }
    }
}

impl CPU {
    fn execute_dey_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "DEY");
        self.store_y((Wrapping::<u8>(self.get_y()) - Wrapping::<u8>(1)).0);
        self.Z = self.get_y() == 0;
        self.N = self.get_y() & 0b_1000_0000 != 0;
        self.increment_pc(1);
    }
}

#[cfg(test)]
mod dey_tests {
    use crate::memory::MEMORY_SIZE;

    use super::*;

    #[test]
    fn test_dey() {
        let mut test_cpu: CPU = CPU::new();
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_cpu.Y = Wrapping(0x02);
        assert_eq!(test_cpu.Y.0, 0x02);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp(&mut test_memory);
        assert_eq!(test_cpu.Y.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.Y = Wrapping(0x43);
        test_cpu.execute_dey_imp(&mut test_memory);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.Y = Wrapping(0x6a);
        test_cpu.execute_dey_imp(&mut test_memory);
        assert_eq!(test_cpu.Y.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }

    #[test]
    fn test_dey_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_cpu.Y = Wrapping(0x00u8);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp(&mut test_memory);
        assert_eq!(test_cpu.Y.0, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }

    #[test]
    fn test_dey_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut test_memory: MEM = MEM::new(MEMORY_SIZE);

        test_cpu.Y = Wrapping(0x01u8);
        assert_eq!(test_cpu.Y.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp(&mut test_memory);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}
