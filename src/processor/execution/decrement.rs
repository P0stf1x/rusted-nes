use crate::processor::*;

impl CPU {
    pub fn execute_dec(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_dec_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_dec_zpgx(memory),
            MemoryMode::Absolute  => self.execute_dec_abs(memory),
            MemoryMode::AbsoluteX => self.execute_dec_absx(memory),
            _                     => panic!("No {:?} memory mode for DEC", mode)
        }
    }

    pub fn execute_dex(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_dex_imp(),
            _                     => panic!("No {:?} memory mode for DEX", mode)
        }
    }

    pub fn execute_dey(&mut self, mode: MemoryMode) {
        match mode {
            MemoryMode::Implicit  => self.execute_dey_imp(),
            _                     => panic!("No {:?} memory mode for DEY", mode)
        }
    }
}

// DEC IMPL
impl CPU {
    fn execute_dec_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let mut memory_value = Wrapping(memory.read(memory_address, 1) as u8);
        memory_value -= 1;
        memory.write(memory_address, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_dec_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let mut memory_value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        memory_value -= 1;
        memory.write(memory_address.0 as usize, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_dec_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let mut memory_value = Wrapping(memory.read(memory_address, 1) as u8);
        memory_value -= 1;
        memory.write(memory_address, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_dec_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        let mut memory_value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        memory_value -= 1;
        memory.write(memory_address.0 as usize, memory_value.0);
        self.Z = memory_value.0 == 0;
        self.N = memory_value.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

// DEX IMPL
impl CPU {
    fn execute_dex_imp(&mut self) {
        self.X -= 1;
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 1;
    }
}

// DEY IMPL
impl CPU {
    fn execute_dey_imp(&mut self) {
        self.Y -= 1;
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 1;
    }
}

#[cfg(test)]
mod dex_tests {
    use super::*;

    #[test]
    fn test_dex() {
        let mut test_cpu: CPU = CPU::new();

        test_cpu.X = Wrapped::byte(0x02);
        assert_eq!(test_cpu.X, 0x02);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.execute_dex_imp();
        assert_eq!(test_cpu.X, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.X = Wrapped::byte(0x43);
        test_cpu.execute_dex_imp();
        assert_eq!(test_cpu.X, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.X = Wrapped::byte(0x6a);
        test_cpu.execute_dex_imp();
        assert_eq!(test_cpu.X, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }
    
    #[test]
    fn test_dex_negative() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.Y = Wrapped::byte(0x00);
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dex_imp();
        assert_eq!(test_cpu.X, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }
    
    #[test]
    fn test_dex_zero() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.X = Wrapped::byte(0x01);
        assert_eq!(test_cpu.X, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dex_imp();
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}

#[cfg(test)]
mod dey_tests {
    use super::*;

    #[test]
    fn test_dey() {
        let mut test_cpu: CPU = CPU::new();

        test_cpu.Y = Wrapped::byte(0x02);
        assert_eq!(test_cpu.Y, 0x02);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapped::byte(0x43);
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        
        test_cpu.Y = Wrapped::byte(0x6a);
        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
    }
    
    #[test]
    fn test_dey_negative() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.Y = Wrapped::byte(0x00);
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y, 0xFF);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
    }
    
    #[test]
    fn test_dey_zero() {
        let mut test_cpu: CPU = CPU::new();
        
        test_cpu.Y = Wrapped::byte(0x01);
        assert_eq!(test_cpu.Y, 0x01);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);

        test_cpu.execute_dey_imp();
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
    }
}
