/// Represents an operation being performed by the CPU.
pub struct Op {
    pub code: OpCode,

    /// Destination register if present.
    pub rd: Option<u8>,

    /// Source register if present.
    pub rs: Option<u8>,

    /// Immediate value if present.
    pub imm: Option<u8>,

    /// Address value if present.
    pub addr: Option<u16>
}

impl Op {
    /// Create a new op set to no-op with no parameters.
    pub fn new() -> Self {
        Op {
            code: OpCode::NOP,
            rd: None,
            rs: None,
            imm: None,
            addr: None
        }
    }
}

/// Defines codes for all operations supported by the Rexta CPU.
#[repr(u8)]
pub enum OpCode {  
    NOP = 0x00,

    RTS = 0x01,
    HLT = 0x02,

    NOT = 0x10,

    ADD = 0x20,
    SUB = 0x21,
    AND = 0x22,
    OR = 0x23,
    XOR = 0x24,

    LOADI = 0x30,
    ADDI = 0x31,

    LOAD = 0x40,
    STORE = 0x41,

    JMP = 0x50,
    JZ = 0x51,
    JC = 0x52,
    JSR = 0x53,
}

impl TryFrom<u8> for OpCode {    
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::NOP),
            0x01 => Ok(OpCode::RTS),
            0x02 => Ok(OpCode::HLT),
            0x10 => Ok(OpCode::NOT),
            0x20 => Ok(OpCode::ADD),
            0x21 => Ok(OpCode::SUB),
            0x22 => Ok(OpCode::AND),
            0x23 => Ok(OpCode::OR),
            0x24 => Ok(OpCode::XOR),
            0x30 => Ok(OpCode::LOADI),
            0x31 => Ok(OpCode::ADDI),
            0x40 => Ok(OpCode::LOAD),
            0x41 => Ok(OpCode::STORE),
            0x50 => Ok(OpCode::JMP),
            0x51 => Ok(OpCode::JZ),
            0x52 => Ok(OpCode::JC),
            0x53 => Ok(OpCode::JSR),            
            _ => Err(()),
        }
    }
}

/// Defines parameter formats for operations; this value is 
/// stored in the upper 4 bits of the opcode.
#[repr(u8)]
pub enum OpFormat {
    None = 0x0,   // No operands
    Rd = 0x1,     // Rd only in lower 4 bits of B1
    RdRs = 0x2,   // Rd in lower 4 bits of B1, Rs in upper 4 bits
    RdImm = 0x3,  // Rd in lower 4 bits of B1, immediate value in B2
    RdAddr = 0x4, // Rd in lower 4 bits of B1, addr in B2 & B3
    Addr = 0x5,   // addr in B1 & B2
}

impl TryFrom<u8> for OpFormat {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(OpFormat::None),
            0x1 => Ok(OpFormat::Rd),
            0x2 => Ok(OpFormat::RdRs),
            0x3 => Ok(OpFormat::RdImm),
            0x4 => Ok(OpFormat::RdAddr),
            0x5 => Ok(OpFormat::Addr),
            _ => Err(()),
        }
    }
}