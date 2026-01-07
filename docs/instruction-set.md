# Instruction Set

## ALU Instructions

### ADD *Rd*, *Rs*
```
*Rd* <- *Rd* + *Rs*
```
*CARRY* flag is set if the operation overflows.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### SUB *Rd*, *Rs*
```
*Rd* <- *Rd* - *Rs*
```
*CARRY* flag is set if *Rd* < *Rs*.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### AND *Rd*, *Rs*
```
*Rd* <- *Rd* AND *Rs*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### OR *Rd*, *Rs*
```
*Rd* <- *Rd* OR *Rs*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### XOR *Rd*, *Rs*
```
*Rd* <- *Rd* XOR *Rs*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### NOT *Rd*
```
*Rd* <- NOT *Rd*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

## Immediate Instructions

### LOADI *Rd*, *imm*
```
*Rd* <- *imm*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### ADDI *Rd*, *imm*
```
*Rd* <- *Rd* + *imm*
```
*CARRY* flag is set if the operation overflows.
*ZERO* flag is set if *Rd* is 0 after the instruction.

## Memory Operations

### LOAD *Rd*, *addr*
```
*Rd* <- value at *addr*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

### STORE *Rd*, *addr*
```
*addr* <- *Rd*
```
*CARRY* flag is cleared.
*ZERO* flag is set if *Rd* is 0 after the instruction.

## Control Flow

### JMP *addr*
```
*PC* <- *addr*
```

### JZ *addr*
If ZERO flag is set: 
```
*PC* <- *addr*
```

### JC *addr*
If CARRY flag is set: 
```
*PC* <- *addr*
```

### JSR *addr*
Push PC onto STACK
```
*PC* <- *addr*
```

### RTS
Pop PC from STACK

## Halt

### HLT
End execution