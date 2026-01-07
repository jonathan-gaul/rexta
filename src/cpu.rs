
use crate::op::Op;
use crate::op::OpCode;
use crate::op::OpFormat;

/// Represents the current state of a CPU.
pub struct Cpu {
    /// Program Counter
    pub pc: u16,

    /// RAM (64KB)
    pub mem: [u8; 65536],

    /// Registers
    pub regs: [u8; 8],

    /// Flags
    pub flags: u8,

    /// Stack Pointer
    pub sp: u16,

    /// True if the CPU is currently executing instructions.
    pub is_running: bool,

    /// Instruction Register (current opcode)
    pub ir: u8,
}

pub enum CpuError {
    InvalidOpCode(u8),
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
            pc: 0,
            mem: [0; 65536],
            regs: [0; 8],
            flags: 0,
            sp: 0xFFFE,

            is_running: false,

            ir: 0,            
        }
    }

    /// Read a value from memory with the given 16-bit address.
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    /// Write a value to memory at the given 16-bit address.
    pub fn mem_write(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }

    /// Read a value from the given register.
    pub fn reg_read(&self, reg: u8) -> u8 {
        self.regs[reg as usize]
    }

    /// Write a value to the given register.
    pub fn reg_write(&mut self, reg: u8, val: u8) {
        self.regs[reg as usize] = val;
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

    /// Fetch the opcode at the current memory location (pointed to by PC) and increase the program counter by 1.
    fn fetch(&mut self) {
        self.ir = self.mem[self.pc as usize];
        self.pc += 1;
    }

    /// Decode the current opcode, retrieving required parameters.
    fn decode(&mut self) -> Result<Op, CpuError> {
        let format = OpFormat::try_from(self.ir >> 4)
            .map_err(|_| CpuError::InvalidOpCode(self.ir))?;

        let op_code = OpCode::try_from(self.ir)
            .map_err(|_| CpuError::InvalidOpCode(self.ir))?;        

        let op = Op { code: op_code, ..Op::new() };

        match format {
            OpFormat::None => 
                Ok(op),
            OpFormat::Rd => { // Rd in lower 4 bits
                let rd = self.mem_read(self.pc) & 0x0F;
                self.pc += 1;
                Ok(Op { rd: Some(rd), ..op })
            }
            OpFormat::RdRs => {
                let b = self.mem_read(self.pc);
                let rd = b & 0x0F;
                let rs = b >> 4;
                self.pc += 1;
                Ok(Op { rd: Some(rd), rs: Some(rs), ..op })
            }
            OpFormat::RdImm => {
                let rd = self.mem_read(self.pc) & 0x0F;
                self.pc += 1;
                let imm = self.mem_read(self.pc);
                self.pc += 1;
                Ok(Op { rd: Some(rd), imm: Some(imm), ..op })
            }
            OpFormat::RdAddr => {
                let rd = self.mem_read(self.pc) & 0x0F;
                self.pc += 1;
                let addr = (self.mem_read(self.pc) as u16) << 8 | self.mem_read(self.pc + 1) as u16;
                self.pc += 2;
                Ok(Op { rd: Some(rd), addr: Some(addr), ..op })
            }
            OpFormat::Addr => {
                let addr = (self.mem_read(self.pc) as u16) << 8 | self.mem_read(self.pc + 1) as u16;
                self.pc += 2;
                Ok(Op { addr: Some(addr), ..op })
            }
        }        
    }

    // Helper functions for handling missing operands.

    fn with_rd<F>(&mut self, op: &Op, f: F) -> Result<(), CpuError>
    where 
        F: FnOnce(u8, &mut Self) -> (),
    {
        if let Some(rd) = op.rd {
            f(rd, self);
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }

    fn with_rd_rs<F>(&mut self, op: &Op, f: F) -> Result<(), CpuError>
    where 
        F: FnOnce(u8, u8, &mut Self) -> (),
    {
        if let (Some(rd), Some(rs)) = (op.rd, op.rs) {
            f(rd, rs, self);
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }

    fn with_rd_imm<F>(&mut self, op: &Op, f: F) -> Result<(), CpuError> 
    where 
        F: FnOnce(u8, u8, &mut Self) -> (),
    {
        if let (Some(rd), Some(imm)) = (op.rd, op.imm) {
            f(rd, imm, self);
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }

    fn with_rd_addr<F>(&mut self, op: &Op, f: F) -> Result<(), CpuError>
    where 
        F: FnOnce(u8, u16, &mut Self) -> (),
    {
        if let (Some(rd), Some(addr)) = (op.rd, op.addr) {
            f(rd, addr, self);
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }

    fn with_addr<F>(&mut self, op: &Op, f: F) -> Result<(), CpuError>
    where 
        F: FnOnce(u16, &mut Self) -> (),
    {
        if let Some(addr) = op.addr {
            f(addr, self);
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }

    /// Execute the given operation on the CPU.
    fn execute(&mut self, op: Op) -> Result<(), CpuError> {
        match op.code {
            OpCode::NOP => Ok(()),

            OpCode::RTS => {
                // Pop address from stack
                self.sp += 2;
                let addr = (self.mem_read(self.sp - 1) as u16) << 8 | self.mem_read(self.sp) as u16;

                // Jump to address
                self.pc = addr;
                Ok(())
            }

            OpCode::HLT => {
                self.is_running = false;
                Ok(())
            }

            OpCode::ADD => self.with_rd_rs(&op, |rd, rs, cpu| {                    
                let value = cpu.reg_read(rd) as u16 + cpu.reg_read(rs) as u16;
                cpu.reg_write(rd, value as u8);
                cpu.flag_write(Cpu::FLAG_ZERO, (value as u8) == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, value & 0x100 != 0);
            }),    

            OpCode::SUB => self.with_rd_rs(&op, |rd, rs, cpu| {
                let rdv = cpu.reg_read(rd);
                let value = rdv as u16 - cpu.reg_read(rs) as u16;
                cpu.reg_write(rd, value as u8);
                cpu.flag_write(Cpu::FLAG_ZERO, (value as u8) == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, rdv < cpu.reg_read(rs));
            }),
            
            OpCode::AND => self.with_rd_rs(&op, |rd, rs, cpu| {                
                let value = cpu.reg_read(rd) & cpu.reg_read(rs);
                cpu.reg_write(rd, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);
            }),

            OpCode::OR => self.with_rd_rs(&op, |rd, rs, cpu| {                
                let value = cpu.reg_read(rd) | cpu.reg_read(rs);
                cpu.reg_write(rd, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);                    
            }),
                 
            OpCode::XOR => self.with_rd_rs(&op, |rd, rs, cpu| {                
                let value = cpu.reg_read(rd) ^ cpu.reg_read(rs);
                cpu.reg_write(rd, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);                    
            }),

            OpCode::NOT => self.with_rd(&op, |rd, cpu| {                
                let value = !cpu.reg_read(rd);
                cpu.reg_write(rd, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);
            }),

            OpCode::LOADI => self.with_rd_imm(&op, |rd, imm, cpu| {                
                cpu.reg_write(rd, imm);
                cpu.flag_write(Cpu::FLAG_ZERO, imm == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);                   
            }),

            OpCode::ADDI => self.with_rd_imm(&op, |rd, imm, cpu| {                
                let value = cpu.reg_read(rd) as u16 + imm as u16;
                cpu.reg_write(rd, value as u8);
                cpu.flag_write(Cpu::FLAG_ZERO, (value as u8) == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, value & 0x100 != 0);
            }),

            OpCode::LOAD => self.with_rd_addr(&op, |rd, addr, cpu| {                
                let value = cpu.mem_read(addr);
                cpu.reg_write(rd, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);
            }),

            OpCode::STORE => self.with_rd_addr(&op, |rd, addr, cpu| {                
                let value = cpu.reg_read(rd);
                cpu.mem_write(addr, value);
                cpu.flag_write(Cpu::FLAG_ZERO, value == 0);
                cpu.flag_write(Cpu::FLAG_CARRY, false);
            }),

            OpCode::JMP => self.with_addr(&op, |addr, cpu| {                
                cpu.pc = addr;
            }),                        

            OpCode::JZ => self.with_addr(&op, |addr, cpu| {                
                if cpu.flag_read(Cpu::FLAG_ZERO) {
                    cpu.pc = addr;                            
                }
            }),                    

            OpCode::JC => self.with_addr(&op, |addr, cpu| {                
                if cpu.flag_read(Cpu::FLAG_CARRY) {
                    cpu.pc = addr;
                }
            }),

            OpCode::JSR => self.with_addr(&op, |addr, cpu| {                
                // Push PC to stack
                cpu.mem_write(cpu.sp, cpu.pc as u8);
                cpu.mem_write(cpu.sp - 1, (cpu.pc >> 8) as u8);
                cpu.sp -= 2;
                // Jump
                cpu.pc = addr;
            }),

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

    // pub fn halt(&mut self) {
    //     println!("CPU halted!");
    //     self.is_running = false;
    // }

    /// Run the CPU until a HLT instruction is reached 
    /// or an error occurs, starting at the current PC.
    pub fn run(&mut self) -> Result<(), CpuError> {
        self.is_running = true;
        while self.is_running {
            self.tick()?;
        }
        Ok(())
    }
}