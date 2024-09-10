# CPU simulation

This project aims to be a simplified version of the Intel 8088, 16-bit CPU.


## Roadmap

 - [ ] [Base](#Base-Instructions) set
 - [ ] [Syscalls](#Syscalls)
 - [ ] [Graphics](#Graphics-Instructions) set

## Registers

| Register | Type |
| -------- | ---- |
| A        | General |
| B        | General |
| C        | General |
| IP       | Instruction Pointer |

## Instruction set

NOTE:
- `const` is a constant.
- `reg` is a register.
- `mem` is a memory address.
- `tag` is a tag in the source code.

> Every operation that modifies a value must be done within registers.
> Eventually the compiler will allow for certain accomodations such as adding a value from memory.

### Base Instructions

| instruction | description | syntax |
| ----------- | ----------- | ------ |
| ld          | Loads a value into either memory or a register | ld `<const>` `<reg/mem>` |
| sum         | Adds the values of a and b and stores the result in b. | sum `<reg a>` `<reg b>` |
| sub         | Subtracts the values of a and b and stores the result b. | sub `<reg a>` `<reg b>`|
| mul         | Multiplies the values of a and b and stores the result b. | mul `<reg a >` `<reg b>` |
| div         | Divides the values of a and b and stores the result b. | div `<reg a>` `<reg b>` |
||||
<!-- | push        | Pushes a value into the stack and increments the SP | push `<reg/const>` | -->
<!-- | pop         | Pops the last value from the stack and decrements SP | pop `<reg>` | -->
<!-- | call        | Pushes current instruction pointer to the stack and jumps to `tag`. | call `<tag>` | -->
<!-- | ret         | Pops value from stack and loads it into the instruction register | ret | -->
<!-- |||| -->
<!-- | jmp | Inconditional jump | jmp `<dest>` | -->
<!-- | jeq | Jump if equal | jeq `<val>` `<val>` `<dest>` | -->
<!-- | jne | Jump if not equal | jne `<val>` `<val>` `<dest>` | -->
<!-- | jgt | Jump if greater than | jgt `<val>` `<val>` `<dest>` | -->
<!-- | jlt | Jump if less than | jlt `<val>` `<val>` `<dest>` | -->

### Graphics Instructions
Unimplemented.

## Flags

Just like the 8088, this processor has a byte dedicated to storing flags.
<!-- - Carry flag : carry on arithmetic -->
<!-- - Sign       : to indicate whether a value is signed or unsigned -->
- Overflow   : indicate if theres overflow after some arithmetics
<!-- - Parity     : indicates whether a given number is odd or even -->
- Zero       : indicates division by zero

## Syscalls
