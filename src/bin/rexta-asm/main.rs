mod ast;
mod assembler;

use ast::Instruction;
use ast::Register;

fn main() {

    let program: [Instruction; 5] = [
        Instruction::LOADI { rd: Register::R0, imm: 10 },
        Instruction::LOADI { rd: Register::R1, imm: 20 },
        Instruction::ADD { rd: Register::R0, rs: Register::R1 },
        Instruction::STORE { rd: Register::R0, addr: 0x2000 },
        Instruction::HLT,
    ];

    let v: Vec<u8> = program.iter().flat_map(|i| i.encode()).collect();
    
    println!("Encoded program:");

    for (i, n) in v.iter().enumerate() {
        println!("0x{0:04X}: 0x{1:02X}", i, n);
    }
}