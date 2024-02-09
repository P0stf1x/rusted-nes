use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_asl(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Acc       => self.execute_asl_acc(),
            MemoryMode::ZeroPage  => self.execute_asl_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_asl_zpgx(memory),
            MemoryMode::Absolute  => self.execute_asl_abs(memory),
            MemoryMode::AbsoluteX => self.execute_asl_absx(memory),
            _                     => panic!("No {:?} memory mode for ASL", mode)
        }
    }

    pub fn execute_lsr(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Acc       => self.execute_lsr_acc(),
            MemoryMode::ZeroPage  => self.execute_lsr_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_lsr_zpgx(memory),
            MemoryMode::Absolute  => self.execute_lsr_abs(memory),
            MemoryMode::AbsoluteX => self.execute_lsr_absx(memory),
            _                     => panic!("No {:?} memory mode for LSR", mode)
        }
    }

    pub fn execute_rol(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Acc       => self.execute_rol_acc(),
            MemoryMode::ZeroPage  => self.execute_rol_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_rol_zpgx(memory),
            MemoryMode::Absolute  => self.execute_rol_abs(memory),
            MemoryMode::AbsoluteX => self.execute_rol_absx(memory),
            _                     => panic!("No {:?} memory mode for ROL", mode)
        }
    }

    pub fn execute_ror(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Acc       => self.execute_ror_acc(),
            MemoryMode::ZeroPage  => self.execute_ror_zpg(memory),
            MemoryMode::ZeroPageX => self.execute_ror_zpgx(memory),
            MemoryMode::Absolute  => self.execute_ror_abs(memory),
            MemoryMode::AbsoluteX => self.execute_ror_absx(memory),
            _                     => panic!("No {:?} memory mode for ROR", mode)
        }
    }
}

// ASL IMPL
impl CPU {
    fn execute_asl_acc(&mut self) {
        let value = self.A.0;
        self.C = value & 0b_1000_0000 != 0;
        self.N = value & 0b_0100_0000 != 0;
        self.A = Wrapping((value << 1) & 0b_1111_1110);
        self.Z = self.A.0 == 0;
        self.PC += 1;
    }

    // FIXME: PROBABLY BROKEN Z FLAG IN ALL BELOW
    fn execute_asl_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value == 0;
        self.N = memory_value & 0b_0100_0000 != 0;
        memory.write(memory_address, (memory_value << 1) & 0b_1111_1110);
        self.PC += 2;
    }

    fn execute_asl_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value == 0;
        self.N = memory_value & 0b_0100_0000 != 0;
        memory.write(memory_address.0 as usize, (memory_value << 1) & 0b_1111_1110);
        self.PC += 2;
    }

    fn execute_asl_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value == 0;
        self.N = memory_value & 0b_0100_0000 != 0;
        memory.write(memory_address as usize, (memory_value << 1) & 0b_1111_1110);
        self.PC += 3;
    }

    fn execute_asl_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value == 0;
        self.N = memory_value & 0b_0100_0000 != 0;
        memory.write(memory_address.0 as usize, (memory_value << 1) & 0b_1111_1110);
        self.PC += 3;
    }
}

// LSR IMPL
impl CPU {
    fn execute_lsr_acc(&mut self) {
        let value = self.A.0;
        self.C = value & 0b_0000_0001 != 0;
        self.Z = value & 0b_1111_1110 == 0;
        self.N = false;
        self.A = Wrapping((value >> 1) & 0b_0111_1111);
        self.PC += 1;
    }

    fn execute_lsr_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value & 0b_1111_1110 == 0;
        self.N = false;
        memory.write(memory_address, (memory_value >> 1) & 0b_0111_1111);
        self.PC += 2;
    }

    fn execute_lsr_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value & 0b_1111_1110 == 0;
        self.N = false;
        memory.write(memory_address.0 as usize, (memory_value >> 1) & 0b_0111_1111);
        self.PC += 2;
    }

    fn execute_lsr_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value & 0b_1111_1110 == 0;
        self.N = false;
        memory.write(memory_address as usize, (memory_value >> 1) & 0b_0111_1111);
        self.PC += 3;
    }

    fn execute_lsr_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = memory_value & 0b_1111_1110 == 0;
        self.N = false;
        memory.write(memory_address.0 as usize, (memory_value >> 1) & 0b_0111_1111);
        self.PC += 3;
    }
}

// ROL IMPL
impl CPU {
    fn execute_rol_acc(&mut self) {
        let value = self.A.0;
        let mut temp = value;
        temp = (temp << 1) & 0b_1111_1110;
        if self.C {temp += 0b_0000_0001};
        self.C = value & 0b_1000_0000 != 0;
        self.Z = temp == 0;
        self.N = temp & 0b_1000_0000 != 0;
        self.A = Wrapping(temp);
        self.PC += 1;
    }

    fn execute_rol_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = memory_value;
        temp = (temp << 1) & 0b_1111_1110;
        if self.C {temp += 0b_0000_0001};
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = temp == 0;
        self.N = temp & 0b_1000_0000 != 0;
        memory.write(memory_address, temp);
        self.PC += 2;
    }

    fn execute_rol_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = memory_value;
        temp = (temp << 1) & 0b_1111_1110;
        if self.C {temp += 0b_0000_0001};
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = temp == 0;
        self.N = temp & 0b_1000_0000 != 0;
        memory.write(memory_address.0 as usize, temp);
        self.PC += 2;
    }

    fn execute_rol_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = memory_value;
        temp = (temp << 1) & 0b_1111_1110;
        if self.C {temp += 0b_0000_0001};
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = temp == 0;
        self.N = temp & 0b_1000_0000 != 0;
        memory.write(memory_address as usize, temp);
        self.PC += 3;
    }

    fn execute_rol_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = memory_value;
        temp = (temp << 1) & 0b_1111_1110;
        if self.C {temp += 0b_0000_0001};
        self.C = memory_value & 0b_1000_0000 != 0;
        self.Z = temp == 0;
        self.N = temp & 0b_1000_0000 != 0;
        memory.write(memory_address.0 as usize, temp);
        self.PC += 3;
    }
}

// ROR IMPL
impl CPU {
    fn execute_ror_acc(&mut self) {
        let value = self.A.0;
        let mut temp = value;
        temp = (temp >> 1) & 0b_0111_1111;
        if self.C {temp += 0b_1000_0000};
        self.N = self.C;
        self.C = value & 0b_0000_0001 != 0;
        self.Z = temp == 0;
        self.A = Wrapping(temp);
        self.PC += 1;
    }

    fn execute_ror_zpg(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 1);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = memory_value;
        temp = (temp >> 1) & 0b_0111_1111;
        if self.C {temp += 0b_1000_0000};
        self.N = self.C;
        self.C = memory_value & 0b_0000_0001 != 0;
        self.Z = temp == 0;
        memory.write(memory_address, temp);
        self.PC += 2;
    }

    fn execute_ror_zpgx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 1) as u8);
        memory_address += self.X;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = memory_value;
        temp = (temp >> 1) & 0b_0111_1111;
        if self.C {temp += 0b_1000_0000};
        self.N = self.C;
        self.C = memory_value & 0b_0000_0001 != 0;
        self.Z = temp == 0;
        memory.write(memory_address.0 as usize, temp);
        self.PC += 2;
    }

    fn execute_ror_abs(&mut self, memory: &mut MEM) {
        let memory_address = memory.read(self.next_pc(), 2);
        let memory_value = memory.read(memory_address, 1) as u8;
        let mut temp = memory_value;
        temp = (temp >> 1) & 0b_0111_1111;
        if self.C {temp += 0b_1000_0000};
        self.N = self.C;
        self.C = memory_value & 0b_0000_0001 != 0;
        self.Z = temp == 0;
        memory.write(memory_address as usize, temp);
        self.PC += 3;
    }

    fn execute_ror_absx(&mut self, memory: &mut MEM) {
        let mut memory_address = Wrapping(memory.read(self.next_pc(), 2) as u16);
        memory_address += self.X.0 as u16;
        let memory_value = memory.read(memory_address.0 as usize, 1) as u8;
        let mut temp = memory_value;
        temp = (temp >> 1) & 0b_0111_1111;
        if self.C {temp += 0b_1000_0000};
        self.N = self.C;
        self.C = memory_value & 0b_0000_0001 != 0;
        self.Z = temp == 0;
        memory.write(memory_address.0 as usize, temp);
        self.PC += 3;
    }
}
