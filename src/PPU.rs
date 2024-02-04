use std::num::Wrapping;

use crate::MEM;

pub mod execution;

#[allow(non_snake_case)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)] // TODO: remove me
#[derive(Default)]
#[derive(Debug)]
pub struct CPU {
    pub PC: Wrapping<u16>,  // Program Count
    A: Wrapping<u8>,    // Accumulator
    X: Wrapping<u8>,    // Register X
    Y: Wrapping<u8>,    // Register Y
    S: Wrapping<u8>,    // Stack Pointer
    
    N: bool,
    V: bool,
    B: bool,
    D: bool,
    I: bool,
    Z: bool,
    C: bool,

    temp_var: u32,
    vblank_cycle: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MemoryMode {
    Implicit,
    Accumulator,
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

#[allow(dead_code)]
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

    ECHO,
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
            0xA2 => Ok(Opcodes::LDX(Immediate)),
            0xA6 => Ok(Opcodes::LDX(ZeroPage)),
            0xA0 => Ok(Opcodes::LDY(Immediate)),
            0xA4 => Ok(Opcodes::LDY(ZeroPage)),
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
            0x65 => Ok(Opcodes::ADC(ZeroPage)),
            0xC9 => Ok(Opcodes::CMP(Immediate)),
            0xCD => Ok(Opcodes::CMP(Absolute)),
            0xE0 => Ok(Opcodes::CPX(Immediate)),
            0xEC => Ok(Opcodes::CPX(Immediate)),
            0xC0 => Ok(Opcodes::CPY(Immediate)),
            0xCC => Ok(Opcodes::CPY(Absolute)),

            // INC/DEC
            0xE6 => Ok(Opcodes::INC(ZeroPage)),
            0xF6 => Ok(Opcodes::INC(ZeroPageX)),
            0xEE => Ok(Opcodes::INC(Absolute)),
            0xFE => Ok(Opcodes::INC(AbsoluteX)),
            0xE8 => Ok(Opcodes::INX(Implicit)),
            0xC8 => Ok(Opcodes::INY(Implicit)),
            // 0xC8 => Ok(Opcodes::DEC(Implicit)),
            0xCA => Ok(Opcodes::DEX(Implicit)),
            0x88 => Ok(Opcodes::DEY(Implicit)),

            // SHIFTS


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

            // 0x01 => Opcodes::ORA(IndirectX),
            // 0x05 => Opcodes::ORA(ZeroPage),
            // 0x06 => Opcodes::ASL(ZeroPage),

            0x02 => Ok(Opcodes::ECHO),
            _ => Err(()),
        }
    }
}

impl CPU {
    pub fn push_stack(&mut self, data: u8, memory: &mut MEM) {
        // println!("memory now {:#04X}", memory.read(0x0100 + self.S.0 as usize, 1));
        memory.data[0x0100 + self.S.0 as usize] = data;
        // println!("memory after {:#04X}", memory.read(0x0100 + self.S.0 as usize, 1));
        self.S -= 1;
    }

    pub fn pull_stack(&mut self, memory: &mut MEM) -> u8 {
        self.S += 1;
        let memory_address = 0x0100 + self.S.0 as usize;
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
        if status & 0b_0001_0000 != 0 {self.B = true} else {self.B = false};
        if status & 0b_0100_0000 != 0 {self.V = true} else {self.V = false};
        if status & 0b_1000_0000 != 0 {self.N = true} else {self.N = false};
    }

    pub fn next_pc(&mut self) -> usize {
        (self.PC + Wrapping(1)).0 as usize
    }
}