use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_pha(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_pha_imp(memory),
            _                     => panic!("No {:?} memory mode for PHA", mode)
        }
    }

    pub fn execute_php(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_php_imp(memory),
            _                     => panic!("No {:?} memory mode for PHP", mode)
        }
    }

    pub fn execute_pla(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_pla_imp(memory),
            _                     => panic!("No {:?} memory mode for PLA", mode)
        }
    }

    pub fn execute_plp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_plp_imp(memory),
            _                     => panic!("No {:?} memory mode for PLP", mode)
        }
    }
}

// PHA IMPL
impl CPU {
    fn execute_pha_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "PHA");
        let value = self.get_a();
        self.push_stack(value, memory);
        self.increment_pc(1);
    }
}

// PHP IMPL
impl CPU {
    fn execute_php_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "PHP");
        let mut value = self.get_s();
        value |= 0b_0011_0000;
        self.push_stack(value, memory);
        self.increment_pc(1);
    }
}

// PLA IMPL
impl CPU {
    fn execute_pla_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "PLA");
        let value = self.pull_stack(memory);
        self.store_a(value);
        self.Z = (self.get_a() & 0b_1111_1111) == 0;
        self.N = (self.get_a() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }
}

// PLP IMPL
impl CPU {
    fn execute_plp_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "PLP");
        let mut value = self.pull_stack(memory);
        let current_status = self.get_s() & 0b_0011_0000;
        value &= 0b_1100_1111;
        value |= current_status;
        self.store_s(value);
        self.increment_pc(1);
    }
}
