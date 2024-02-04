use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_adc(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_adc_imm(memory),
            MemoryMode::ZeroPage  => self.execute_adc_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_adc_zpgx(memory),
            MemoryMode::Absolute  => self.execute_adc_abs(memory),
            MemoryMode::AbsoluteX => self.execute_adc_absx(memory),
            MemoryMode::AbsoluteY => self.execute_adc_absy(memory),
            MemoryMode::IndirectX => self.execute_adc_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_adc_indirect_y(memory),
            _                     => panic!()
        }
    }

    pub fn execute_sbc(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_sbc_imm(memory),
            MemoryMode::ZeroPage  => self.execute_sbc_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_sbc_zpgx(memory),
            MemoryMode::Absolute  => self.execute_sbc_abs(memory),
            MemoryMode::AbsoluteX => self.execute_sbc_absx(memory),
            MemoryMode::AbsoluteY => self.execute_sbc_absy(memory),
            MemoryMode::IndirectX => self.execute_sbc_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_sbc_indirect_y(memory),
            _                     => panic!()
        }
    }

    pub fn execute_cmp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_cmp_imm(memory),
            MemoryMode::ZeroPage  => self.execute_cmp_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_cmp_zpgx(memory),
            MemoryMode::Absolute  => self.execute_cmp_abs(memory),
            MemoryMode::AbsoluteX => self.execute_cmp_absx(memory),
            MemoryMode::AbsoluteY => self.execute_cmp_absy(memory),
            MemoryMode::IndirectX => self.execute_cmp_indirect_x(memory),
            MemoryMode::IndirectY => self.execute_cmp_indirect_y(memory),
            _                     => panic!()
        }
    }

    pub fn execute_cpx(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_cpx_imm(memory),
            MemoryMode::ZeroPage  => self.execute_cpx_zpg(memory),
            MemoryMode::Absolute  => self.execute_cpx_abs(memory),
            _                     => panic!()
        }
    }

    pub fn execute_cpy(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Immediate => self.execute_cpy_imm(memory),
            MemoryMode::ZeroPage  => self.execute_cpy_zpg(memory),
            MemoryMode::Absolute  => self.execute_cpy_abs(memory),
            _                     => panic!()
        }
    }
}

// ADC IMPL
impl CPU {
    fn execute_adc_imm(&mut self, memory: &mut MEM) {
        let memory_value = memory.read(self.next_pc(), 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = (self.A.0 & 0b_1000_0000) != (prev_a.0 & 0b_1000_0000);
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_adc_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_adc_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_adc_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_adc_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_adc_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_adc_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_adc_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let carry = (self.A.0 as u16 + memory_value as u16) > 0xFF;
        let prev_a = self.A;
        self.A += memory_value;
        if self.C {self.A += 1};
        self.C = carry;
        self.V = self.A.0 & 0b_1000_0000 != prev_a.0 & 0b_1000_0000;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// SBC IMPL
impl CPU {
    fn execute_sbc_imm(&mut self, memory: &mut MEM) {
        let memory_value = memory.read(self.next_pc(), 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_sbc_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_sbc_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_sbc_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_sbc_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_sbc_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 3;
    }

    fn execute_sbc_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }

    fn execute_sbc_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = (self.A.0 as i8) as i32;
        self.A -= memory_value;
        if !self.C {self.A -= 1};
        temp -= (memory_value as i8) as i32;
        let carry = temp >= 0;
        self.C = carry;
        self.V = temp > 127 || temp < -128;
        self.Z = self.A.0 == 0;
        self.N = self.A.0 & 0b_1000_0000 != 0;
        self.PC += 2;
    }
}

// CMP IMPL
impl CPU {
    fn execute_cmp_imm(&mut self, memory: &mut MEM) {
        let value = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cmp_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cmp_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cmp_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 3;
    }

    fn execute_cmp_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 3;
    }

    fn execute_cmp_absy(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.Y.0 as u16;
        let value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 3;
    }

    fn execute_cmp_indirect_x(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cmp_indirect_y(&mut self, memory: &mut MEM) {
        let memory_pointer = memory.read(self.next_pc(), 1);
        let mut memory_address = Wrapping(memory.read(memory_pointer, 2) as u16);
        memory_address += self.Y.0 as u16;
        let value = Wrapping(memory.read(memory_address.0 as usize, 1) as u8);
        self.C = self.A >= value;
        self.Z = self.A == value;
        self.N = ((self.A - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }
}

// CPX IMPL
impl CPU {
    fn execute_cpx_imm(&mut self, memory: &mut MEM) {
        let value = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.C = self.X >= value;
        self.Z = self.X == value;
        self.N = ((self.X - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cpx_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.X >= value;
        self.Z = self.X == value;
        self.N = ((self.X - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cpx_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.X >= value;
        self.Z = self.X == value;
        self.N = ((self.X - value).0 & 0b_1000_0000) != 0;
        self.PC += 3;
    }
}

// CPY IMPL
impl CPU {
    fn execute_cpy_imm(&mut self, memory: &mut MEM) {
        let value = Wrapping(memory.read(self.next_pc(), 1) as u8);
        self.C = self.Y >= value;
        self.Z = self.Y == value;
        self.N = ((self.Y - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cpy_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.Y >= value;
        self.Z = self.Y == value;
        self.N = ((self.Y - value).0 & 0b_1000_0000) != 0;
        self.PC += 2;
    }

    fn execute_cpy_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let value = Wrapping(memory.read(memory_address, 1) as u8);
        self.C = self.Y >= value;
        self.Z = self.Y == value;
        self.N = ((self.Y - value).0 & 0b_1000_0000) != 0;
        self.PC += 3;
    }
}





