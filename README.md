# Rexta CPU Simulation

Rexta is a simple **8-bit CPU** designed mainly for experimentation and learning about CPU architectures. 

It features:

- 8 general-purpose 8-bit registers (R0–R7)
- 16-bit program counter (PC)
- 16-bit stack pointer (SP)
- 64 KB of RAM
- Basic ALU operations, load/store, jumps, and subroutine calls
- Carry and zero flags

The simulator in this repo is intended to model CPU behavior before potentially implementing it on hardware (FPGA or similar).

---

## Features

- ALU instructions: `ADD`, `SUB`, `AND`, `OR`, `XOR`, `NOT`
- Immediate operations: `LOADI`, `ADDI`
- Memory operations: `LOAD`, `STORE`
- Program flow: `JMP`, `JZ`, `JC`, `JSR`, `RTS`
- Halt: `HLT`

---

## Instruction Format

The first 4 bits of the opcode determine the operand layout:

| Opcode | Instruction     | Format                  | Notes                       |
|--------|----------------|------------------------|-----------------------------|
| 0x20   | ADD Rd, Rs     | Rd in lower 4 bits, Rs in upper 4 bits | Rd = Rd + Rs |
| 0x21   | SUB Rd, Rs     | Rd/Rs as above          | Rd = Rd - Rs               |
| 0x22   | AND Rd, Rs     | Rd/Rs as above          | Bitwise AND                |
| 0x23   | OR Rd, Rs      | Rd/Rs as above          | Bitwise OR                 |
| 0x24   | XOR Rd, Rs     | Rd/Rs as above          | Bitwise XOR                |
| 0x10   | NOT Rd         | Rd in lower 4 bits      | Bitwise NOT                |
| 0x30   | LOADI Rd, imm  | Rd in lower 4 bits, imm in next byte | Load immediate value |
| 0x31   | ADDI Rd, imm   | Rd in lower 4 bits, imm in next byte | Rd += imm |
| 0x40   | LOAD Rd, addr  | Rd in lower 4 bits, addr in next 2 bytes | Load from memory |
| 0x41   | STORE Rd, addr | Rd in lower 4 bits, addr in next 2 bytes | Store to memory |
| 0x50   | JMP addr       | Addr in next 2 bytes    | Jump unconditionally       |
| 0x51   | JZ addr        | Addr in next 2 bytes    | Jump if zero flag set      |
| 0x52   | JC addr        | Addr in next 2 bytes    | Jump if carry flag set     |
| 0x53   | JSR addr       | Addr in next 2 bytes    | Call subroutine            |
| 0x01   | RTS            | None                    | Return from subroutine     |
| 0x02   | HLT            | None                    | Halt execution             |

---

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

... or an editor that supports dev containers.

---

### Build and Run

Clone the repository:

```bash
git clone https://github.com/jonathan-gaul/rexta.git
cd rexta
```

Assemble the demo file:

```bash
cargo run --bin rexta-asm demo-files/test.rxa
```

Simulate the demo file:
```bash
cargo run --bin rexta-sim demo-files/test.b 0x2000
```

You should see output from the simulation, for example:

```
Run successful
Value at 0x2000: 30
```