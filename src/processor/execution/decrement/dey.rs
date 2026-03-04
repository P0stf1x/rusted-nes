use std::num::Wrapping;

use crate::processor::MemoryMode;
use crate::CPU;

impl CPU {
    pub fn execute_dey(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_dey_imp(),
            _                     => panic!("No {:?} memory mode for DEY", mode)
        }
    }
}

impl CPU {
    fn execute_dey_imp(&mut self) {
        self.store_y((Wrapping::<u8>(self.get_y()) - Wrapping::<u8>(1)).0);
        self.Z = self.get_y() == 0;
        self.N = self.get_y() & 0b_1000_0000 != 0;
        self.increment_pc(1);
    }
}

#[cfg(test)]
mod dey_tests {
    use super::*;

    #[test]
    fn test_dey() {
        let mut test_cpu: CPU = CPU::new();

        test_cpu.Y = Wrapping(0x02);
        assert_eq!(test_cpu.Y.0, 0x02);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapping(0x43);
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapping(0x6a);
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }
    
    #[test]
    fn test_dey_negative() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.Y = Wrapping(0x00u8);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y.0, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }
    
    #[test]
    fn test_dey_zero() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.Y = Wrapping(0x01u8);
        assert_eq!(test_cpu.Y.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}
