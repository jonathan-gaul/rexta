use crate::ast::Instruction;

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