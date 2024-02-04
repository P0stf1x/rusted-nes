use crate::processor::*;
use crate::memory::MEM;

impl CPU {
    fn execute_branch(&mut self, memory: &mut MEM) {
        let offset: i8 = memory.read(self.next_pc(), 1) as i8;
        self.PC += 2;
        self.PC += Wrapping(offset as u16);
    }

    pub fn execute_bcc(&mut self, memory: &mut MEM) {
        if self.C == false {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bcs(&mut self, memory: &mut MEM) {
        if self.C == true {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_beq(&mut self, memory: &mut MEM) {
        if self.Z == true {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bne(&mut self, memory: &mut MEM) {
        if self.Z == false {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bmi(&mut self, memory: &mut MEM) {
        if self.N == true {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bpl(&mut self, memory: &mut MEM) {
        if self.N == false {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bvc(&mut self, memory: &mut MEM) {
        if self.V == false {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }

    pub fn execute_bvs(&mut self, memory: &mut MEM) {
        if self.V == true {self.execute_branch(memory)}
        else {self.PC += Wrapping(2)};
    }
}