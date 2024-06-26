use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_and(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_and_imm(memory),
            MemoryMode::ZeroPage  => self.execute_and_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_and_zpgx(memory),
            MemoryMode::Absolute  => self.execute_and_abs(memory),
            MemoryMode::AbsoluteX => self.execute_and_absx(memory),
            MemoryMode::AbsoluteY => self.execute_and_absy(memory),
            MemoryMode::IndirectX => self.execute_and_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_and_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for AND", mode)
        }
    }

    pub fn execute_eor(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_eor_imm(memory),
            MemoryMode::ZeroPage  => self.execute_eor_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_eor_zpgx(memory),
            MemoryMode::Absolute  => self.execute_eor_abs(memory),
            MemoryMode::AbsoluteX => self.execute_eor_absx(memory),
            MemoryMode::AbsoluteY => self.execute_eor_absy(memory),
            MemoryMode::IndirectX => self.execute_eor_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_eor_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for EOR", mode)
        }
    }

    pub fn execute_ora(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_ora_imm(memory),
            MemoryMode::ZeroPage  => self.execute_ora_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ora_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ora_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ora_absx(memory),
            MemoryMode::AbsoluteY => self.execute_ora_absy(memory),
            MemoryMode::IndirectX => self.execute_ora_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_ora_indirect_y(memory),
            _                     => panic!("No {:?} memory mode for ORA", mode)
        }
    }

    pub fn execute_bit(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::ZeroPage  => self.execute_bit_zpg(memory),
            MemoryMode::Absolute  => self.execute_bit_abs(memory),
            _                     => panic!("No {:?} memory mode for BIT", mode)
        }
    }
}

// AND IMPL
impl CPU {
    fn execute_and_imm(&mut self, memory: &mut MEM) {
        let memory_value = memory.read(self.next_pc(), 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_and_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_and_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_and_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_and_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_and_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_and_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_and_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) & memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// EOR IMPL
impl CPU {
    fn execute_eor_imm(&mut self, memory: &mut MEM) {
        let memory_value = memory.read(self.next_pc(), 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_eor_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_eor_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_eor_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_eor_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_eor_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_eor_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_eor_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) ^ memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// ORA IMPL
impl CPU {
    fn execute_ora_imm(&mut self, memory: &mut MEM) {
        let memory_value = memory.read(self.next_pc(), 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ora_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ora_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ora_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ora_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ora_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_ora_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X.value as u8;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_ora_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.value as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.A = Wrapped::byte(((self.A.value as u8) | memory_value) as isize);
        self.Z = self.A == 0;
        self.N = (self.A.value as u8) & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// BIT IMPL
impl CPU {
    fn execute_bit_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read((self.PC + 1).value as usize, 1);
        let value = memory.read(memory_address, 1) as u8;
        self.Z = ((self.A.value as u8) & value) == 0;
        self.V = (value & 0b_0100_0000) != 0;
        self.N = (value & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_bit_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read((self.PC + 1).value as usize, 2);
        let value = memory.read(memory_address, 1) as u8;
        self.Z = ((self.A.value as u8) & value) == 0;
        self.V = (value & 0b_0100_0000) != 0;
        self.N = (value & 0b_1000_0000) != 0;
        self.PC += 3;
    }
}
