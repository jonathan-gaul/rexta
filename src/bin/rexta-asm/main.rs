mod ast;
mod assembler;

use ast::Instruction;
use ast::Register;

fn main() {
    let i = Instruction::STORE { rd: Register::R3, addr: 0x2000 };
    
    let v = i.encode();

    println!("Encoded instruction:");

    for n in v {
        println!("0x{0:02X}", n);
    }
}