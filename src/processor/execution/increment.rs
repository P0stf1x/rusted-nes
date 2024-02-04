use crate::processor::*;

impl CPU {
    pub fn execute_inc(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_inc_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_inc_zpgx(memory),
            MemoryMode::Absolute  => self.execute_inc_abs(memory),
            MemoryMode::AbsoluteX => self.execute_inc_absx(memory),
            _                     => panic!("{:?}", memory.data)
        }
    }

    pub fn execute_inx(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_inx_imp(),
            _                     => panic!()
        }
    }

    pub fn execute_iny(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_iny_imp(),
            _                     => panic!()
        }
    }
}

// INC IMPL
impl CPU {
    fn execute_inc_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let mut memory_value = Wrapping(memory.read(memory_address, 1) as u8);
        memory_value += 1;
        memory.write(memory_address, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_inc_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let mut memory_value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        memory_value += 1;
        memory.write(memory_address.0 as usize, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_inc_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let mut memory_value = Wrapping(memory.read(memory_address, 1) as u8);
        memory_value += 1;
        memory.write(memory_address, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_inc_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let mut memory_value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        memory_value += 1;
        memory.write(memory_address.0 as usize, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

// INX IMPL
impl CPU {
    fn execute_inx_imp(&mut self) {
        self.X = self.X + Wrapping(1);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 1;
    }
}

// INY IMPL
impl CPU {
    fn execute_iny_imp(&mut self) {
        self.Y = self.Y + Wrapping(1);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 1;
    }
}

#[cfg(test)]
mod inx_tests {
    use super::*;

    #[test]
    fn test_inx() {
        let mut test_cpu: CPU = Default::default();

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
        let mut test_cpu: CPU = Default::default();
        
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
        let mut test_cpu: CPU = Default::default();
        
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

#[cfg(test)]
mod iny_tests {
    use super::*;

    #[test]
    fn test_iny() {
        // TODO: IMPLEMENT PC TEST

        let mut test_cpu: CPU = Default::default();

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.execute_iny_imp();
        assert_eq!(test_cpu.Y.0, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapping(0x41);
        test_cpu.execute_iny_imp();
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapping(0x68);
        test_cpu.execute_iny_imp();
        assert_eq!(test_cpu.Y.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }
    
    #[test]
    fn test_iny_negative() {
        let mut test_cpu: CPU = Default::default();
        
        test_cpu.Y = Wrapping(0x7Fu8);
        assert_eq!(test_cpu.Y.0, 0x7F);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_iny_imp();
        assert_eq!(test_cpu.Y.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }
    
    #[test]
    fn test_iny_zero() {
        let mut test_cpu: CPU = Default::default();
        
        test_cpu.Y = Wrapping(0xFFu8);
        assert_eq!(test_cpu.Y.0, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_iny_imp();
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}