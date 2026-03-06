use std::num::Wrapping;

use crate::memory::{combine_operands, MEM};
use crate::logging::Logger;

use super::{MemoryMode::{self, *}, CPU};

pub struct Instruction {
    pub mode: MemoryMode,
    pub instruction: u8,
    pub operand1: Option<u8>,
    pub operand2: Option<u8>,
    pub value: Option<u8>,
    pub memory_address: Option<u16>,
    pub memory_indirect_address: Option<u8>,
}

impl Instruction {
    pub fn log(&self, cpu: &CPU, instruction_name: &str) {
        if instruction_name == "JMP" || instruction_name == "JSR" {
            match self.mode {
                Absolute => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X}", self.memory_address.unwrap())),
                _ => panic!(),
            }
            return;
        }
        match self.mode {
            Implicit => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name}")),
            Acc => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} A")),
            Immediate => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} #${:02X}", self.value.unwrap())),
            ZeroPage => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X} = {:02X}", self.memory_address.unwrap(), self.value.unwrap())),
            ZeroPageX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X},X @ {:02X} = {:02X}", self.memory_address.unwrap(), (Wrapping::<u8>(self.memory_address.unwrap() as u8) + Wrapping::<u8>(cpu.get_x())).0, self.value.unwrap())),
            ZeroPageY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X},Y @ {:02X} = {:02X}", self.memory_address.unwrap(), (Wrapping::<u8>(self.memory_address.unwrap() as u8) + Wrapping::<u8>(cpu.get_y())).0, self.value.unwrap())),
            Relative => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X}", self.memory_address.unwrap())),
            Absolute => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X} = {:02X}", self.memory_address.unwrap(), self.value.unwrap())),
            AbsoluteX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X},X @ {:04X} = {:02X}", self.memory_address.unwrap(), (Wrapping::<u16>(self.memory_address.unwrap()) + Wrapping::<u16>(cpu.get_x() as u16)).0, self.value.unwrap())),
            AbsoluteY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X},Y @ {:04X} = {:02X}", self.memory_address.unwrap(), (Wrapping::<u16>(self.memory_address.unwrap()) + Wrapping::<u16>(cpu.get_y() as u16)).0, self.value.unwrap())),
            Indirect => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:04X}) = {:04X}", combine_operands(self.operand1.unwrap(), self.operand2.unwrap()), self.memory_address.unwrap())),
            IndirectX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X},X) @ {:02X} = {:04X} = {:02X}", self.memory_indirect_address.unwrap(), (Wrapping::<u8>(self.memory_indirect_address.unwrap()) + Wrapping::<u8>(cpu.get_x())).0, self.memory_address.unwrap(), self.value.unwrap())),
            IndirectY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", self.memory_indirect_address.unwrap(), (Wrapping::<u16>(self.memory_address.unwrap()) - Wrapping::<u16>(cpu.get_y() as u16)).0, self.memory_address.unwrap(), self.value.unwrap())),
        }
    }

    pub fn count_cycles(instruction: u8) -> u64 {
        let mut cycle_count: u64 = match Self::get_memory_mode(instruction) {
            Implicit  => 2,
            Acc       => 2,
            Immediate => 2,
            ZeroPage  => 3,
            ZeroPageX => 4,
            ZeroPageY => 4,
            Relative  => 2, // Only in branch instructions // FIXME: +1 if branch taken // FIXME: +1 on page cross
            Absolute  => 4,
            AbsoluteX => 4, // FIXME: +1 on page cross
            AbsoluteY => 4, // FIXME: +1 on page cross
            Indirect  => 5,
            IndirectX => 6,
            IndirectY => 5, // FIXME: +1 on page cross
        };
        cycle_count += Self::is_rmw(instruction);
        return cycle_count;
    }

    fn is_rmw(instruction: u8) -> u64 { // read/modify/write instruction
        match instruction {
            // ASL
            0x06 => 2, 0x16 => 2, 0x0E => 2, 0x1E => 2,
            // LSR
            0x46 => 2, 0x56 => 2, 0x4E => 2, 0x5E => 2,
            // ROL
            0x26 => 2, 0x36 => 2, 0x2E => 2, 0x3E => 2,
            // ROR
            0x66 => 2, 0x76 => 2, 0x6E => 2, 0x7E => 2,
            // INC
            0xE6 => 2, 0xF6 => 2, 0xEE => 2, 0xFE => 2,
            // DEC
            0xC6 => 2, 0xD6 => 2, 0xCE => 2, 0xDE => 2,
            // Else
            _ => 0,
        }
    }

    fn get_memory_mode(instruction: u8) -> MemoryMode {
        // let instruction = cpu.get_instr_without_hook(memory);
        match instruction { // Non non standard instructions
            0x00 => return MemoryMode::Implicit,
            0x20 => return MemoryMode::Absolute,
            0x40 => return MemoryMode::Implicit,
            0x60 => return MemoryMode::Implicit,

            0x08 => return MemoryMode::Implicit,
            0x28 => return MemoryMode::Implicit,
            0x48 => return MemoryMode::Implicit,
            0x68 => return MemoryMode::Implicit,
            0x88 => return MemoryMode::Implicit,
            0xA8 => return MemoryMode::Implicit,
            0xC8 => return MemoryMode::Implicit,
            0xE8 => return MemoryMode::Implicit,

            0x18 => return MemoryMode::Implicit,
            0x38 => return MemoryMode::Implicit,
            0x58 => return MemoryMode::Implicit,
            0x78 => return MemoryMode::Implicit,
            0x98 => return MemoryMode::Implicit,
            0xB8 => return MemoryMode::Implicit,
            0xD8 => return MemoryMode::Implicit,
            0xF8 => return MemoryMode::Implicit,

            0x8A => return MemoryMode::Implicit,
            0x9A => return MemoryMode::Implicit,
            0xAA => return MemoryMode::Implicit,
            0xBA => return MemoryMode::Implicit,
            0xCA => return MemoryMode::Implicit,
            0xEA => return MemoryMode::Implicit,
            _ => ()
        };
        match instruction & 0b_0000_0011 {
            0b01 => {
                match (instruction >> 2) & 0b_0000_0111 {
                    0b000 => MemoryMode::IndirectX,
                    0b001 => MemoryMode::ZeroPage,
                    0b010 => MemoryMode::Immediate,
                    0b011 => MemoryMode::Absolute,
                    0b100 => MemoryMode::IndirectY,
                    0b101 => MemoryMode::ZeroPageX,
                    0b110 => MemoryMode::AbsoluteX,
                    0b111 => MemoryMode::AbsoluteY,
                    _ => panic!("index out of bounds"),
                }
            },
            0b10 => {
                match instruction { // Non standard instructions
                    0x96 => return MemoryMode::ZeroPageY,
                    0xB6 => return MemoryMode::ZeroPageY,
                    0xBE => return MemoryMode::AbsoluteY,
                    _ => ()
                };
                match (instruction >> 2) & 0b_0000_0111 {
                    0b000 => MemoryMode::Immediate,
                    0b001 => MemoryMode::ZeroPage,
                    0b010 => MemoryMode::Acc,
                    0b011 => MemoryMode::Absolute,
                    0b100 => panic!(),
                    0b101 => MemoryMode::ZeroPageX,
                    0b110 => panic!(),
                    0b111 => MemoryMode::AbsoluteX,
                    _ => panic!("index out of bounds"),
                }
            }
            0b00 => {
                match instruction { // Non standard instructions
                    0x6C => return MemoryMode::Indirect,
                    _ => ()
                };
                match (instruction >> 2) & 0b_0000_0111 {
                    0b000 => MemoryMode::Immediate,
                    0b001 => MemoryMode::ZeroPage,
                    0b010 => panic!(),
                    0b011 => MemoryMode::Absolute,
                    0b100 => MemoryMode::Relative, // all the branch instructions
                    0b101 => MemoryMode::ZeroPageX,
                    0b110 => panic!(),
                    0b111 => MemoryMode::AbsoluteX,
                    _ => panic!("index out of bounds"),
                }
            }
            0b11 => panic!("Instructions Group 4 unsupported"),
            _ => panic!("index out of bounds"),
        }
    }

    pub fn get_imp(cpu: &CPU, memory: &mut MEM) -> Self {
        let instruction = cpu.get_instr(memory);
        return Self { mode: Implicit, instruction, operand1: None, operand2: None, value: None, memory_address: None, memory_indirect_address: None }
    }

    pub fn get_acc(cpu: &CPU, memory: &mut MEM) -> Self {
        let instruction = cpu.get_instr(memory);
        let value = cpu.get_a();
        return Self { mode: Acc, instruction, operand1: None, operand2: None, value: Some(value), memory_address: None, memory_indirect_address: None }
    }

    pub fn get_imm(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, value) = cpu.get_instr_and_operand(memory);
        return Self { mode: Immediate, instruction, operand1: Some(value), operand2: None, value: Some(value), memory_address: None, memory_indirect_address: None }
    }

    pub fn get_zpg(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_address) = cpu.get_instr_and_operand(memory);
        let value = memory.read(memory_address as usize, 1) as u8;
        return Self { mode: ZeroPage, instruction, operand1: Some(memory_address), operand2: None, value: Some(value), memory_address: Some(memory_address as u16), memory_indirect_address: None }
    }

    pub fn get_zpgx(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_address) = cpu.get_instr_and_operand(memory);
        let offsetted_memory_address = (Wrapping::<u8>(memory_address) + Wrapping::<u8>(cpu.get_x())).0;
        let value = memory.read(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: ZeroPageX, instruction, operand1: Some(memory_address), operand2: None, value: Some(value), memory_address: Some(offsetted_memory_address as u16), memory_indirect_address: None }
    }

    pub fn get_zpgy(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_address) = cpu.get_instr_and_operand(memory);
        let offsetted_memory_address = (Wrapping::<u8>(memory_address) + Wrapping::<u8>(cpu.get_y())).0;
        let value = memory.read(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: ZeroPageY, instruction, operand1: Some(memory_address), operand2: None, value: Some(value), memory_address: Some(offsetted_memory_address as u16), memory_indirect_address: None }
    }

    pub fn get_rel(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, offset) = cpu.get_instr_and_operand(memory);
        let absolute_address = (Wrapping::<u16>(cpu.get_pc()) + Wrapping::<u16>(2) + Wrapping::<u16>(offset as i8 as u16)).0;
        return Self { mode: Relative, instruction, operand1: Some(offset), operand2: None, value: Some(offset), memory_address: Some(absolute_address), memory_indirect_address: None }
    }

    pub fn get_abs(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let value = memory.read_no_hook(memory_address as usize, 1) as u8;
        return Self { mode: Absolute, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(memory_address), memory_indirect_address: None }
    }

    pub fn get_absx(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_x() as u16)).0;
        let value = memory.read_no_hook(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: AbsoluteX, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(offsetted_memory_address), memory_indirect_address: None }
    }

    pub fn get_absy(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0;
        let value = memory.read_no_hook(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: AbsoluteY, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(offsetted_memory_address), memory_indirect_address: None }
    }

    pub fn get_indirect(cpu: &CPU, memory: &mut MEM) -> Self {
        // indirect is only used by JMP and there's a bug in it
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_indirect_address = combine_operands(operand1, operand2);
        // let memory_address = memory.read((memory_indirect_address) as usize, 2) as u16;
        let memory_address = match operand1 {
            0x00..=0xFE => {
                memory.read(memory_indirect_address as usize, 2) as u16
            },
            0xFF => {
                (memory.read(memory_indirect_address as usize, 1) +
                (memory.read((memory_indirect_address - 0xFF) as usize, 1) << 8)) as u16
            },
        };
        return Self { mode: Indirect, instruction, operand1: Some(operand1), operand2: Some(operand2), value: None, memory_address: Some(memory_address), memory_indirect_address: None }
    }

    pub fn get_indirect_x(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_indirect_address) = cpu.get_instr_and_operand(memory);
        let memory_low_byte_address = memory.read((Wrapping::<u8>(memory_indirect_address) + Wrapping::<u8>(cpu.get_x())).0 as usize, 1) as u16;
        let memory_high_byte_address = memory.read((Wrapping::<u8>(memory_indirect_address) + Wrapping::<u8>(cpu.get_x()) + Wrapping::<u8>(1)).0 as usize, 1) as u16;
        let memory_address = (memory_high_byte_address << 8) + memory_low_byte_address;
        let value = memory.read_no_hook(memory_address as usize, 1) as u8;
        return Self { mode: IndirectX, instruction, operand1: Some(memory_indirect_address), operand2: None, value: Some(value), memory_address: Some(memory_address), memory_indirect_address: Some(memory_indirect_address) }
    }

    pub fn get_indirect_y(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_indirect_address) = cpu.get_instr_and_operand(memory);
        let memory_low_byte_address = memory.read(memory_indirect_address as usize, 1) as u16;
        let memory_high_byte_address = memory.read((Wrapping::<u8>(memory_indirect_address) + Wrapping::<u8>(1)).0 as usize, 1) as u16;
        let memory_address = ((memory_high_byte_address << 8) + memory_low_byte_address) as u16;
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0;
        let value = memory.read_no_hook(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: IndirectY, instruction, operand1: Some(memory_indirect_address), operand2: None, value: Some(value), memory_address: Some(offsetted_memory_address), memory_indirect_address: Some(memory_indirect_address) }
    }

    pub fn read(&self, memory: &mut MEM) -> u8 {
        match self.memory_address {
            Some(addr) => memory.read(addr as usize, 1) as u8,
            None => self.value.unwrap(), // this is ugly, but this is a quick fix for invalid read hooks on write operations
        }
    }
}
