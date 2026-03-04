use crate::processor::instruction::Instruction;
use crate::MEM;
use crate::CPU;

impl CPU {
    pub fn execute_tax(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TAX");
        let value = self.get_a();
        self.store_x(value);
        self.Z = (self.get_x() & 0b_1111_1111) == 0;
        self.N = (self.get_x() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }

    pub fn execute_tay(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TAY");
        let value = self.get_a();
        self.store_y(value);
        self.Z = (self.get_y() & 0b_1111_1111) == 0;
        self.N = (self.get_y() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }
    
    pub fn execute_txa(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TXA");
        let value = self.get_x();
        self.store_a(value);
        self.Z = (self.get_a() & 0b_1111_1111) == 0;
        self.N = (self.get_a() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }
    
    pub fn execute_tya(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TYA");
        let value = self.get_y();
        self.store_a(value);
        self.Z = (self.get_a() & 0b_1111_1111) == 0;
        self.N = (self.get_a() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }

    pub fn execute_tsx(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TSX");
        let value = self.get_s();
        self.store_x(value);
        self.Z = (self.get_x() & 0b_1111_1111) == 0;
        self.N = (self.get_x() & 0b_1000_0000) != 0;
        self.increment_pc(1);
    }

    pub fn execute_txs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "TXS");
        let value = self.get_x();
        self.store_s(value);
        self.increment_pc(1);
    }
}
