use crate::processor::*;
// use crate::MEM;

impl CPU {
    pub fn execute_tax(&mut self) {
        self.X = self.A;
        self.Z = (self.X.0 & 0b_1111_1111) == 0;
        self.N = (self.X.0 & 0b_1000_0000) != 0;
        self.PC += 1;
    }

    pub fn execute_tay(&mut self) {
        self.Y = self.A;
        self.Z = (self.Y.0 & 0b_1111_1111) == 0;
        self.N = (self.Y.0 & 0b_1000_0000) != 0;
        self.PC += 1;
    }
    
    pub fn execute_txa(&mut self) {
        self.A = self.X;
        self.Z = (self.A.0 & 0b_1111_1111) == 0;
        self.N = (self.A.0 & 0b_1000_0000) != 0;
        self.PC += 1;
    }
    
        pub fn execute_tya(&mut self) {
            self.A = self.Y;
            self.Z = (self.A.0 & 0b_1111_1111) == 0;
            self.N = (self.A.0 & 0b_1000_0000) != 0;
            self.PC += 1;
        }

    pub fn execute_tsx(&mut self) {
        self.X = self.S;
        self.Z = (self.X.0 & 0b_1111_1111) == 0;
        self.N = (self.X.0 & 0b_1000_0000) != 0;
        self.PC += 1;
    }

    pub fn execute_txs(&mut self) {
        self.S = self.X;
        self.PC += 1;
    }
}
