mod ast;
mod assembler;

use std::{env, fs::{self, File}, io::Write, path::Path};
use crate::assembler::assemble;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("use: rexta-asm <file>");
        return;        
    }

    let source_path = Path::new(&args[1]);

    let program = fs::read_to_string(source_path).expect("unable to read source file");
  
    let bytes: Vec<u8> = assemble(program.as_str());
    
    let dest_path = source_path.with_extension("b");
    let mut dest_file = File::create(&dest_path).expect("failed to create output file");
    
    dest_file.write_all(&bytes).expect("failed to write binary data to file");
    
    println!("Wrote {} bytes to {}", bytes.len(), dest_path.display());
}