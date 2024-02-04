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
            _                     => panic!()
        }
    }

    pub fn execute_ldx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldx_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldx_zpg(memory),
            MemoryMode::ZeroPageY => self.execute_ldx_zpgy(memory),
            MemoryMode::Absolute  => self.execute_ldx_abs(memory),
            MemoryMode::AbsoluteY => self.execute_ldx_absy(memory),
            _                     => panic!()
        }
    }

    pub fn execute_ldy(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ldy_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ldy_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ldy_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ldy_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ldy_absx(memory),
            _                     => panic!()
        }
    }
}

// LDA IMPL
impl CPU {
    fn execute_lda_imm(&mut self, memory: &mut MEM) {
        self.A = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.A = Wrapping(memory.read(memory_address, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }
    
    fn execute_lda_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        self.A = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.A = Wrapping(memory.read(memory_address, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        self.A = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.0 as u16;
        self.A = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_lda_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        self.A = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_lda_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.0 as u16;
        self.A = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// LDX IMPL
impl CPU {
    fn execute_ldx_imm(&mut self, memory: &mut MEM) {
        self.X = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.X = Wrapping(memory.read(memory_address, 1) as u8);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_zpgy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.Y;
        self.X = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldx_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.X = Wrapping(memory.read(memory_address, 1) as u8);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ldx_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.0 as u16;
        self.X = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.X.0 == 0;
        self.N = self.X.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

// LDY IMPL
impl CPU {
    fn execute_ldy_imm(&mut self, memory: &mut MEM) {
        self.Y = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }
    
    fn execute_ldy_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        self.Y = Wrapping(memory.data[memory_address]);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldy_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        self.Y = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ldy_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        self.Y = Wrapping(memory.read(memory_address, 1) as u8);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ldy_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        self.Y = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.Z = self.Y.0 == 0;
        self.N = self.Y.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_negative() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);
        
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_immediate_zero() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);
        
        test_cpu.X = Wrapping(0x42u8);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_imm(&mut memory);
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);
        
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_negative() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);
        
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lnx_zeropage_zero() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);
        
        test_cpu.X = Wrapping(0x69u8);
        assert_eq!(test_cpu.X.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);

        test_cpu.execute_ldx_zpg(&mut memory);
        assert_eq!(test_cpu.X.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
}

#[cfg(test)]
mod ldy_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_ldy_immediate() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);

        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_immediate_negative() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x80]);
        
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_immediate_zero() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x00]);
        
        test_cpu.Y = Wrapping(0x42u8);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_imm(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x69]);
        memory.data[0x69..0x6A].copy_from_slice(&[0x42]);
        
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x42);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage_negative() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0x42]);
        memory.data[0x42..0x43].copy_from_slice(&[0x80]);
        
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x80);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, true);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
    
    #[test]
    fn test_lny_zeropage_zero() {
        let mut test_cpu: CPU = Default::default();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xA2, 0xAB]);
        memory.data[0xAB..0xAC].copy_from_slice(&[0x00]);
        
        test_cpu.Y = Wrapping(0x69u8);
        assert_eq!(test_cpu.Y.0, 0x69);
        assert_eq!(test_cpu.Z, false);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0000);
        
        test_cpu.execute_ldy_zpg(&mut memory);
        assert_eq!(test_cpu.Y.0, 0x00);
        assert_eq!(test_cpu.Z, true);
        assert_eq!(test_cpu.N, false);
        assert_eq!(test_cpu.PC.0, 0x0002);
    }
}