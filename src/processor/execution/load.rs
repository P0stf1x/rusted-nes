use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_lda(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_lda_imm(memory),
            MemoryMode::ZeroPage  => self.execute_lda_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_lda_zpgx(memory),
            MemoryMode::Absolute  => self.execute_lda_abs(memory),
            MemoryMode::AbsoluteX => self.execute_lda_absx(memory),
            MemoryMode::AbsoluteY => self.execute_lda_absy(memory),
            MemoryMode::IndirectX => self.execute_lda_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_lda_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for LDA", mode)
        }
    }

    pub fn execute_ldx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldx_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldx_zpg(memory),
            MemoryMode::ZeroPageY => self.execute_ldx_zpgy(memory),
            MemoryMode::Absolute  => self.execute_ldx_abs(memory),
            MemoryMode::AbsoluteY => self.execute_ldx_absy(memory),
            _                     => panic!("No {:?} memory mode for LDX", mode)
        }
    }

    pub fn execute_ldy(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldy_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldy_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ldy_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ldy_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ldy_absx(memory),
            _                     => panic!("No {:?} memory mode for LDY", mode)
        }
    }
}

// LDA IMPL
impl CPU {
    fn execute_lda_imm(&mut self, memory: &mut MEM) {
        self.A = Wrapped::byte(memory.read(self.next_pc(), 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.A = Wrapped::byte(memory.read(memory_address, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
    
    fn execute_lda_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        self.A = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.A = Wrapped::byte(memory.read(memory_address, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        self.A = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.value as u16;
        self.A = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        self.A = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.value as u16;
        self.A = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// LDX IMPL
impl CPU {
    fn execute_ldx_imm(&mut self, memory: &mut MEM) {
        self.X = Wrapped::byte(memory.read(self.next_pc(), 1) as isize);
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.X = Wrapped::byte(memory.read(memory_address, 1) as isize);
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_zpgy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.Y.value as u8;
        self.X = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.X = Wrapped::byte(memory.read(memory_address, 1) as isize);
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ldx_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.value as u16;
        self.X = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.X == 0;
        self.N = (self.X.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

// LDY IMPL
impl CPU {
    fn execute_ldy_imm(&mut self, memory: &mut MEM) {
        self.Y = Wrapped::byte(memory.read(self.next_pc(), 1) as isize);
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
    
    fn execute_ldy_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.Y = Wrapped::byte(memory.data[memory_address] as isize);
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldy_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        self.Y = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldy_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.Y = Wrapped::byte(memory.read(memory_address, 1) as isize);
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ldy_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        self.Y = Wrapped::byte(memory.read(memory_address.0 as usize, 1) as isize);
        self.Z = self.Y == 0;
        self.N = (self.Y.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);
        
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);
        
        test_cpu.X = Wrapped::byte(0x42);
        assert_eq!(test_cpu.X, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);
        
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);
        
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);
        
        test_cpu.X = Wrapped::byte(0x69);
        assert_eq!(test_cpu.X, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
}

#[cfg(test)]
mod ldy_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldy_immediate() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lny_immediate_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);
        
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lny_immediate_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);
        
        test_cpu.Y = Wrapped::byte(0x42);
        assert_eq!(test_cpu.Y, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);
        
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage_negative() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);
        
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage_zero() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);
        
        test_cpu.Y = Wrapped::byte(0x69);
        assert_eq!(test_cpu.Y, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC, 0x0002);
    }
}
