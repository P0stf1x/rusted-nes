use std::num::Wrapping;

use derive_new::new;

use crate::MEM;
use crate::types::Wrapped;

use self::settings::Settings;

pub mod execution;
pub mod settings;

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[derive(Debug)]
#[derive(new)]
pub struct CPU {
    #[new(value = "Wrapped::word(0)")]
    pub PC: Wrapped,  // Program Count
    #[new(value = "Wrapped::byte(0)")]
    A: Wrapped,    // Accumulator
    #[new(value = "Wrapped::byte(0)")]
    X: Wrapped,    // Register X
    #[new(value = "Wrapped::byte(0)")]
    Y: Wrapped,    // Register Y
    #[new(value = "Wrapped::byte(0)")]
    pub S: Wrapped,    // Stack Pointer
    
    #[new(default)]
    N: bool,
    #[new(default)]
    V: bool,
    #[new(default)]
    B: bool,
    #[new(default)]
    D: bool,
    #[new(default)]
    pub I: bool,
    #[new(default)]
    Z: bool,
    #[new(default)]
    C: bool,
    
    #[new(default)]
    settings: Settings,
    #[new(default)]
    executed_opcodes: u32,
}

#[derive(Debug)]
pub enum MemoryMode {
    Implicit,
    Acc,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[derive(Debug)]
pub enum Opcodes {
    ADC(MemoryMode),
    AND(MemoryMode),
    ASL(MemoryMode),
    BCC(MemoryMode),
    BCS(MemoryMode),
    BEQ(MemoryMode),
    BIT(MemoryMode),
    BMI(MemoryMode),
    BNE(MemoryMode),
    BPL(MemoryMode),
    BRK(MemoryMode),
    BVC(MemoryMode),
    BVS(MemoryMode),
    CLC(MemoryMode),
    CLD(MemoryMode),
    CLI(MemoryMode),
    CLV(MemoryMode),
    CMP(MemoryMode),
    CPX(MemoryMode),
    CPY(MemoryMode),
    DEC(MemoryMode),
    DEX(MemoryMode),
    DEY(MemoryMode),
    EOR(MemoryMode),
    INC(MemoryMode),
    INX(MemoryMode),
    INY(MemoryMode),
    JMP(MemoryMode),
    JSR(MemoryMode),
    LDA(MemoryMode),
    LDX(MemoryMode),
    LDY(MemoryMode),
    LSR(MemoryMode),
    NOP(MemoryMode),
    ORA(MemoryMode),
    PHA(MemoryMode),
    PHP(MemoryMode),
    PLA(MemoryMode),
    PLP(MemoryMode),
    ROL(MemoryMode),
    ROR(MemoryMode),
    RTI(MemoryMode),
    RTS(MemoryMode),
    SBC(MemoryMode),
    SEC(MemoryMode),
    SED(MemoryMode),
    SEI(MemoryMode),
    STA(MemoryMode),
    STX(MemoryMode),
    STY(MemoryMode),
    TAX(MemoryMode),
    TAY(MemoryMode),
    TSX(MemoryMode),
    TXA(MemoryMode),
    TXS(MemoryMode),
    TYA(MemoryMode),
}

impl CPU {
    pub fn from(&mut self, value: u8) -> Result<Opcodes, ()> {
        use MemoryMode::*;
        match value {

            // LOAD/STORE
            0xA9 => Ok(Opcodes::LDA(Immediate)),
            0xA5 => Ok(Opcodes::LDA(ZeroPage)),
            0xB5 => Ok(Opcodes::LDA(ZeroPageX)),
            0xAD => Ok(Opcodes::LDA(Absolute)),
            0xBD => Ok(Opcodes::LDA(AbsoluteX)),
            0xB9 => Ok(Opcodes::LDA(AbsoluteY)),
            0xA1 => Ok(Opcodes::LDA(IndirectX)),
            0xB1 => Ok(Opcodes::LDA(IndirectY)),
            0xA2 => Ok(Opcodes::LDX(Immediate)),
            0xA6 => Ok(Opcodes::LDX(ZeroPage)),
            0xB6 => Ok(Opcodes::LDX(ZeroPageY)),
            0xAE => Ok(Opcodes::LDX(Absolute)),
            0xBE => Ok(Opcodes::LDX(AbsoluteY)),
            0xA0 => Ok(Opcodes::LDY(Immediate)),
            0xA4 => Ok(Opcodes::LDY(ZeroPage)),
            0xB4 => Ok(Opcodes::LDY(ZeroPageX)),
            0xAC => Ok(Opcodes::LDY(Absolute)),
            0xBC => Ok(Opcodes::LDY(AbsoluteX)),
            0x85 => Ok(Opcodes::STA(ZeroPage)),
            0x95 => Ok(Opcodes::STA(ZeroPageX)),
            0x8D => Ok(Opcodes::STA(Absolute)),
            0x9D => Ok(Opcodes::STA(AbsoluteX)),
            0x99 => Ok(Opcodes::STA(AbsoluteY)),
            0x81 => Ok(Opcodes::STA(IndirectX)),
            0x91 => Ok(Opcodes::STA(IndirectY)),
            0x86 => Ok(Opcodes::STX(ZeroPage)),
            0x96 => Ok(Opcodes::STX(ZeroPageY)),
            0x8E => Ok(Opcodes::STX(Absolute)),
            0x84 => Ok(Opcodes::STY(ZeroPage)),
            0x94 => Ok(Opcodes::STY(ZeroPageX)),
            0x8C => Ok(Opcodes::STY(Absolute)),

            // TRANSFERS
            0xAA => Ok(Opcodes::TAX(Implicit)),
            0xA8 => Ok(Opcodes::TAY(Implicit)),
            0x8A => Ok(Opcodes::TXA(Implicit)),
            0x98 => Ok(Opcodes::TYA(Implicit)),
            0xBA => Ok(Opcodes::TSX(Implicit)),
            0x9A => Ok(Opcodes::TXS(Implicit)),

            // STACK
            0x48 => Ok(Opcodes::PHA(Implicit)),
            0x08 => Ok(Opcodes::PHP(Implicit)),
            0x68 => Ok(Opcodes::PLA(Implicit)),
            0x28 => Ok(Opcodes::PLP(Implicit)),

            // LOGIC
            0x29 => Ok(Opcodes::AND(Immediate)),
            0x25 => Ok(Opcodes::AND(ZeroPage)),
            0x35 => Ok(Opcodes::AND(ZeroPageX)),
            0x2D => Ok(Opcodes::AND(Absolute)),
            0x3D => Ok(Opcodes::AND(AbsoluteX)),
            0x39 => Ok(Opcodes::AND(AbsoluteY)),
            0x21 => Ok(Opcodes::AND(IndirectX)),
            0x31 => Ok(Opcodes::AND(IndirectY)),
            0x49 => Ok(Opcodes::EOR(Immediate)),
            0x45 => Ok(Opcodes::EOR(ZeroPage)),
            0x55 => Ok(Opcodes::EOR(ZeroPageX)),
            0x4D => Ok(Opcodes::EOR(Absolute)),
            0x5D => Ok(Opcodes::EOR(AbsoluteX)),
            0x59 => Ok(Opcodes::EOR(AbsoluteY)),
            0x41 => Ok(Opcodes::EOR(IndirectX)),
            0x51 => Ok(Opcodes::EOR(IndirectY)),
            0x09 => Ok(Opcodes::ORA(Immediate)),
            0x05 => Ok(Opcodes::ORA(ZeroPage)),
            0x15 => Ok(Opcodes::ORA(ZeroPageX)),
            0x0D => Ok(Opcodes::ORA(Absolute)),
            0x1D => Ok(Opcodes::ORA(AbsoluteX)),
            0x19 => Ok(Opcodes::ORA(AbsoluteY)),
            0x01 => Ok(Opcodes::ORA(IndirectX)),
            0x11 => Ok(Opcodes::ORA(IndirectY)),
            0x24 => Ok(Opcodes::BIT(ZeroPage)),
            0x2C => Ok(Opcodes::BIT(Absolute)),

            // ARITHMETIC
            0x69 => Ok(Opcodes::ADC(Immediate)),
            0x65 => Ok(Opcodes::ADC(ZeroPage)),
            0x75 => Ok(Opcodes::ADC(ZeroPageX)),
            0x6D => Ok(Opcodes::ADC(Absolute)),
            0x7D => Ok(Opcodes::ADC(AbsoluteX)),
            0x79 => Ok(Opcodes::ADC(AbsoluteY)),
            0x61 => Ok(Opcodes::ADC(IndirectX)),
            0x71 => Ok(Opcodes::ADC(IndirectY)),
            0xE9 => Ok(Opcodes::SBC(Immediate)),
            0xE5 => Ok(Opcodes::SBC(ZeroPage)),
            0xF5 => Ok(Opcodes::SBC(ZeroPageX)),
            0xED => Ok(Opcodes::SBC(Absolute)),
            0xFD => Ok(Opcodes::SBC(AbsoluteX)),
            0xF9 => Ok(Opcodes::SBC(AbsoluteY)),
            0xE1 => Ok(Opcodes::SBC(IndirectX)),
            0xF1 => Ok(Opcodes::SBC(IndirectY)),
            0xC9 => Ok(Opcodes::CMP(Immediate)),
            0xC5 => Ok(Opcodes::CMP(ZeroPage)),
            0xD5 => Ok(Opcodes::CMP(ZeroPageX)),
            0xCD => Ok(Opcodes::CMP(Absolute)),
            0xDD => Ok(Opcodes::CMP(AbsoluteX)),
            0xD9 => Ok(Opcodes::CMP(AbsoluteY)),
            0xC1 => Ok(Opcodes::CMP(IndirectX)),
            0xD1 => Ok(Opcodes::CMP(IndirectY)),
            0xE0 => Ok(Opcodes::CPX(Immediate)),
            0xE4 => Ok(Opcodes::CPX(ZeroPage)),
            0xEC => Ok(Opcodes::CPX(Absolute)),
            0xC0 => Ok(Opcodes::CPY(Immediate)),
            0xC4 => Ok(Opcodes::CPY(ZeroPage)),
            0xCC => Ok(Opcodes::CPY(Absolute)),

            // INC/DEC
            0xE6 => Ok(Opcodes::INC(ZeroPage)),
            0xF6 => Ok(Opcodes::INC(ZeroPageX)),
            0xEE => Ok(Opcodes::INC(Absolute)),
            0xFE => Ok(Opcodes::INC(AbsoluteX)),
            0xE8 => Ok(Opcodes::INX(Implicit)),
            0xC8 => Ok(Opcodes::INY(Implicit)),
            0xC6 => Ok(Opcodes::DEC(ZeroPage)),
            0xD6 => Ok(Opcodes::DEC(ZeroPageX)),
            0xCE => Ok(Opcodes::DEC(Absolute)),
            0xDE => Ok(Opcodes::DEC(AbsoluteX)),
            0xCA => Ok(Opcodes::DEX(Implicit)),
            0x88 => Ok(Opcodes::DEY(Implicit)),

            // SHIFTS
            0x0A => Ok(Opcodes::ASL(Acc)),
            0x06 => Ok(Opcodes::ASL(ZeroPage)),
            0x16 => Ok(Opcodes::ASL(ZeroPageX)),
            0x0E => Ok(Opcodes::ASL(Absolute)),
            0x1E => Ok(Opcodes::ASL(AbsoluteX)),
            0x4A => Ok(Opcodes::LSR(Acc)),
            0x46 => Ok(Opcodes::LSR(ZeroPage)),
            0x56 => Ok(Opcodes::LSR(ZeroPageX)),
            0x4E => Ok(Opcodes::LSR(Absolute)),
            0x5E => Ok(Opcodes::LSR(AbsoluteX)),
            0x2A => Ok(Opcodes::ROL(Acc)),
            0x26 => Ok(Opcodes::ROL(ZeroPage)),
            0x36 => Ok(Opcodes::ROL(ZeroPageX)),
            0x2E => Ok(Opcodes::ROL(Absolute)),
            0x3E => Ok(Opcodes::ROL(AbsoluteX)),
            0x6A => Ok(Opcodes::ROR(Acc)),
            0x66 => Ok(Opcodes::ROR(ZeroPage)),
            0x76 => Ok(Opcodes::ROR(ZeroPageX)),
            0x6E => Ok(Opcodes::ROR(Absolute)),
            0x7E => Ok(Opcodes::ROR(AbsoluteX)),

            // JUMPS
            0x4C => Ok(Opcodes::JMP(Absolute)),
            0x6C => Ok(Opcodes::JMP(Indirect)),
            0x20 => Ok(Opcodes::JSR(Absolute)),
            0x60 => Ok(Opcodes::RTS(Implicit)),

            // BRANCHES
            0xB0 => Ok(Opcodes::BCS(Relative)),
            0x90 => Ok(Opcodes::BCC(Relative)),
            0xF0 => Ok(Opcodes::BEQ(Relative)),
            0xD0 => Ok(Opcodes::BNE(Relative)),
            0x30 => Ok(Opcodes::BMI(Relative)),
            0x10 => Ok(Opcodes::BPL(Relative)),
            0x70 => Ok(Opcodes::BVS(Relative)),
            0x50 => Ok(Opcodes::BVC(Relative)),

            // STATUS
            0x38 => Ok(Opcodes::SEC(Implicit)),
            0x18 => Ok(Opcodes::CLC(Implicit)),
            0x78 => Ok(Opcodes::SEI(Implicit)),
            0x58 => Ok(Opcodes::CLI(Implicit)),
            0xF8 => Ok(Opcodes::SED(Implicit)),
            0xD8 => Ok(Opcodes::CLD(Implicit)),
            0xB8 => Ok(Opcodes::CLV(Implicit)),

            // SYSTEM
            0x00 => Ok(Opcodes::BRK(Implicit)),
            0xEA => Ok(Opcodes::NOP(Implicit)),
            0x40 => Ok(Opcodes::RTI(Implicit)),

            _ => Err(()),
        }
    }
}

impl CPU {
    pub fn push_stack(&mut self, data: u8, memory: &mut MEM) {
        memory.data[0x0100 + self.S.value as usize] = data;
        self.S -= 1;
    }

    pub fn pull_stack(&mut self, memory: &mut MEM) -> u8 {
        self.S += 1;
        let memory_address = 0x0100 + self.S.value as usize;
        let val = memory.read(memory_address, 1) as u8;
        return val;
    }

    pub fn store_status(&mut self) -> u8 {
        let mut status: u8 = 0b_0010_0000;
        if self.C {status |= 0b_0000_0001};
        if self.Z {status |= 0b_0000_0010};
        if self.I {status |= 0b_0000_0100};
        if self.D {status |= 0b_0000_1000};
        if self.B {status |= 0b_0001_0000};
        if self.V {status |= 0b_0100_0000};
        if self.N {status |= 0b_1000_0000};
        return status;
    }

    pub fn load_status(&mut self, status: u8) {
        if status & 0b_0000_0001 != 0 {self.C = true} else {self.C = false};
        if status & 0b_0000_0010 != 0 {self.Z = true} else {self.Z = false};
        if status & 0b_0000_0100 != 0 {self.I = true} else {self.I = false};
        if status & 0b_0000_1000 != 0 {self.D = true} else {self.D = false};
        // if status & 0b_0001_0000 != 0 {self.B = true} else {self.B = false};
        if status & 0b_0100_0000 != 0 {self.V = true} else {self.V = false};
        if status & 0b_1000_0000 != 0 {self.N = true} else {self.N = false};
    }

    pub fn next_pc(&mut self) -> usize {
        (self.PC + 1).value as usize
    }

    #[allow(dead_code)]
    pub fn nmi(&mut self, memory: &mut MEM) {
        let vector = memory.read(0xFFFA, 2);
        self.PC = Wrapped::word(vector as isize);
    }
    
    #[allow(dead_code)]
    pub fn reset(&mut self, memory: &mut MEM) {
        let vector = memory.read(0xFFFC, 2);
        self.PC = Wrapped::word(vector as isize);
    }
    
    #[allow(dead_code)]
    pub fn irq_brk(&mut self, memory: &mut MEM) {
        let vector = memory.read(0xFFFE, 2);
        self.PC = Wrapped::word(vector as isize);
    }
}
