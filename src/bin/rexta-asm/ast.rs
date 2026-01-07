pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7
}

impl Register {
    pub fn encode(&self) -> u8 {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
        }
    }
}

pub enum Address {
    Addr(u16),
    Label(String),
}

pub enum Instruction {
    ADD { rd: Register, rs: Register },
    SUB { rd: Register, rs: Register },
    AND { rd: Register, rs: Register },
    OR { rd: Register, rs: Register },
    XOR { rd: Register, rs: Register },
    NOT { rd: Register },
    LOADI { rd: Register, imm: u8 },
    ADDI { rd: Register, imm: u8 },
    LOAD { rd: Register, addr: Address },
    STORE { rd: Register, addr: Address },
    JMP { addr: Address },
    JZ { addr: Address },
    JC { addr: Address },
    JSR { addr: Address },
    RTS,
    HLT,
}

impl Instruction {
    /// Get the opcode for an instruction
    pub fn opcode(&self) -> u8 {
        match self {
            Instruction::RTS => 0x01,
            Instruction::HLT => 0x02,
            Instruction::NOT { .. } => 0x10,
            Instruction::ADD { .. } => 0x20,
            Instruction::SUB { .. } => 0x21,
            Instruction::AND { .. } => 0x22,
            Instruction::OR { .. } => 0x23,
            Instruction::XOR { .. } => 0x24,            
            Instruction::LOADI { .. } => 0x30,
            Instruction::ADDI { .. } => 0x31,
            Instruction::LOAD { .. } => 0x40,
            Instruction::STORE { .. } => 0x41,
            Instruction::JMP { .. } => 0x50,
            Instruction::JZ { .. } => 0x51,
            Instruction::JC { .. } => 0x52,
            Instruction::JSR { .. } => 0x53,            
        }
    }

    pub fn length(&self) -> u8 {
        match self.opcode() >> 4 {
            0x0 => 1, // instruction only
            0x1 => 2, // instruction + rd
            0x2 => 2, // instruction + rd:rs
            0x3 => 3, // instruction + rd + imm
            0x4 => 4, // instruction + rd + addr
            0x5 => 3, // instruction + addr,
            _ => 1,   // unknown
        }
    }
}