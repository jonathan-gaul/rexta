use crate::ast::Instruction;
use crate::ast::Register;

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        let opcode = self.opcode();
        match self {
            Instruction::NOT { rd} =>
                vec![ opcode, rd.encode() ],

            Instruction::ADD { rd, rs }
            | Instruction::SUB { rd, rs } 
            | Instruction::AND { rd, rs } 
            | Instruction::OR { rd, rs } 
            | Instruction::XOR { rd, rs } => 
                vec![ opcode, rs.encode() << 4 | rd.encode() ],

            Instruction::LOADI { rd, imm } 
            | Instruction::ADDI { rd, imm} =>
                vec![ opcode, rd.encode(), *imm ],

            Instruction::LOAD { rd, addr } 
            | Instruction::STORE { rd, addr } =>
                vec![ opcode, rd.encode(), (*addr >> 8) as u8, *addr as u8 ],

            Instruction::JMP { addr } 
            | Instruction::JZ { addr } 
            | Instruction::JC { addr }
            | Instruction::JSR { addr } =>
                vec![ opcode, (*addr >> 8) as u8, *addr as u8 ],

            Instruction::RTS
            | Instruction::HLT => 
                vec![ opcode ]
        }
    }
}

fn parse_register(s: &str) -> Option<Register> {
    match s.to_uppercase().as_str() {
        "R0" => Some(Register::R0),
        "R1" => Some(Register::R1),
        "R2" => Some(Register::R2),        
        "R3" => Some(Register::R3),
        "R4" => Some(Register::R4),
        "R5" => Some(Register::R5),
        "R6" => Some(Register::R6),
        "R7" => Some(Register::R7),
        _ => None,
    }
}

fn parse_line(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line
        .split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return None;
    }

    match parts[0].to_uppercase().as_str() {
        "LOADI" => Some(Instruction::LOADI {
            rd: parse_register(parts[1])?,
            imm: parts[2].parse().ok()?,
        }),
        "ADDI" => Some(Instruction::ADDI {
            rd: parse_register(parts[1])?,
            imm: parts[2].parse().ok()?,
        }),
        "ADD" => Some(Instruction::ADD {
            rd: parse_register(parts[1])?,
            rs: parse_register(parts[2])?,
        }),
        "SUB" => Some(Instruction::SUB {
            rd: parse_register(parts[1])?,
            rs: parse_register(parts[2])?,
        }),
        "AND" => Some(Instruction::AND {
            rd: parse_register(parts[1])?,
            rs: parse_register(parts[2])?,
        }),
        "OR" => Some(Instruction::OR {
            rd: parse_register(parts[1])?,
            rs: parse_register(parts[2])?,
        }),
        "XOR" => Some(Instruction::XOR {
            rd: parse_register(parts[1])?,
            rs: parse_register(parts[2])?,
        }),
        "NOT" => Some(Instruction::NOT {
            rd: parse_register(parts[1])?,
        }),
        "LOAD" => Some(Instruction::LOAD {
            rd: parse_register(parts[1])?,
            addr: u16::from_str_radix(parts[2].trim_start_matches("0x"), 16).ok()?,
        }),
        "STORE" => Some(Instruction::STORE {
            rd: parse_register(parts[1])?,
            addr: u16::from_str_radix(parts[2].trim_start_matches("0x"), 16).ok()?,
        }),
        "JMP" => Some(Instruction::JMP {
            addr: u16::from_str_radix(parts[1].trim_start_matches("0x"), 16).ok()?,
        }),
        "JZ" => Some(Instruction::JZ {
            addr: u16::from_str_radix(parts[1].trim_start_matches("0x"), 16).ok()?,
        }),
        "JC" => Some(Instruction::JC {
            addr: u16::from_str_radix(parts[1].trim_start_matches("0x"), 16).ok()?,
        }),
        "JSR" => Some(Instruction::JSR {
            addr: u16::from_str_radix(parts[1].trim_start_matches("0x"), 16).ok()?,
        }),
        "RTS" => Some(Instruction::RTS),
        "HLT" => Some(Instruction::HLT),
        _ => None,
    }
}

pub fn assemble(text: &str) -> Vec<u8> {
    let lines: Vec<&str> = text
        .lines()
        .map(|line| line.split(';').next().unwrap().trim()) // strip comments
        .filter(|line| !line.is_empty())
        .collect();

    let program: Vec<Instruction> = lines
        .iter()
        .filter_map(|line| parse_line(line))
        .collect();

    program.iter().flat_map(|i| i.encode()).collect()
}