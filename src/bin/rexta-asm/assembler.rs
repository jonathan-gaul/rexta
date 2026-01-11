use std::collections::HashMap;

use rexta::u24::U24;

use crate::ast::Address;
use crate::ast::Instruction;
use crate::ast::Register;

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.opcode_bytes());

        bytes.extend_from_slice(&match self {
            Instruction::NOT1 { rd }
            | Instruction::NOT2 { rd }
            | Instruction::NOT3 { rd }
            | Instruction::INC1 { rd }
            | Instruction::INC2 { rd }
            | Instruction::INC3 { rd }
            | Instruction::DEC1 { rd }
            | Instruction::DEC2 { rd }
            | Instruction::DEC3 { rd }
            | Instruction::NEG1 { rd }
            | Instruction::NEG2 { rd }
            | Instruction::NEG3 { rd }
            | Instruction::SHL1 { rd }
            | Instruction::SHL2 { rd }
            | Instruction::SHL3 { rd }
            | Instruction::SHR1 { rd }
            | Instruction::SHR2 { rd }
            | Instruction::SHR3 { rd }
            | Instruction::ROL1 { rd }
            | Instruction::ROL2 { rd }
            | Instruction::ROL3 { rd }
            | Instruction::ROR1 { rd }
            | Instruction::ROR2 { rd }
            | Instruction::ROR3 { rd }
            | Instruction::POP1 { rd }
            | Instruction::POP2 { rd }
            | Instruction::POP3 { rd } => vec![rd.encode() << 4],

            Instruction::PUSH1 { rs } | Instruction::PUSH2 { rs } | Instruction::PUSH3 { rs } => {
                vec![rs.encode()]
            }

            Instruction::ADD1 { rd, rs }
            | Instruction::SUB1 { rd, rs }
            | Instruction::AND1 { rd, rs }
            | Instruction::OR1 { rd, rs }
            | Instruction::XOR1 { rd, rs }
            | Instruction::ADD2 { rd, rs }
            | Instruction::SUB2 { rd, rs }
            | Instruction::AND2 { rd, rs }
            | Instruction::OR2 { rd, rs }
            | Instruction::XOR2 { rd, rs }
            | Instruction::ADD3 { rd, rs }
            | Instruction::SUB3 { rd, rs }
            | Instruction::AND3 { rd, rs }
            | Instruction::OR3 { rd, rs }
            | Instruction::XOR3 { rd, rs }
            | Instruction::MOV1 { rd, rs }
            | Instruction::MOV2 { rd, rs }
            | Instruction::MOV3 { rd, rs }
            | Instruction::CMP1 { rd, rs }
            | Instruction::CMP2 { rd, rs }
            | Instruction::CMP3 { rd, rs }
            | Instruction::TST1 { rd, rs }
            | Instruction::TST2 { rd, rs }
            | Instruction::TST3 { rd, rs } => vec![rs.encode() | rd.encode() << 4],

            Instruction::LOADI1 { rd, imm } | Instruction::ADDI1 { rd, imm } => {
                vec![rd.encode() << 4, *imm]
            }

            Instruction::LOADI2 { rd, imm } | Instruction::ADDI2 { rd, imm } => {
                let [b1, b2] = imm.to_le_bytes();
                vec![rd.encode() << 4, b1, b2]
            }

            Instruction::LOADI3 { rd, imm } | Instruction::ADDI3 { rd, imm } => {
                let [b1, b2, b3] = imm.to_le_bytes();
                vec![rd.encode() << 4, b1, b2, b3]
            }

            Instruction::LOAD1 { rd, addr }
            | Instruction::LOAD2 { rd, addr }
            | Instruction::LOAD3 { rd, addr } => {
                if let Address::Addr(a) = addr {
                    let [b1, b2, b3] = a.to_le_bytes();
                    vec![rd.encode() << 4, b1, b2, b3]
                } else {
                    panic!("Label not resolved")
                }
            }

            Instruction::STORE1 { rs, addr }
            | Instruction::STORE2 { rs, addr }
            | Instruction::STORE3 { rs, addr } => {
                if let Address::Addr(a) = addr {
                    let [b1, b2, b3] = a.to_le_bytes();
                    vec![rs.encode(), b1, b2, b3]
                } else {
                    panic!("Label not resolved")
                }
            }

            Instruction::JMP { addr }
            | Instruction::JZ { addr }
            | Instruction::JC { addr }
            | Instruction::JNZ { addr }
            | Instruction::JNC { addr }
            | Instruction::JSR { addr }
            | Instruction::JMPA { addr }
            | Instruction::JZA { addr }
            | Instruction::JCA { addr }
            | Instruction::JNZA { addr }
            | Instruction::JNCA { addr }
            | Instruction::JSRA { addr } => {
                if let Address::Addr(a) = addr {
                    let [b1, b2, b3] = a.to_le_bytes();
                    vec![b1, b2, b3]
                } else {
                    panic!("Label not resolved")
                }
            }

            Instruction::RTS | Instruction::HLT => vec![],
        });

        bytes
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
        "R8" => Some(Register::R8),
        _ => None,
    }
}

fn parse_address(addr: &str) -> Option<Address> {
    if addr.starts_with("0x") || addr.starts_with(&['0','1','2','3','4','5','6','7','8','9']) {
        let value: U24 = addr.parse().ok()?;
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

    let base = parts[0].to_uppercase();
    let (opcode, width) = if let Some(pos) = base.find('.') {
        (&base[..pos], &base[pos + 1..])
    } else {
        (&base[..], "1") // default width
    };

    let parse_rd = || -> Option<Register> {
        if parts.len() < 2 {
            return None;
        }
        Some(parse_register(parts[1])?)
    };

    let parse_rs = || -> Option<Register> {
        if parts.len() < 2 {
            return None;
        }
        Some(parse_register(parts[1])?)
    };

    let parse_rd_rs = || -> Option<(Register, Register)> {
        if parts.len() < 3 {
            return None;
        }
        Some((parse_register(parts[1])?, parse_register(parts[2])?))
    };

    let parse_addr = || -> Option<Address> {
        if parts.len() < 2 {
            return None;
        }
        Some(parse_address(parts[1])?)
    };

    let parse_rd_addr = || -> Option<(Register, Address)> {
        if parts.len() < 3 {
            return None;
        }
        Some((parse_register(parts[1])?, parse_address(parts[2])?))
    };

    let parse_rd_imm1 = || -> Option<(Register, u8)> {
        if parts.len() < 3 {
            return None;
        }
        Some((parse_register(parts[1])?, parts[2].parse().ok()?))
    };

    let parse_rd_imm2 = || -> Option<(Register, u16)> {
        if parts.len() < 3 {
            return None;
        }
        Some((parse_register(parts[1])?, parts[2].parse().ok()?))
    };

    let parse_rd_imm3 = || -> Option<(Register, U24)> {
        if parts.len() < 3 {
            return None;
        }
        Some((parse_register(parts[1])?, parts[2].parse().ok()?))
    };

    match opcode {
        "ADD" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::ADD1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::ADD2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::ADD3 { rd, rs }))?,
            _ => None,
        },
        "SUB" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::SUB1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::SUB2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::SUB3 { rd, rs }))?,
            _ => None,
        },
        "AND" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::AND1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::AND2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::AND3 { rd, rs }))?,
            _ => None,
        },
        "OR" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::OR1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::OR2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::OR3 { rd, rs }))?,
            _ => None,
        },
        "XOR" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::XOR1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::XOR2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::XOR3 { rd, rs }))?,
            _ => None,
        },
        "MOV" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::MOV1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::MOV2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::MOV3 { rd, rs }))?,
            _ => None,
        },
        "CMP" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::CMP1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::CMP2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::CMP3 { rd, rs }))?,
            _ => None,
        },
        "TST" => match width {
            "1" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::TST1 { rd, rs }))?,
            "2" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::TST2 { rd, rs }))?,
            "3" => parse_rd_rs().map(|(rd, rs)| Some(Instruction::TST3 { rd, rs }))?,
            _ => None,
        },
        "NOT" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::NOT1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::NOT2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::NOT3 { rd }))?,
            _ => None,
        },
        "INC" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::INC1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::INC2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::INC3 { rd }))?,
            _ => None,
        },
        "DEC" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::DEC1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::DEC2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::DEC3 { rd }))?,
            _ => None,
        },
        "NEG" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::NEG1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::NEG2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::NEG3 { rd }))?,
            _ => None,
        },
        "SHL" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::SHL1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::SHL2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::SHL3 { rd }))?,
            _ => None,
        },
        "SHR" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::SHR1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::SHR2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::SHR3 { rd }))?,
            _ => None,
        },
        "ROL" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::ROL1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::ROL2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::ROL3 { rd }))?,
            _ => None,
        },
        "ROR" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::ROR1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::ROR2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::ROR3 { rd }))?,
            _ => None,
        },
        "POP" => match width {
            "1" => parse_rd().map(|rd| Some(Instruction::POP1 { rd }))?,
            "2" => parse_rd().map(|rd| Some(Instruction::POP2 { rd }))?,
            "3" => parse_rd().map(|rd| Some(Instruction::POP3 { rd }))?,
            _ => None,
        },
        "PUSH" => match width {
            "1" => parse_rs().map(|rs| Some(Instruction::PUSH1 { rs }))?,
            "2" => parse_rs().map(|rs| Some(Instruction::PUSH2 { rs }))?,
            "3" => parse_rs().map(|rs| Some(Instruction::PUSH3 { rs }))?,
            _ => None,
        },
        "LOAD" => match width {
            "1" => parse_rd_addr().map(|(rd, addr)| Some(Instruction::LOAD1 { rd, addr }))?,
            "2" => parse_rd_addr().map(|(rd, addr)| Some(Instruction::LOAD2 { rd, addr }))?,
            "3" => parse_rd_addr().map(|(rd, addr)| Some(Instruction::LOAD3 { rd, addr }))?,
            _ => None,
        },
        "STORE" => match width {
            "1" => parse_rd_addr().map(|(rs, addr)| Some(Instruction::STORE1 { rs, addr }))?,
            "2" => parse_rd_addr().map(|(rs, addr)| Some(Instruction::STORE2 { rs, addr }))?,
            "3" => parse_rd_addr().map(|(rs, addr)| Some(Instruction::STORE3 { rs, addr }))?,
            _ => None,
        },
        "LOADI" => match width {
            "1" => parse_rd_imm1().map(|(rd, imm)| Some(Instruction::LOADI1 { rd, imm }))?,
            "2" => parse_rd_imm2().map(|(rd, imm)| Some(Instruction::LOADI2 { rd, imm }))?,
            "3" => parse_rd_imm3().map(|(rd, imm)| Some(Instruction::LOADI3 { rd, imm }))?,
            _ => None,
        },
        "ADDI" => match width {
            "1" => parse_rd_imm1().map(|(rd, imm)| Some(Instruction::ADDI1 { rd, imm }))?,
            "2" => parse_rd_imm2().map(|(rd, imm)| Some(Instruction::ADDI2 { rd, imm }))?,
            "3" => parse_rd_imm3().map(|(rd, imm)| Some(Instruction::ADDI3 { rd, imm }))?,
            _ => None,
        },
        "JMP" => parse_addr().map(|addr| Some(Instruction::JMP { addr }))?,
        "JZ" => parse_addr().map(|addr| Some(Instruction::JZ { addr }))?,
        "JC" => parse_addr().map(|addr| Some(Instruction::JC { addr }))?,
        "JSR" => parse_addr().map(|addr| Some(Instruction::JSR { addr }))?,
        "JNZ" => parse_addr().map(|addr| Some(Instruction::JNZ { addr }))?,
        "JNC" => parse_addr().map(|addr| Some(Instruction::JNC { addr }))?,

        "JMPA" => parse_addr().map(|addr| Some(Instruction::JMPA { addr }))?,
        "JZA" => parse_addr().map(|addr| Some(Instruction::JZA { addr }))?,
        "JCA" => parse_addr().map(|addr| Some(Instruction::JCA { addr }))?,
        "JSRA" => parse_addr().map(|addr| Some(Instruction::JSRA { addr }))?,
        "JNZA" => parse_addr().map(|addr| Some(Instruction::JNZA { addr }))?,
        "JNCA" => parse_addr().map(|addr| Some(Instruction::JNCA { addr }))?,

        "RTS" => Some(Instruction::RTS),
        "HLT" => Some(Instruction::HLT),
        _ => None,
    }
}

fn create_label_map(lines: &Vec<&str>) -> HashMap<String, U24> {
    let mut labels = HashMap::new();
    let mut pc = U24::new(0);

    for line in lines {
        let line = line.trim();
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').to_string();
            labels.insert(label, pc);
        } else if let Some(instr) = parse_line(line) {
            pc += instr.length() as u32;
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
        .map(|instr| {
            println!("  {:?} => {:?}", instr, instr.encode());
            instr
        })
        .map(|mut instr| match &mut instr {
            Instruction::JMP { addr }
            | Instruction::JZ { addr }
            | Instruction::JC { addr }
            | Instruction::JSR { addr } => {
                if let Address::Label(name) = addr {
                    *addr = Address::Addr(
                        *labels.get(name).expect(&format!("unknown label: {}", name)),
                    );
                }
                instr
            }
            _ => instr,
        })
        .collect();

    program.iter().flat_map(|i| i.encode()).collect()
}
