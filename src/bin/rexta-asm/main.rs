mod ast;
mod assembler;

use crate::assembler::assemble;

fn main() {

    let program = "
        LOADI R0, 10
        LOADI R1, 20
        ADD R0, R1
        STORE R0, 0x2000
        HLT
    ";
    
    let v: Vec<u8> = assemble(program);
    
    println!("Encoded program:");

    for (i, n) in v.iter().enumerate() {
        println!("0x{0:04X}: 0x{1:02X}", i, n);
    }
}