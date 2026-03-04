use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_rts(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Implicit  => self.execute_rts_imp(memory),
            _                     => panic!("No {:?} memory mode for RTS", mode)
        }
    }
}

macro_rules! rts {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        // let memory_address = $instruction.memory_address.unwrap() as u16;
        let PCL: u16 = $cpu.pull_stack($memory) as u16;
        let PCH: u16 = ($cpu.pull_stack($memory) as u16) << 8;
        $cpu.store_pc(PCH + PCL + 1);
    }}
}

// RTS IMPL
impl CPU {
    #[allow(non_snake_case)]
    fn execute_rts_imp(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_imp(&self, memory);
        inst.log(&self, "RTS");
        rts!(self, inst, memory);
    }
}

#[cfg(test)]
mod rts_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_rts_imp(low_byte in 0x00u16..=0xFF, high_byte in 0x00u16..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            let return_address = (high_byte << 8) + low_byte;
            let new_low_byte = (return_address - 1) as u8;
            let new_high_byte = ((return_address - 1) >> 8) as u8;
            memory.write_bulk(0x01FE, vec![new_low_byte, new_high_byte]);
            test_cpu.store_s(0xFD);

            test_cpu.execute_rts_imp(&mut memory);

            prop_assert_eq!(test_cpu.PC.0, return_address);
        }
    }
}
