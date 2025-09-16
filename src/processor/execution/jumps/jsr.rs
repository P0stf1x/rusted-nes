use crate::memory::MEM;
use crate::processor::MemoryMode;
use crate::processor::instruction::Instruction;
use crate::CPU;

impl CPU {
    pub fn execute_jsr(&mut self, mode: MemoryMode, memory: &mut MEM) {
        match mode {
            MemoryMode::Absolute  => self.execute_jsr_abs(memory),
            _                     => panic!("No {:?} memory mode for JSR", mode)
        }
    }
}

macro_rules! jsr {
    ($cpu:ident, $instruction:ident, $memory:ident) => {{
        let memory_address = $instruction.memory_address.unwrap() as u16;
        $cpu.increment_pc(2);
        let PCH: u8 = ($cpu.get_pc() >> 8) as u8;
        let PCL: u8 = $cpu.get_pc() as u8;
        $cpu.push_stack(PCH, $memory);
        $cpu.push_stack(PCL, $memory);
        $cpu.store_pc(memory_address);
    }}
}

// JSR IMPL
impl CPU {
    #[allow(non_snake_case)]
    fn execute_jsr_abs(&mut self, memory: &mut MEM) {
        let inst = Instruction::get_abs(&self, memory);
        inst.log(&self, "JSR");
        jsr!(self, inst, memory);
    }
}

#[cfg(test)]
mod jsr_tests {
    use crate::memory::MEMORY_SIZE;
    use super::*;
    static JSR_ABS: u8 = 0x6C;

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_jsr_abs_jump_address(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF) {
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            memory.write_bulk(0x0000, vec![JSR_ABS, low_byte, high_byte]);

            assert_eq!(test_cpu.PC.0, 0x0000);

            test_cpu.execute_jsr_abs(&mut memory);

            prop_assert_eq!(test_cpu.PC.0, ((high_byte as u16) << 8) + low_byte as u16);
        }

        #[test]
        fn test_jsr_abs_return_address(low_byte in 0x00u8..=0xFF, high_byte in 0x00u8..=0xFF) {
            prop_assume!(high_byte != 0xFFu8 || low_byte < 0xFE); // so we don't write outside the memory
            let mut test_cpu: CPU = CPU::new();
            let mut memory: MEM = MEM::new(MEMORY_SIZE);
            test_cpu.store_s(0xFF);
            memory.write_bulk((((high_byte as u16) << 8)+(low_byte as u16)) as usize, vec![JSR_ABS, low_byte, high_byte]);

            test_cpu.store_pc(((high_byte as u16) << 8)+(low_byte as u16));

            test_cpu.execute_jsr_abs(&mut memory);

            let should_return_to = ((((high_byte as u16) << 8)+(low_byte as u16)) as u16) + 2; // return address is +2 from jsr instruction
            prop_assert_eq!(memory.read(0x01FE, 2), should_return_to as usize);
        }
    }
}
