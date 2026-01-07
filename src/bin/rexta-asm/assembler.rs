use std::collections::HashMap;

use crate::ast::Address;
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
                if let Address::Addr(a) = addr {
                    vec![ opcode, rd.encode(), (*a >> 8) as u8, *a as u8 ]
                } else {
                    panic!("Label not resolved");
                }                

            Instruction::JMP { addr } 
            | Instruction::JZ { addr } 
            | Instruction::JC { addr }
            | Instruction::JSR { addr } =>
                if let Address::Addr(a) = addr {
                    vec![ opcode, (*a >> 8) as u8, *a as u8 ]
                } else {
                    panic!("Label not resolved")
                }

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

fn parse_address(addr: &str) -> Option<Address> {
    if addr.starts_with("0x") {        
        let value = u16::from_str_radix(addr.trim_start_matches("0x"), 16).ok()?;
        Some(Address::Addr(value))
    } else {
        Some(Address::Label(addr.to_string()))
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
            addr: parse_address(parts[2])?,
        }),
        "STORE" => Some(Instruction::STORE {
            rd: parse_register(parts[1])?,
            addr: parse_address(parts[2])?,
        }),
        "JMP" => Some(Instruction::JMP {
            addr: parse_address(parts[1])?,
        }),
        "JZ" => Some(Instruction::JZ {
            addr: parse_address(parts[1])?,
        }),
        "JC" => Some(Instruction::JC {
            addr: parse_address(parts[1])?,
        }),
        "JSR" => Some(Instruction::JSR {
            addr: parse_address(parts[1])?,
        }),
        "RTS" => Some(Instruction::RTS),
        "HLT" => Some(Instruction::HLT),
        _ => None,
    }
}

fn create_label_map(lines: &Vec<&str>) -> HashMap<String, u16> {
    let mut labels = HashMap::new();
    let mut pc: u16 = 0;

    for line in lines {
        let line = line.trim();
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').to_string();
            labels.insert(label, pc);
        } else if let Some(instr) = parse_line(line) {
            pc += instr.length() as u16;
        }
    }

    labels
}

pub fn assemble(text: &str) -> Vec<u8> {
    let lines: Vec<&str> = text
        .lines()
        .map(|line| line.split(';').next().unwrap().trim()) // strip comments
        .filter(|line| !line.is_empty())
        .collect();    

    let labels = create_label_map(&lines);

    let program: Vec<Instruction> = lines
        .iter()
        .filter(|line| !line.ends_with(':'))
        .filter_map(|line| parse_line(line))
        .map(|mut instr| match &mut instr {
            Instruction::JMP { addr }
            | Instruction::JZ { addr }
            | Instruction::JC { addr } 
            | Instruction::JSR { addr } => {
                if let Address::Label(name) = addr {
                    *addr = Address::Addr(*labels.get(name).expect(&format!("unknown label: {}", name)));
                }
                instr
            }
            _ => instr
        })
        .collect();

    program
        .iter()
        .flat_map(|i| i.encode()).collect()
}