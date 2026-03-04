use std::num::Wrapping;

use crate::processor::MemoryMode;
use crate::CPU;

impl CPU {
    pub fn execute_inx(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_inx_imp(),
            _                     => panic!("No {:?} memory mode for INX", mode)
        }
    }
}

impl CPU {
    fn execute_inx_imp(&mut self) {
        self.store_x((Wrapping::<u8>(self.get_x()) + Wrapping::<u8>(1)).0);
        self.Z = self.get_x() == 0;
        self.N = self.get_x() & 0b_1000_0000 != 0;
        self.increment_pc(1);
    }
}

#[cfg(test)]
mod inx_tests {
    use super::*;

    #[test]
    fn test_inx() {
        let mut test_cpu: CPU = CPU::new();

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.execute_inx_imp();
        assert_eq!(test_cpu.X.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.X = Wrapping(0x41);
        test_cpu.execute_inx_imp();
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.X = Wrapping(0x68);
        test_cpu.execute_inx_imp();
        assert_eq!(test_cpu.X.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }
    
    #[test]
    fn test_inx_negative() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.X = Wrapping(0x7Fu8);
        assert_eq!(test_cpu.X.0, 0x7F);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_inx_imp();
        assert_eq!(test_cpu.X.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }
    
    #[test]
    fn test_inx_zero() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.X = Wrapping(0xFFu8);
        assert_eq!(test_cpu.X.0, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_inx_imp();
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}
