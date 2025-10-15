use crate::processor::*;
use crate::memory::MEM;
use std::time::Duration;
use std::thread;

mod load;
mod store;
mod transfers;
mod stack;
mod logic;
mod arithmetic;
mod increment;
mod decrement;
mod shifts;
mod jumps;
mod branches;

impl CPU {
    pub fn sleep(&mut self, ticks: u32) {
        // FIXME: right now it sleeps n ticks + time it took to execute instruction
        let sleep_timer = Duration::from_nanos((self.settings.clock_delta / self.settings.emulation_speed * ticks as f64) as u64);
        thread::sleep(sleep_timer);
    }

    pub fn fetch_mem_address(&mut self, address: u16, memory: &mut MEM) -> u16 {
        let least_significant_byte = memory.data[address as usize];
        let most_significant_byte = memory.data[address as usize + 1];
        return ((most_significant_byte as u16) << 8) + least_significant_byte as u16;
    }

    pub fn execute(&mut self, memory: &mut MEM) -> Result<(), ()> {
        let pc_data = memory.read(self.PC.0 as usize, 1) as u8;
        self.executed_opcodes += 1;
        let operation = self.from(pc_data);
        use Opcodes::*;
        match operation {
            Err(_) => panic!("Unexpected instruction {pc_data:#04X} at {:#06X}", self.PC.0),
            
            Ok(operation) => {
                let status = self.store_status();

                // TODO: use logger instead of just printing
                // 
                println!("{:04X}  {:02X}                                        A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", self.PC.0, pc_data, self.A.0, self.X.0, self.Y.0, status, self.S.0);
                
                #[allow(unreachable_patterns)] // In case of adding new opcodes to the enum
                match operation {

                    // LOAD/STORE
                    LDA(memory_mode) => {Ok(self.execute_lda(memory_mode, memory))},
                    LDX(memory_mode) => {Ok(self.execute_ldx(memory_mode, memory))},
                    LDY(memory_mode) => {Ok(self.execute_ldy(memory_mode, memory))},
                    STA(memory_mode) => {Ok(self.execute_sta(memory_mode, memory))},
                    STX(memory_mode) => {Ok(self.execute_stx(memory_mode, memory))},
                    STY(memory_mode) => {Ok(self.execute_sty(memory_mode, memory))},
                    
                    // TRANSFERS
                    TAX(_memory_mode) => {Ok(self.execute_tax(memory))},
                    TAY(_memory_mode) => {Ok(self.execute_tay(memory))},
                    TXA(_memory_mode) => {Ok(self.execute_txa(memory))},
                    TYA(_memory_mode) => {Ok(self.execute_tya(memory))},
                    TSX(_memory_mode) => {Ok(self.execute_tsx(memory))},
                    TXS(_memory_mode) => {Ok(self.execute_txs(memory))},
                    
                    // STACK
                    PHA(memory_mode) => {Ok(self.execute_pha(memory_mode, memory))},
                    PHP(memory_mode) => {Ok(self.execute_php(memory_mode, memory))},
                    PLA(memory_mode) => {Ok(self.execute_pla(memory_mode, memory))},
                    PLP(memory_mode) => {Ok(self.execute_plp(memory_mode, memory))},
                    
                    // LOGIC
                    AND(memory_mode) => {Ok(self.execute_and(memory_mode, memory))},
                    EOR(memory_mode) => {Ok(self.execute_eor(memory_mode, memory))},
                    ORA(memory_mode) => {Ok(self.execute_ora(memory_mode, memory))},
                    BIT(memory_mode) => {Ok(self.execute_bit(memory_mode, memory))},

                    // ARITHMETIC
                    ADC(memory_mode) => {Ok(self.execute_adc(memory_mode, memory))},
                    SBC(memory_mode) => {Ok(self.execute_sbc(memory_mode, memory))},
                    CMP(memory_mode) => {Ok(self.execute_cmp(memory_mode, memory))},
                    CPX(memory_mode) => {Ok(self.execute_cpx(memory_mode, memory))},
                    CPY(memory_mode) => {Ok(self.execute_cpy(memory_mode, memory))},
                    
                    // INC/DEC
                    INC(memory_mode) => {Ok(self.execute_inc(memory_mode, memory))},
                    INX(memory_mode) => {Ok(self.execute_inx(memory_mode))},
                    INY(memory_mode) => {Ok(self.execute_iny(memory_mode))},
                    DEC(memory_mode) => {Ok(self.execute_dec(memory_mode, memory))},
                    DEX(memory_mode) => {Ok(self.execute_dex(memory_mode))},
                    DEY(memory_mode) => {Ok(self.execute_dey(memory_mode))},
                    
                    // SHIFTS
                    ASL(memory_mode) => {Ok(self.execute_asl(memory_mode, memory))},
                    LSR(memory_mode) => {Ok(self.execute_lsr(memory_mode, memory))},
                    ROL(memory_mode) => {Ok(self.execute_rol(memory_mode, memory))},
                    ROR(memory_mode) => {Ok(self.execute_ror(memory_mode, memory))},
                    
                    // JUMPS
                    JMP(memory_mode) => {Ok(self.execute_jmp(memory_mode, memory))},
                    JSR(memory_mode) => {Ok(self.execute_jsr(memory_mode, memory))},
                    RTS(memory_mode) => {Ok(self.execute_rts(memory_mode, memory))},

                    // BRANCHES
                    BCS(_memory_mode) => {Ok(self.execute_bcs(memory))},
                    BCC(_memory_mode) => {Ok(self.execute_bcc(memory))},
                    BEQ(_memory_mode) => {Ok(self.execute_beq(memory))},
                    BNE(_memory_mode) => {Ok(self.execute_bne(memory))},
                    BMI(_memory_mode) => {Ok(self.execute_bmi(memory))},
                    BPL(_memory_mode) => {Ok(self.execute_bpl(memory))},
                    BVS(_memory_mode) => {Ok(self.execute_bvs(memory))},
                    BVC(_memory_mode) => {Ok(self.execute_bvc(memory))},

                    // STATUS
                    SEC(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("SEC"));
                        self.C = true;
                        Ok(self.PC += 1)
                    },
                    CLC(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("CLC"));
                        self.C = false;
                        Ok(self.PC += 1)
                    },
                    SEI(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("SEI"));
                        self.I = true;
                        Ok(self.PC += 1)
                    },
                    CLI(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("CLI"));
                        self.I = false;
                        Ok(self.PC += 1)
                    },
                    SED(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("SED"));
                        self.D = true;
                        Ok(self.PC += 1)
                    },
                    CLD(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("CLD"));
                        self.D = false;
                        Ok(self.PC += 1)
                    },
                    CLV(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("CLV"));
                        self.V = false;
                        Ok(self.PC += 1)
                    },
                    
                    // SYSTEM
                    BRK(_memory_mode) => {
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("BRK"));
                        Ok(self.irq_brk(memory))
                    },
                    NOP(_memory_mode) => Ok({
                        Logger::log_cpu_instruction(&self, pc_data, None, None, format!("NOP"));
                        self.increment_pc(1);
                    }),
                    RTI(_memory_mode) => Ok({
                        let status = self.pull_stack(memory);
                        self.load_status(status);
                        let pcl = self.pull_stack(memory) as u16;
                        let pch = self.pull_stack(memory) as u16;
                        let pc = (pch << 8) + pcl;
                        self.PC = Wrapping(pc);
                    }),

                    opcode => {panic!("Unexpected instruction {opcode:?}")},
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::memory::MEMORY_SIZE;
    use super::*;

    #[test]
    fn test_fetch_mem_address() {
        let mut test_cpu: CPU = CPU::new();
        let mut memory: MEM = MEM::new(MEMORY_SIZE);
        memory.data[0..2].copy_from_slice(&[0xCD, 0xAB]);

        assert_eq!(test_cpu.PC.0, 0x0000);
        
        let fetched_address = test_cpu.fetch_mem_address(0x0000, &mut memory);

        assert_eq!(fetched_address, 0xABCD);
    }
}
