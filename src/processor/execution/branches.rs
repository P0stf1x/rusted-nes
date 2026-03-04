use crate::processor::instruction::Instruction;
use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    fn execute_branch(&mut self, memory: &mut MEM, caller: &str, result: bool) {
        let inst = Instruction::get_rel(&self, memory);
        let offset: i8 = inst.value.unwrap() as i8;
        inst.log(&self, caller);
        self.increment_pc(2);
        if result {
            self.offset_pc(offset);
        }
    }

    pub fn execute_bcc(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BCC", self.C == false);
    }

    pub fn execute_bcs(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BCS", self.C == true);
    }

    pub fn execute_beq(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BEQ", self.Z == true);
    }

    pub fn execute_bne(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BNE", self.Z == false);
    }

    pub fn execute_bmi(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BMI", self.N == true);
    }

    pub fn execute_bpl(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BPL", self.N == false);
    }

    pub fn execute_bvc(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BVC", self.V == false);
    }

    pub fn execute_bvs(&mut self, memory: &mut MEM) {
        self.execute_branch(memory, "BVS", self.V == true);
    }
}
