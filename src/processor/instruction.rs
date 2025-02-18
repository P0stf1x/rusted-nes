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
            Indirect => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:04X}) = {:04X}", self.memory_indirect_address.unwrap(), self.memory_address.unwrap())),
            IndirectX => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X},X) @ {:02X} = {:04X} = {:02X}", self.memory_indirect_address.unwrap(), (Wrapping::<u8>(self.memory_indirect_address.unwrap()) + Wrapping::<u8>(cpu.get_x())).0, self.memory_address.unwrap(), self.value.unwrap())),
            IndirectY => Logger::log_cpu_instruction(cpu, self.instruction, self.operand1, self.operand2, format!("{instruction_name} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", self.memory_indirect_address.unwrap(), self.memory_address.unwrap(), (Wrapping::<u16>(self.memory_address.unwrap()) + Wrapping::<u16>(cpu.get_y() as u16)).0, self.value.unwrap())),
        }
    }

    pub fn get_imp(cpu: &CPU, memory: &mut MEM) -> Self {
        let instruction = cpu.get_instr(memory);
        return Self { mode: Implicit, instruction, operand1: None, operand2: None, value: None, memory_address: None, memory_indirect_address: None }
    }

    pub fn get_acc(cpu: &CPU, memory: &mut MEM) -> Self {
        let instruction = cpu.get_instr(memory);
        return Self { mode: Acc, instruction, operand1: None, operand2: None, value: None, memory_address: None, memory_indirect_address: None }
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
        let value = memory.read((Wrapping::<u8>(memory_address) + Wrapping::<u8>(cpu.get_x())).0 as usize, 1) as u8;
        return Self { mode: ZeroPageX, instruction, operand1: Some(memory_address), operand2: None, value: Some(value), memory_address: Some(memory_address as u16), memory_indirect_address: None }
    }

    pub fn get_zpgy(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_address) = cpu.get_instr_and_operand(memory);
        let value = memory.read((Wrapping::<u8>(memory_address) + Wrapping::<u8>(cpu.get_y())).0 as usize, 1) as u8;
        return Self { mode: ZeroPageY, instruction, operand1: Some(memory_address), operand2: None, value: Some(value), memory_address: Some(memory_address as u16), memory_indirect_address: None }
    }

    pub fn get_rel(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, offset) = cpu.get_instr_and_operand(memory);
        let absolute_address = (Wrapping::<u16>(cpu.get_pc()) + Wrapping::<u16>(2) + Wrapping::<u16>(offset as i8 as u16)).0;
        return Self { mode: Relative, instruction, operand1: Some(offset), operand2: None, value: Some(offset), memory_address: Some(absolute_address), memory_indirect_address: None }
    }

    pub fn get_abs(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let value = memory.read(memory_address as usize, 1) as u8;
        return Self { mode: Absolute, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(memory_address), memory_indirect_address: None }
    }

    pub fn get_absx(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let value = memory.read((Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_x() as u16)).0 as usize, 1) as u8;
        return Self { mode: AbsoluteX, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(memory_address), memory_indirect_address: None }
    }

    pub fn get_absy(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, operand1, operand2) = cpu.get_instr_and_operands(memory);
        let memory_address = combine_operands(operand1, operand2);
        let value = memory.read((Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0 as usize, 1) as u8;
        return Self { mode: AbsoluteY, instruction, operand1: Some(operand1), operand2: Some(operand2), value: Some(value), memory_address: Some(memory_address), memory_indirect_address: None }
    }

    pub fn get_indirect(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_indirect_address) = cpu.get_instr_and_operand(memory);
        let memory_address = memory.read((memory_indirect_address) as usize, 2) as u16;
        return Self { mode: Indirect, instruction, operand1: Some(memory_indirect_address), operand2: None, value: None, memory_address: Some(memory_address), memory_indirect_address: Some(memory_indirect_address) }
    }

    pub fn get_indirect_x(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_indirect_address) = cpu.get_instr_and_operand(memory);
        let memory_address = memory.read((Wrapping::<u8>(memory_indirect_address) + Wrapping::<u8>(cpu.get_x())).0 as usize, 2) as u16;
        let value = memory.read(memory_address as usize, 1) as u8;
        return Self { mode: IndirectX, instruction, operand1: Some(memory_indirect_address), operand2: None, value: Some(value), memory_address: Some(memory_address), memory_indirect_address: Some(memory_indirect_address) }
    }

    pub fn get_indirect_y(cpu: &CPU, memory: &mut MEM) -> Self {
        let (instruction, memory_indirect_address) = cpu.get_instr_and_operand(memory);
        let memory_address = memory.read(memory_indirect_address as usize, 2) as u16;
        let value = memory.read((Wrapping::<u16>(memory_address) + Wrapping::<u16>(cpu.get_y() as u16)).0 as usize, 1) as u8;
        return Self { mode: IndirectY, instruction, operand1: Some(memory_indirect_address), operand2: None, value: Some(value), memory_address: Some(memory_address), memory_indirect_address: Some(memory_indirect_address) }
    }
}