use rexta::{op::OpCode, u24::U24};

#[derive(Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
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
            Register::R8 => 8,
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Addr(U24),
    Label(String),
}

#[derive(Debug)]
pub enum Instruction {
    ADD1 { rd: Register, rs: Register },
    ADD2 { rd: Register, rs: Register },
    ADD3 { rd: Register, rs: Register },

    SUB1 { rd: Register, rs: Register },
    SUB2 { rd: Register, rs: Register },
    SUB3 { rd: Register, rs: Register },

    AND1 { rd: Register, rs: Register },
    AND2 { rd: Register, rs: Register },
    AND3 { rd: Register, rs: Register },

    OR1 { rd: Register, rs: Register },
    OR2 { rd: Register, rs: Register },
    OR3 { rd: Register, rs: Register },

    XOR1 { rd: Register, rs: Register },
    XOR2 { rd: Register, rs: Register },
    XOR3 { rd: Register, rs: Register },

    NOT1 { rd: Register },
    NOT2 { rd: Register },
    NOT3 { rd: Register },

    LOADI1 { rd: Register, imm: u8 },
    LOADI2 { rd: Register, imm: u16 },
    LOADI3 { rd: Register, imm: U24 },

    ADDI1 { rd: Register, imm: u8 },
    ADDI2 { rd: Register, imm: u16 },
    ADDI3 { rd: Register, imm: U24 },

    MOV1 { rd: Register, rs: Register },
    MOV2 { rd: Register, rs: Register },
    MOV3 { rd: Register, rs: Register },

    INC1 { rd: Register },
    INC2 { rd: Register },
    INC3 { rd: Register },

    DEC1 { rd: Register },
    DEC2 { rd: Register },
    DEC3 { rd: Register },

    NEG1 { rd: Register },
    NEG2 { rd: Register },
    NEG3 { rd: Register },

    SHL1 { rd: Register },
    SHL2 { rd: Register },
    SHL3 { rd: Register },

    SHR1 { rd: Register },
    SHR2 { rd: Register },
    SHR3 { rd: Register },

    ROL1 { rd: Register },
    ROL2 { rd: Register },
    ROL3 { rd: Register },

    ROR1 { rd: Register },
    ROR2 { rd: Register },
    ROR3 { rd: Register },

    CMP1 { rd: Register, rs: Register },
    CMP2 { rd: Register, rs: Register },
    CMP3 { rd: Register, rs: Register },

    TST1 { rd: Register, rs: Register },
    TST2 { rd: Register, rs: Register },
    TST3 { rd: Register, rs: Register },

    PUSH1 { rs: Register },
    PUSH2 { rs: Register },
    PUSH3 { rs: Register },

    POP1 { rd: Register },
    POP2 { rd: Register },
    POP3 { rd: Register },

    LOAD1 { rd: Register, addr: Address },
    LOAD2 { rd: Register, addr: Address },
    LOAD3 { rd: Register, addr: Address },

    STORE1 { rs: Register, addr: Address },
    STORE2 { rs: Register, addr: Address },
    STORE3 { rs: Register, addr: Address },

    JMP { addr: Address },
    JZ { addr: Address },
    JC { addr: Address },
    JNZ { addr: Address },
    JNC { addr: Address },
    JSR { addr: Address },

    JMPA { addr: Address },
    JZA { addr: Address },
    JCA { addr: Address },
    JNZA { addr: Address },
    JNCA { addr: Address },
    JSRA { addr: Address },

    RTS,
    HLT,
}

impl Instruction {
    pub fn opcode_bytes(&self) -> [u8; 2] {
        let opcode = self.opcode() as u16;
        opcode.to_le_bytes()
    }

    /// Get the opcode for an instruction
    pub fn opcode(&self) -> OpCode {
        match self {
            Instruction::HLT => OpCode::HLT,
            Instruction::RTS => OpCode::RTS,
            Instruction::ADD1 { .. } => OpCode::ADD1,
            Instruction::SUB1 { .. } => OpCode::SUB1,
            Instruction::AND1 { .. } => OpCode::AND1,
            Instruction::OR1 { .. } => OpCode::OR1,
            Instruction::XOR1 { .. } => OpCode::XOR1,
            Instruction::MOV1 { .. } => OpCode::MOV1,
            Instruction::INC1 { .. } => OpCode::INC1,
            Instruction::DEC1 { .. } => OpCode::DEC1,
            Instruction::NEG1 { .. } => OpCode::NEG1,
            Instruction::NOT1 { .. } => OpCode::NOT1,
            Instruction::SHL1 { .. } => OpCode::SHL1,
            Instruction::SHR1 { .. } => OpCode::SHR1,
            Instruction::ROL1 { .. } => OpCode::ROL1,
            Instruction::ROR1 { .. } => OpCode::ROR1,
            Instruction::CMP1 { .. } => OpCode::CMP1,
            Instruction::TST1 { .. } => OpCode::TST1,
            Instruction::PUSH1 { .. } => OpCode::PUSH1,
            Instruction::POP1 { .. } => OpCode::POP1,
            Instruction::ADD2 { .. } => OpCode::ADD2,
            Instruction::SUB2 { .. } => OpCode::SUB2,
            Instruction::AND2 { .. } => OpCode::AND2,
            Instruction::OR2 { .. } => OpCode::OR2,
            Instruction::XOR2 { .. } => OpCode::XOR2,
            Instruction::MOV2 { .. } => OpCode::MOV2,
            Instruction::INC2 { .. } => OpCode::INC2,
            Instruction::DEC2 { .. } => OpCode::DEC2,
            Instruction::NEG2 { .. } => OpCode::NEG2,
            Instruction::NOT2 { .. } => OpCode::NOT2,
            Instruction::SHL2 { .. } => OpCode::SHL2,
            Instruction::SHR2 { .. } => OpCode::SHR2,
            Instruction::ROL2 { .. } => OpCode::ROL2,
            Instruction::ROR2 { .. } => OpCode::ROR2,
            Instruction::CMP2 { .. } => OpCode::CMP2,
            Instruction::TST2 { .. } => OpCode::TST2,
            Instruction::PUSH2 { .. } => OpCode::PUSH2,
            Instruction::POP2 { .. } => OpCode::POP2,
            Instruction::ADD3 { .. } => OpCode::ADD3,
            Instruction::SUB3 { .. } => OpCode::SUB3,
            Instruction::AND3 { .. } => OpCode::AND3,
            Instruction::OR3 { .. } => OpCode::OR3,
            Instruction::XOR3 { .. } => OpCode::XOR3,
            Instruction::MOV3 { .. } => OpCode::MOV3,
            Instruction::INC3 { .. } => OpCode::INC3,
            Instruction::DEC3 { .. } => OpCode::DEC3,
            Instruction::NEG3 { .. } => OpCode::NEG3,
            Instruction::NOT3 { .. } => OpCode::NOT3,
            Instruction::SHL3 { .. } => OpCode::SHL3,
            Instruction::SHR3 { .. } => OpCode::SHR3,
            Instruction::ROL3 { .. } => OpCode::ROL3,
            Instruction::ROR3 { .. } => OpCode::ROR3,
            Instruction::CMP3 { .. } => OpCode::CMP3,
            Instruction::TST3 { .. } => OpCode::TST3,
            Instruction::PUSH3 { .. } => OpCode::PUSH3,
            Instruction::POP3 { .. } => OpCode::POP3,
            Instruction::LOADI1 { .. } => OpCode::LOADI1,
            Instruction::ADDI1 { .. } => OpCode::ADDI1,
            Instruction::JMP { .. } => OpCode::JMP,
            Instruction::JZ { .. } => OpCode::JZ,
            Instruction::JNZ { .. } => OpCode::JNZ,
            Instruction::JC { .. } => OpCode::JC,
            Instruction::JNC { .. } => OpCode::JNC,
            Instruction::JSR { .. } => OpCode::JSR,
            Instruction::LOADI2 { .. } => OpCode::LOADI2,
            Instruction::ADDI2 { .. } => OpCode::ADDI2,
            Instruction::JMPA { .. } => OpCode::JMPA,
            Instruction::JZA { .. } => OpCode::JZA,
            Instruction::JNZA { .. } => OpCode::JNZA,
            Instruction::JCA { .. } => OpCode::JCA,
            Instruction::JNCA { .. } => OpCode::JNCA,
            Instruction::JSRA { .. } => OpCode::JSRA,
            Instruction::LOAD1 { .. } => OpCode::LOAD1,
            Instruction::STORE1 { .. } => OpCode::STORE1,
            Instruction::LOAD2 { .. } => OpCode::LOAD2,
            Instruction::STORE2 { .. } => OpCode::STORE2,
            Instruction::LOADI3 { .. } => OpCode::LOADI3,
            Instruction::LOAD3 { .. } => OpCode::LOAD3,
            Instruction::STORE3 { .. } => OpCode::STORE3,
            Instruction::ADDI3 { .. } => OpCode::ADDI3,
        }
    }

    pub fn length(&self) -> u8 {
        ((self.opcode() as u16 & 0xE00) >> 9) as u8 + 2
    }
}
