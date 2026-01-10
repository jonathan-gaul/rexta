
/// Represents an operation being performed by the CPU.
pub struct Op {
    pub code: OpCode,

    // Operands    
    pub operands: [u8; 4],    
}

impl Op {
    /// Create a new op set to no-op with no parameters.
    pub fn new() -> Self {
        Op {
            code: OpCode::NOP,
            operands: [0; 4],
        }
    }

    pub fn rd(&self) -> u8 {
        (self.operands[0] & 0xF0) >> 4
    }

    pub fn rs(&self) -> u8 {
        self.operands[0] & 0x0F
    }    
}

/// Defines codes for all operations supported by the Rexta CPU.
/// A complete list is here:
/// https://github.com/jonathan-gaul/rexta-docs/blob/main/CPU/OpCode%20Table.xlsx
#[repr(u16)]
pub enum OpCode {
    NOP = 0x0000,

    HLT = 0x0004,
    RTS = 0x0008,
    ADD1 = 0x0201,
    ADD2 = 0x0202,
    ADD3 = 0x0203,
    SUB1 = 0x0205,
    SUB2 = 0x0206,
    SUB3 = 0x0207,
    AND1 = 0x0209,
    AND2 = 0x020A,
    AND3 = 0x020B,
    OR1 = 0x020D,
    OR2 = 0x020E,
    OR3 = 0x020F,
    XOR1 = 0x0211,
    XOR2 = 0x0212,
    XOR3 = 0x0213,
    MOV1 = 0x0215,
    MOV2 = 0x0216,
    MOV3 = 0x0217,
    INC1 = 0x0219,
    INC2 = 0x021A,
    INC3 = 0x021B,
    DEC1 = 0x021D,
    DEC2 = 0x021E,
    DEC3 = 0x021F,
    NEG1 = 0x0221,
    NEG2 = 0x0222,
    NEG3 = 0x0223,
    NOT1 = 0x0225,
    NOT2 = 0x0226,
    NOT3 = 0x0227,
    SHL1 = 0x0229,
    SHL2 = 0x022A,
    SHL3 = 0x022B,
    SHR1 = 0x022D,
    SHR2 = 0x022E,
    SHR3 = 0x022F,
    ROL1 = 0x0231,
    ROL2 = 0x0232,
    ROL3 = 0x0233,
    ROR1 = 0x0235,
    ROR2 = 0x0236,
    ROR3 = 0x0237,
    CMP1 = 0x0239,
    CMP2 = 0x023A,
    CMP3 = 0x023B,
    TST1 = 0x023D,
    TST2 = 0x023E,
    TST3 = 0x023F,
    PUSH1 = 0x0241,
    PUSH2 = 0x0242,
    PUSH3 = 0x0243,
    POP1 = 0x0245,
    POP2 = 0x0246,
    POP3 = 0x0247,
    JMP = 0x0600,
    JMPA = 0x0603,
    JZ = 0x0604,
    JZA = 0x0607,
    JNZ = 0x0608,
    JNZA = 0x060B,
    JC = 0x060C,
    JCA = 0x060F,
    JNC = 0x0610,
    JNCA = 0x0613,
    JSR = 0x0614,
    JSRA = 0x0617,
    LOADI1 = 0x0801,
    LOADI2 = 0x0802,
    LOADI3 = 0x0803,
    LOAD1 = 0x0805,
    LOAD2 = 0x0806,
    LOAD3 = 0x0807,
    STORE1 = 0x0809,
    STORE2 = 0x080A,
    STORE3 = 0x080B,
}

impl TryFrom<u16> for OpCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0004 => Ok(OpCode::HLT),
            0x0008 => Ok(OpCode::RTS),
            0x0201 => Ok(OpCode::ADD1),
            0x0202 => Ok(OpCode::ADD2),
            0x0203 => Ok(OpCode::ADD3),
            0x0205 => Ok(OpCode::SUB1),
            0x0206 => Ok(OpCode::SUB2),
            0x0207 => Ok(OpCode::SUB3),
            0x0209 => Ok(OpCode::AND1),
            0x020A => Ok(OpCode::AND2),
            0x020B => Ok(OpCode::AND3),
            0x020D => Ok(OpCode::OR1),
            0x020E => Ok(OpCode::OR2),
            0x020F => Ok(OpCode::OR3),
            0x0211 => Ok(OpCode::XOR1),
            0x0212 => Ok(OpCode::XOR2),
            0x0213 => Ok(OpCode::XOR3),
            0x0215 => Ok(OpCode::MOV1),
            0x0216 => Ok(OpCode::MOV2),
            0x0217 => Ok(OpCode::MOV3),
            0x0219 => Ok(OpCode::INC1),
            0x021A => Ok(OpCode::INC2),
            0x021B => Ok(OpCode::INC3),
            0x021D => Ok(OpCode::DEC1),
            0x021E => Ok(OpCode::DEC2),
            0x021F => Ok(OpCode::DEC3),
            0x0221 => Ok(OpCode::NEG1),
            0x0222 => Ok(OpCode::NEG2),
            0x0223 => Ok(OpCode::NEG3),
            0x0225 => Ok(OpCode::NOT1),
            0x0226 => Ok(OpCode::NOT2),
            0x0227 => Ok(OpCode::NOT3),
            0x0229 => Ok(OpCode::SHL1),
            0x022A => Ok(OpCode::SHL2),
            0x022B => Ok(OpCode::SHL3),
            0x022D => Ok(OpCode::SHR1),
            0x022E => Ok(OpCode::SHR2),
            0x022F => Ok(OpCode::SHR3),
            0x0231 => Ok(OpCode::ROL1),
            0x0232 => Ok(OpCode::ROL2),
            0x0233 => Ok(OpCode::ROL3),
            0x0235 => Ok(OpCode::ROR1),
            0x0236 => Ok(OpCode::ROR2),
            0x0237 => Ok(OpCode::ROR3),
            0x0239 => Ok(OpCode::CMP1),
            0x023A => Ok(OpCode::CMP2),
            0x023B => Ok(OpCode::CMP3),
            0x023D => Ok(OpCode::TST1),
            0x023E => Ok(OpCode::TST2),
            0x023F => Ok(OpCode::TST3),
            0x0241 => Ok(OpCode::PUSH1),
            0x0242 => Ok(OpCode::PUSH2),
            0x0243 => Ok(OpCode::PUSH3),
            0x0245 => Ok(OpCode::POP1),
            0x0246 => Ok(OpCode::POP2),
            0x0247 => Ok(OpCode::POP3),
            0x0600 => Ok(OpCode::JMP),
            0x0603 => Ok(OpCode::JMPA),
            0x0604 => Ok(OpCode::JZ),
            0x0607 => Ok(OpCode::JZA),
            0x0608 => Ok(OpCode::JNZ),
            0x060B => Ok(OpCode::JNZA),
            0x060C => Ok(OpCode::JC),
            0x060F => Ok(OpCode::JCA),
            0x0610 => Ok(OpCode::JNC),
            0x0613 => Ok(OpCode::JNCA),
            0x0614 => Ok(OpCode::JSR),
            0x0617 => Ok(OpCode::JSRA),
            0x0801 => Ok(OpCode::LOADI1),
            0x0802 => Ok(OpCode::LOADI2),
            0x0803 => Ok(OpCode::LOADI3),
            0x0805 => Ok(OpCode::LOAD1),
            0x0806 => Ok(OpCode::LOAD2),
            0x0807 => Ok(OpCode::LOAD3),
            0x0809 => Ok(OpCode::STORE1),
            0x080A => Ok(OpCode::STORE2),
            0x080B => Ok(OpCode::STORE3),
            _ => Err(()),
        }
    }
}
