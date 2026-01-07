
use rexta::cpu::Cpu;
use rexta::cpu::CpuError;

fn main() {
    let program: [u8; 13] = [
        0x30, 0x00, 0x0A,       // LOADI R0, 10
        0x30, 0x01, 0x14,       // LOADI R1, 20
        0x20, 0x10,             // ADD R0, R1
        0x41, 0x00, 0x20, 0x00, // STORE R0, 0x2000
        0x02                    // HLT
    ];

    let mut cpu = Cpu::new();
    cpu.mem[0..program.len()].copy_from_slice(&program);
    
    match cpu.run() {
        Ok(()) => {
            println!("Run successful");
            println!("Value at 0x2000: {0}", cpu.mem_read(0x2000));
        }
        Err(CpuError::InvalidInstruction) => {
            println!("Invalid instruction: PC={0:4X}", cpu.pc);

        }
        Err(CpuError::InvalidOpCode(code)) => {
            println!("Invalid opcode {0}: PC={1:4X}", code, cpu.pc);
        }
    }
}
