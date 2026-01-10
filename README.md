# Rexta CPU Simulation & Assembler

The Rexta CPU is a simple **8-bit CPU** with 24-bit addressing designed mainly for experimentation and learning about CPU architectures. 

It features:

- 9 general-purpose 8-bit registers (`R0`–`R8`)
- 24-bit program counter (PC)
- 24-bit stack pointer (SP)
- Up to 16 MiB of addressable memory
- Basic ALU operations, load/store, jumps, and subroutine calls
- Carry and zero flags

The simulator in this repo is intended to model CPU behavior before potentially implementing it on hardware (FPGA or similar).

The assembler in this repo creates flat binary files which can be directly executed by the simulator or could eventually be loaded and run on the hardware itself.

---

## Features

- ALU instructions: `ADD`, `SUB`, `AND`, `OR`, `XOR`, `NOT`
- Immediate operations: `LOADI`, `ADDI`
- Memory operations: `LOAD`, `STORE`
- Program flow: `JMP`, `JZ`, `JC`, `JSR`, `RTS`
- Halt: `HLT`

---

## Instructions

For detailed information on the CPU itself and the instructions, see the documentation:

[Rexta CPU documentation](https://github.com/jonathan-gaul/rexta-docs/tree/main/CPU)

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

This should create a `test.b` file in the demo-files directory.

Simulate the demo file:
```bash
cargo run --bin rexta-sim demo-files/test.b 0x2000
```

You should see output from the simulation, for example:

```
Run successful
Value at 0x2000: 0x09
```