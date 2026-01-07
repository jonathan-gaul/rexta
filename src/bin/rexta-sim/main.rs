use std::{env, fs, path::Path};

use rexta::cpu::{Cpu, CpuError};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("use: rexta-sim <file> [<addr>]");
        println!("simulate the file and output the value at <addr> if given");
        return;
    }

    let source_path = Path::new(&args[1]);
    let addr =
        if args.len() < 3 {
            None
        } else {
            Some(u16::from_str_radix(&args[2].trim_start_matches("0x"), 16).unwrap())
        };

    println!("Executing: {}", source_path.display());

    let program = fs::read(source_path).expect("unable to read program");

    let mut cpu = Cpu::new();
    cpu.mem[0..program.len()].copy_from_slice(&program);

    match cpu.run() {
        Ok(()) => {
            println!("Run successful");
            match addr {
                Some(addr) => println!("Value at 0x{0:04X}: 0x{1:02X}", addr, cpu.mem_read(addr)),
                None => {}
            }
            println!("Executed {} tick(s)", cpu.ic);
        }
        Err(CpuError::InvalidInstruction) => {
            println!("Invalid instruction: PC={0:04X}", cpu.pc);

        }
        Err(CpuError::InvalidOpCode(code)) => {
            println!("Invalid opcode {0}: PC={1:04X}", code, cpu.pc);
        }
    }
}