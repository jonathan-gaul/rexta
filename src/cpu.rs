
use crate::u24::U24;
use crate::op::Op;
use crate::op::OpCode;

/// Represents the current state of a CPU.
pub struct Cpu {
    /// Program Counter
    pub pc: U24,

    /// Addressable memory (up to 16 MiB) - default to 64KiB
    pub mem: [u8; 65536],

    /// Registers
    pub regs: [u8; 9],

    /// Flags
    pub flags: u8,

    /// Stack Pointer
    pub sp: U24,

    /// True if the CPU is currently executing instructions.
    pub is_running: bool,

    /// Instruction Register (current opcode)
    pub ir: u16,

    /// Instruction Counter
    pub ic: U24,
}

pub enum CpuError {
    InvalidOpCode(u16),
    InvalidInstruction,
}

impl Cpu {

    pub const FLAG_ZERO: u8 = 0x01;
    pub const FLAG_CARRY: u8 = 0x02;

    /// Construct a new CPU with 64kb RAM,
    /// the stack pointer set to the end of RAM,
    /// and registers< PC etc set to 0.
    pub fn new() -> Self {
        Cpu {
            pc: U24::new(0),
            mem: [0; 65536],
            regs: [0; 9],
            flags: 0,
            sp: U24::new(0xFFFE),

            is_running: false,

            ir: 0,
            ic: U24::new(0),
        }
    }

    /// Read a value from memory with the given address.
    pub fn mem_read(&self, addr: U24) -> u8 {
        self.mem[addr.value() as usize]
    }

    /// Write a byte to memory at the given address.
    pub fn mem_write(&mut self, addr: U24, val: u8) {
        self.mem[addr.value() as usize] = val;
    }

    /// Write two bytes to memory at the given address.
    pub fn mem_write2(&mut self, addr: U24, val: u16) {
        let bytes = val.to_le_bytes();
        let pos = addr.value() as usize;
        self.mem[pos..pos+2].copy_from_slice(&bytes);
    }

    /// Write three bytes to memory at the given address.
    pub fn mem_write3(&mut self, addr: U24, val: U24) {
        let bytes = val.to_le_bytes();
        let pos = addr.value() as usize;
        self.mem[pos..pos+3].copy_from_slice(&bytes);
    }

    /// Read a value from the given register.
    pub fn reg_read(&self, reg: u8) -> u8 {
        self.regs[reg as usize]
    }

    // Read two bytes from register & register+1
    pub fn reg_read2(&self, reg: u8) -> u16 {
        (self.regs[reg as usize + 1] as u16) << 8
        | self.regs[reg as usize] as u16
    }

    /// Read 3 bytes from register & register+1 & register+2
    pub fn reg_read3(&self, reg: u8) -> U24 {
        let pos = reg as usize;

        let bytes: [u8; 3] = self.regs[pos..pos + 3]
            .try_into()
            .expect("out of bounds read");

        U24::from_le_bytes(bytes)
    }

    /// Write a value to the given register.
    pub fn reg_write(&mut self, reg: u8, val: u8) {
        self.regs[reg as usize] = val;
    }

    /// Write a 16-bit value to register & register+1
    pub fn reg_write2(&mut self, reg: u8, val: u16) {
        self.regs[reg as usize] = (val & 0xFF) as u8;
        self.regs[reg as usize + 1] = ((val & 0xFF00) >> 8) as u8;
    }

    pub fn reg_write3(&mut self, reg: u8, val: U24) {
        let bytes = val.to_le_bytes();
        println!("reg_write3: {:?} @ {:?}", bytes, reg);
        let pos = reg as usize;
        self.regs[pos..pos+3].copy_from_slice(&bytes);
    }

    /// Determine whether the given flag is set.
    pub fn flag_read(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    /// Set or clear the given flag.
    pub fn flag_write(&mut self, flag: u8, val: bool) {
        if val {
            self.flags = self.flags | flag
        }
        else {
            self.flags = self.flags & !flag
        }
    }

    /// Fetch the opcode at the current memory location (pointed to by PC) and increase the program counter by 2.
    fn fetch(&mut self) {
        let pos = self.pc.value() as usize;
        self.ir = u16::from_le_bytes(self.mem[pos..pos + 2].try_into().expect("Out of bounds"));
        self.pc += 2;
    }

    /// Decode the current opcode, retrieving required parameters.
    fn decode(&mut self) -> Result<Op, CpuError> {
        let operand_count = ((self.ir & 0xE00) >> 9) as usize;

        let op_code = OpCode::try_from(self.ir)
            .map_err(|_| CpuError::InvalidOpCode(self.ir))?;

        let mut op = Op { code: op_code, ..Op::new() };

        for i in 0..operand_count {
            op.operands[i] = self.mem_read(self.pc);
            self.pc += 1;
        }

        Ok(op)
    }

    /// Execute the given operation on the CPU.
    fn execute(&mut self, op: Op) -> Result<(), CpuError> {
        match op.code {
            OpCode::NOP => Ok(()),

            OpCode::RTS => {
                // Pop address from stack
                self.sp += 2;
                let addr =
                    U24::new(self.mem_read(self.sp - 2) as u32) << 16
                    | U24::new(self.mem_read(self.sp - 1) as u32) << 8
                    | U24::new(self.mem_read(self.sp) as u32);

                // Jump to address
                self.pc = addr;
                Ok(())
            }

            OpCode::HLT => {
                self.is_running = false;
                Ok(())
            }

            // ----------------------------------------
            // ADD
            // ----------------------------------------

            OpCode::ADD1 => {
                let value = self.reg_read(op.rd()) as u16 + self.reg_read(op.rs()) as u16;
                self.reg_write(op.rd(), value as u8);
                self.flag_write(Cpu::FLAG_ZERO, (value as u8) == 0);
                self.flag_write(Cpu::FLAG_CARRY, value & 0x100 != 0);
                Ok(())
            },

            OpCode::ADD2 => {
                let value: u32 = self.reg_read2(op.rd()) as u32 + self.reg_read2(op.rs()) as u32;
                self.reg_write2(op.rd(), value as u16);
                self.flag_write(Cpu::FLAG_ZERO, value as u16 == 0);
                self.flag_write(Cpu::FLAG_CARRY, value & 0x10000 != 0);
                Ok(())
            }

            OpCode::ADD3 => {
                let lhs: u32 = self.reg_read3(op.rd()).into();
                let rhs: u32 = self.reg_read3(op.rs()).into();
                let value = lhs + rhs;
                self.reg_write3(op.rd(), U24::new(value));
                self.flag_write(Cpu::FLAG_ZERO, value & 0xFFFFFF == 0);
                self.flag_write(Cpu::FLAG_CARRY, value & 0x1000000 != 0);
                Ok(())
            }

            // ----------------------------------------
            // SUB
            // ----------------------------------------

            OpCode::SUB1 => {
                let rdv: u16 = self.reg_read(op.rd()) as u16;
                let rsv: u16 = self.reg_read(op.rs()) as u16;
                let value: u16 = rdv - rsv;
                self.reg_write(op.rd(), value as u8);
                self.flag_write(Cpu::FLAG_ZERO, (value as u8) == 0);
                self.flag_write(Cpu::FLAG_CARRY, rdv < rsv);
                Ok(())
            }

            OpCode::SUB2 => {
                let rdv: u32 = self.reg_read2(op.rd()) as u32;
                let rsv: u32 = self.reg_read2(op.rs()) as u32;
                let value: u32 = rdv - rsv;
                self.reg_write2(op.rd(), value as u16);
                self.flag_write(Cpu::FLAG_ZERO, (value as u16) == 0);
                self.flag_write(Cpu::FLAG_CARRY, rdv < rsv);
                Ok(())
            }

            OpCode::SUB3 => {
                let rdv: u32 = self.reg_read3(op.rd()).into();
                let rsv: u32 = self.reg_read3(op.rs()).into();
                let value: U24 = U24::new(rdv - rsv);
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, rdv < rsv);
                Ok(())
            }

            // ----------------------------------------
            // AND
            // ----------------------------------------

            OpCode::AND1 => {
                let value: u8 = self.reg_read(op.rd()) & self.reg_read(op.rs());
                self.reg_write(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::AND2 => {
                let value: u16 = self.reg_read2(op.rd()) & self.reg_read2(op.rs());
                self.reg_write2(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::AND3 => {
                let value: U24 = self.reg_read3(op.rd()) & self.reg_read3(op.rs());
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            // ----------------------------------------
            // OR
            // ----------------------------------------

            OpCode::OR1 => {
                let value: u8 = self.reg_read(op.rd()) | self.reg_read(op.rs());
                self.reg_write(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::OR2 => {
                let value: u16 = self.reg_read2(op.rd()) | self.reg_read2(op.rs());
                self.reg_write2(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::OR3 => {
                let value: U24 = self.reg_read3(op.rd()) | self.reg_read3(op.rs());
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            // ----------------------------------------
            // XOR
            // ----------------------------------------

            OpCode::XOR1 => {
                let value: u8 = self.reg_read(op.rd()) ^ self.reg_read(op.rs());
                self.reg_write(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::XOR2 => {
                let value: u16 = self.reg_read2(op.rd()) ^ self.reg_read2(op.rs());
                self.reg_write2(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::XOR3 => {
                let value: U24 = self.reg_read3(op.rd()) ^ self.reg_read3(op.rs());
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            // ----------------------------------------
            // NOT
            // ----------------------------------------

            OpCode::NOT1 => {
                let value: u8 = !self.reg_read(op.rd());
                self.reg_write(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::NOT2 => {
                let value: u16 = !self.reg_read2(op.rd());
                self.reg_write2(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            OpCode::NOT3 => {
                let value: U24 = !self.reg_read3(op.rd());
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            }

            // ----------------------------------------
            // LOADI
            // ----------------------------------------

            OpCode::LOADI1 => {
                let imm = op.read_op(1);
                self.reg_write(op.rd(), imm);
                self.flag_write(Cpu::FLAG_ZERO, imm == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            },

            OpCode::LOADI2 => {
                let imm: u16 = op.read_op2(1);
                self.reg_write2(op.rd(), imm);
                self.flag_write(Cpu::FLAG_ZERO, imm == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            },

            OpCode::LOADI3 => {                
                let imm: U24 = op.read_op3(1);
                self.reg_write3(op.rd(), imm);
                self.flag_write(Cpu::FLAG_ZERO, imm == 0);
                self.flag_write(Cpu::FLAG_CARRY, false);
                Ok(())
            },

            // ----------------------------------------
            // ADDI
            // ----------------------------------------

            OpCode::ADDI1 => {
                let value: u16 = self.reg_read(op.rd()) as u16 + op.read_op(1) as u16;
                self.reg_write(op.rd(), (value & 0xFF) as u8);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x100) != 0);
                Ok(())
            }

            OpCode::ADDI2 => {
                let value: u32 = self.reg_read2(op.rd()) as u32 + op.read_op2(1) as u32;
                self.reg_write2(op.rd(), (value & 0xFFFF) as u16);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFFFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x10000) != 0);
                Ok(())
            }

            OpCode::ADDI3 => {
                let mut value: u32 = self.reg_read3(op.rd()).into();
                value += op.read_op3(1).as_u32();
                self.reg_write3(op.rd(), U24::new(value));
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFFFFFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x1000000) != 0);
                Ok(())
            }

            // ----------------------------------------
            // INC
            // ----------------------------------------

            OpCode::INC1 => {
                let value: u16 = self.reg_read(op.rd()) as u16 + 1;
                self.reg_write(op.rd(), (value & 0xFF) as u8);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x100) != 0);
                Ok(())
            }

            OpCode::INC2 => {
                let value: u32 = self.reg_read2(op.rd()) as u32 + 1;
                self.reg_write2(op.rd(), (value & 0xFFFF) as u16);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFFFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x10000) != 0);
                Ok(())
            }

            OpCode::INC3 => {
                let mut value: u32 = self.reg_read3(op.rd()).into();
                value += 1;
                self.reg_write3(op.rd(), U24::new(value));
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFFFFFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0x1000000) != 0);
                Ok(())
            }

            // ----------------------------------------
            // DEC
            // ----------------------------------------

            OpCode::DEC1 => {
                let value: u16 = self.reg_read(op.rd()) as u16 - 1;
                self.reg_write(op.rd(), (value & 0xFF) as u8);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0xFF) == 0xFF);
                Ok(())
            }

            OpCode::DEC2 => {
                let value: u32 = self.reg_read2(op.rd()) as u32 - 1;
                self.reg_write2(op.rd(), (value & 0xFFFF) as u16);
                self.flag_write(Cpu::FLAG_ZERO, (value & 0xFFFF) == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0xFFFF) == 0xFFFF);
                Ok(())
            }

            OpCode::DEC3 => {
                let value = self.reg_read3(op.rd()) - 1;
                self.reg_write3(op.rd(), value);
                self.flag_write(Cpu::FLAG_ZERO, value == 0);
                self.flag_write(Cpu::FLAG_CARRY, (value & 0xFFFFFF) == 0xFFFFFF);
                Ok(())
            }

            // ----------------------------------------
            // JMP
            // ----------------------------------------

            OpCode::JMP => {
                self.pc = U24::new(
                    op.operands[0] as u32 |
                    (op.operands[1] as u32) << 8 |
                    (op.operands[2] as u32) << 16);
                Ok(())
            },

            OpCode::JZ => {
                if self.flag_read(Cpu::FLAG_ZERO) {
                    self.pc = U24::new(
                    op.operands[0] as u32 |
                    (op.operands[1] as u32) << 8 |
                    (op.operands[2] as u32) << 16);
                }
                Ok(())
            },

            // ----------------------------------------
            // STORE
            // ----------------------------------------

            OpCode::STORE1 => {
                self.mem_write(op.read_op3(1),self.reg_read(op.rs()));
                Ok(())
            }
            OpCode::STORE2 => {
                self.mem_write2(op.read_op3(1), self.reg_read2(op.rs()));
                Ok(())
            }
            OpCode::STORE3 => {                
                self.mem_write3(op.read_op3(1), self.reg_read3(op.rs()));
                Ok(())
            }

            _ => {
                panic!("OpCode not implemented")
            }

        }
    }

    /// Execute a single tick (clock cycle) for this
    /// CPU.
    fn tick(&mut self) -> Result<(), CpuError> {
        self.fetch();
        let op = self.decode()?;
        self.execute(op)?;
        Ok(())
    }

    pub fn halt(&mut self) {
        println!("CPU halted!");
        self.is_running = false;
    }

    /// Run the CPU until a HLT instruction is reached
    /// or an error occurs, starting at the current PC.
    pub fn run(&mut self) -> Result<(), CpuError> {
        self.ic = U24::new(0);
        self.is_running = true;
        while self.is_running {
            self.tick()?;
            self.ic += 1;
        }
        Ok(())
    }
}