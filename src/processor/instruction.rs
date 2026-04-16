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
                Absolute => { Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X}", self.memory_address.unwrap())); return; },
                _ => (),
            }
        }
        match self.mode {
            Implicit => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name}")),
            Acc => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} A")),
            Immediate => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} #${:02X}", self.value.unwrap())),
            ZeroPage => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X} = {:02X}", self.memory_address.unwrap(), self.value.unwrap())),
            ZeroPageX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X},X @ {:02X} = {:02X}", (Wrapping::<u8>(self.memory_address.unwrap() as u8) - Wrapping::<u8>(cpu.get_x())).0, self.memory_address.unwrap(), self.value.unwrap())),
            ZeroPageY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:02X},Y @ {:02X} = {:02X}", (Wrapping::<u8>(self.memory_address.unwrap() as u8) - Wrapping::<u8>(cpu.get_y())).0, self.memory_address.unwrap(), self.value.unwrap())),
            Relative => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X}", self.memory_address.unwrap())),
            Absolute => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X} = {:02X}", self.memory_address.unwrap(), self.value.unwrap())),
            AbsoluteX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X},X @ {:04X} = {:02X}", (Wrapping::<u16>(self.memory_address.unwrap()) - Wrapping::<u16>(cpu.get_x() as u16)).0, self.memory_address.unwrap(), self.value.unwrap())),
            AbsoluteY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} ${:04X},Y @ {:04X} = {:02X}", (Wrapping::<u16>(self.memory_address.unwrap()) - Wrapping::<u16>(cpu.get_y() as u16)).0, self.memory_address.unwrap(), self.value.unwrap())),
            Indirect => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:04X}) = {:04X}", combine_operands(self.operand1.unwrap(), self.operand2.unwrap()), self.memory_address.unwrap())),
            IndirectX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X},X) @ {:02X} = {:04X} = {:02X}", self.memory_indirect_address.unwrap(), (Wrapping::<u8>(self.memory_indirect_address.unwrap()) + Wrapping::<u8>(cpu.get_x())).0, self.memory_address.unwrap(), self.value.unwrap())),
            IndirectY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", self.memory_indirect_address.unwrap(), (Wrapping::<u16>(self.memory_address.unwrap()) - Wrapping::<u16>(cpu.get_y() as u16)).0, self.memory_address.unwrap(), self.value.unwrap())),
        }
    }

    pub fn get_base_execution_time(instruction: u8) -> usize {
        // should also add +1 on page cross and +1 if branch taken
        match instruction {
            0x00 => 7,
            0x01 => 6,
            0x05 => 3,
            0x06 => 5,
            0x08 => 3,
            0x09 => 2,
            0x0A => 2,
            0x0D => 4,
            0x0E => 6,
            0x10 => 2,
            0x11 => 5,
            0x15 => 4,
            0x16 => 6,
            0x18 => 2,
            0x19 => 4,
            0x1D => 4,
            0x1E => 7,
            0x20 => 6,
            0x21 => 6,
            0x24 => 3,
            0x25 => 3,
            0x26 => 5,
            0x28 => 4,
            0x29 => 2,
            0x2A => 2,
            0x2C => 4,
            0x2D => 4,
            0x2E => 6,
            0x30 => 2,
            0x31 => 5,
            0x35 => 4,
            0x36 => 6,
            0x38 => 2,
            0x39 => 4,
            0x3D => 4,
            0x3E => 7,
            0x40 => 6,
            0x41 => 6,
            0x45 => 3,
            0x46 => 5,
            0x48 => 3,
            0x49 => 2,
            0x4A => 2,
            0x4C => 3,
            0x4D => 4,
            0x4E => 6,
            0x50 => 2,
            0x51 => 5,
            0x55 => 4,
            0x56 => 6,
            0x58 => 2,
            0x59 => 4,
            0x5D => 4,
            0x5E => 7,
            0x60 => 6,
            0x61 => 6,
            0x65 => 3,
            0x66 => 5,
            0x68 => 4,
            0x69 => 2,
            0x6A => 2,
            0x6C => 5,
            0x6D => 4,
            0x6E => 6,
            0x70 => 2,
            0x71 => 5,
            0x75 => 4,
            0x76 => 6,
            0x78 => 2,
            0x79 => 4,
            0x7D => 4,
            0x7E => 7,
            0x81 => 6,
            0x84 => 3,
            0x85 => 3,
            0x86 => 3,
            0x88 => 2,
            0x8A => 2,
            0x8C => 4,
            0x8D => 4,
            0x8E => 4,
            0x90 => 2,
            0x91 => 6,
            0x94 => 4,
            0x95 => 4,
            0x96 => 4,
            0x98 => 2,
            0x99 => 5,
            0x9A => 2,
            0x9D => 5,
            0xA0 => 2,
            0xA1 => 6,
            0xA2 => 2,
            0xA4 => 3,
            0xA5 => 3,
            0xA6 => 3,
            0xA8 => 2,
            0xA9 => 2,
            0xAA => 2,
            0xAC => 4,
            0xAD => 4,
            0xAE => 4,
            0xB0 => 2,
            0xB1 => 5,
            0xB4 => 4,
            0xB5 => 4,
            0xB6 => 4,
            0xB8 => 2,
            0xB9 => 4,
            0xBA => 2,
            0xBC => 4,
            0xBD => 4,
            0xBE => 4,
            0xC0 => 2,
            0xC1 => 6,
            0xC4 => 3,
            0xC5 => 3,
            0xC6 => 5,
            0xC8 => 2,
            0xC9 => 2,
            0xCA => 2,
            0xCC => 4,
            0xCD => 4,
            0xCE => 6,
            0xD0 => 2,
            0xD1 => 5,
            0xD5 => 4,
            0xD6 => 6,
            0xD8 => 2,
            0xD9 => 4,
            0xDD => 4,
            0xDE => 7,
            0xE0 => 2,
            0xE1 => 6,
            0xE4 => 3,
            0xE5 => 3,
            0xE6 => 5,
            0xE8 => 2,
            0xE9 => 2,
            0xEA => 2,
            0xEC => 4,
            0xED => 4,
            0xEE => 6,
            0xF0 => 2,
            0xF1 => 5,
            0xF5 => 4,
            0xF6 => 6,
            0xF8 => 2,
            0xF9 => 4,
            0xFD => 4,
            0xFE => 7,
            _ => panic!(),
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
        let previous_page = memory_address/256;
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_x() as u16)).0;
        let new_page = offsetted_memory_address/256;
        if new_page != previous_page { cpu.add_sleep_cycles(1); }
        let value = memory.read_no_hook(offsetted_memory_address as usize, 1) as u8;
        return Self { mode: AbsoluteX, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(offsetted_memory_address), memory_indirect_address: None }
    }

    pub fn get_absy(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let previous_page = memory_address/256;
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0;
        let new_page = offsetted_memory_address/256;
        if new_page != previous_page { cpu.add_sleep_cycles(1); }
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
        let previous_page = memory_address/256;
        let offsetted_memory_address = (Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0;
        let new_page = offsetted_memory_address/256;
        if new_page != previous_page { cpu.add_sleep_cycles(1); }
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
