use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    pub fn execute_pha(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_pha_imp(memory),
            _                     => panic!()
        }
    }

    pub fn execute_php(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_php_imp(memory),
            _                     => panic!()
        }
    }

    pub fn execute_pla(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_pla_imp(memory),
            _                     => panic!()
        }
    }

    pub fn execute_plp(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_plp_imp(memory),
            _                     => panic!()
        }
    }
}

// PHA IMPL
impl CPU {
    fn execute_pha_imp(&mut self, memory: &mut MEM) {
        self.push_stack(self.A.0, memory);
        self.PC += 1;
    }
}

// PHP IMPL
impl CPU {
    fn execute_php_imp(&mut self, memory: &mut MEM) {
        let b_temp = self.B;
        self.B = true;
        let status: u8 = self.store_status();
        self.B = b_temp;
        self.push_stack(status, memory);
        self.PC += 1;
    }
}

// PLA IMPL
impl CPU {
    fn execute_pla_imp(&mut self, memory: &mut MEM) {
        self.A = Wrapping(self.pull_stack(memory));
        self.Z = (self.A.0 & 0b_1111_1111) == 0;
        self.N = (self.A.0 & 0b_1000_0000) != 0;
        self.PC += 1;
    }
}

// PLP IMPL
impl CPU {
    fn execute_plp_imp(&mut self, memory: &mut MEM) {
        let status: u8 = self.pull_stack(memory);
        self.load_status(status);
        self.PC += 1;
    }
}

