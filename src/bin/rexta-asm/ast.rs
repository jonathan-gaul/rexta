use rexta::u24::U24;

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

pub enum Address {
    Addr(U24),
    Label(String),
}

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

    CMP1 { rd: Register, rs:Register },
    CMP2 { rd: Register, rs:Register },
    CMP3 { rd: Register, rs:Register },

    TST1 { rd: Register, rs:Register },
    TST2 { rd: Register, rs:Register },
    TST3 { rd: Register, rs:Register },

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
        let opcode = self.opcode();
        [ (opcode & 0xFF) as u8, ((opcode & 0xFF00 >> 8) as u8) ]
    }

    /// Get the opcode for an instruction
    pub fn opcode(&self) -> u16 {
        match self {
            Instruction::HLT => 0x0004,
            Instruction::RTS => 0x0008,
            Instruction::ADD1 { .. } => 0x0201,
            Instruction::SUB1 { .. } => 0x0205,
            Instruction::AND1 { .. } => 0x0209,
            Instruction::OR1 { .. } => 0x020D,
            Instruction::XOR1 { .. } => 0x0211,
            Instruction::MOV1 { .. } => 0x0215,
            Instruction::INC1 { .. } => 0x0219,
            Instruction::DEC1 { .. } => 0x021D,
            Instruction::NEG1 { .. } => 0x0221,
            Instruction::NOT1 { .. } => 0x0225,
            Instruction::SHL1 { .. } => 0x0229,
            Instruction::SHR1 { .. } => 0x022D,
            Instruction::ROL1 { .. } => 0x0231,
            Instruction::ROR1 { .. } => 0x0235,
            Instruction::CMP1 { .. } => 0x0239,
            Instruction::TST1 { .. } => 0x023D,
            Instruction::PUSH1 { .. } => 0x0241,
            Instruction::POP1 { .. } => 0x0245,
            Instruction::ADD2 { .. } => 0x0202,
            Instruction::SUB2 { .. } => 0x0206,
            Instruction::AND2 { .. } => 0x020A,
            Instruction::OR2 { .. } => 0x020E,
            Instruction::XOR2 { .. } => 0x0212,
            Instruction::MOV2 { .. } => 0x0216,
            Instruction::INC2 { .. } => 0x021A,
            Instruction::DEC2 { .. } => 0x021E,
            Instruction::NEG2 { .. } => 0x0222,
            Instruction::NOT2 { .. } => 0x0226,
            Instruction::SHL2 { .. } => 0x022A,
            Instruction::SHR2 { .. } => 0x022E,
            Instruction::ROL2 { .. } => 0x0232,
            Instruction::ROR2 { .. } => 0x0236,
            Instruction::CMP2 { .. } => 0x023A,
            Instruction::TST2 { .. } => 0x023E,
            Instruction::PUSH2 { .. } => 0x0242,
            Instruction::POP2 { .. } => 0x0246,
            Instruction::ADD3 { .. } => 0x0203,
            Instruction::SUB3 { .. } => 0x0207,
            Instruction::AND3 { .. } => 0x020B,
            Instruction::OR3 { .. } => 0x020F,
            Instruction::XOR3 { .. } => 0x0213,
            Instruction::MOV3 { .. } => 0x0217,
            Instruction::INC3 { .. } => 0x021B,
            Instruction::DEC3 { .. } => 0x021F,
            Instruction::NEG3 { .. } => 0x0223,
            Instruction::NOT3 { .. } => 0x0227,
            Instruction::SHL3 { .. } => 0x022B,
            Instruction::SHR3 { .. } => 0x022F,
            Instruction::ROL3 { .. } => 0x0233,
            Instruction::ROR3 { .. } => 0x0237,
            Instruction::CMP3 { .. } => 0x023B,
            Instruction::TST3 { .. } => 0x023F,
            Instruction::PUSH3 { .. } => 0x0243,
            Instruction::POP3 { .. } => 0x0247,
            Instruction::LOADI1 { .. } => 0x0401,
            Instruction::ADDI1 { .. } => 0x0449,
            Instruction::JMP { .. } => 0x0600,
            Instruction::JZ { .. } => 0x0604,
            Instruction::JNZ { .. } => 0x0608,
            Instruction::JC { .. } => 0x060C,
            Instruction::JNC { .. } => 0x0610,
            Instruction::JSR { .. } => 0x0614,
            Instruction::LOADI2 { .. } => 0x0602,
            Instruction::ADDI2 { .. } => 0x064E,
            Instruction::JMPA { .. } => 0x0603,
            Instruction::JZA { .. } => 0x0607,
            Instruction::JNZA { .. } => 0x060B,
            Instruction::JCA { .. } => 0x060F,
            Instruction::JNCA { .. } => 0x0613,
            Instruction::JSRA { .. } => 0x0617,
            Instruction::LOAD1 { .. } => 0x0805,
            Instruction::STORE1 { .. } => 0x0809,
            Instruction::LOAD2 { .. } => 0x0806,
            Instruction::STORE2 { .. } => 0x080A,
            Instruction::LOADI3 { .. } => 0x0803,
            Instruction::LOAD3 { .. } => 0x0807,
            Instruction::STORE3 { .. } => 0x080B,
            Instruction::ADDI3 { .. } => 0x0853,
        }
    }

    pub fn length(&self) -> u8 {
        ((self.opcode() & 0xE00) >> 9) as u8 + 2
    }
}
