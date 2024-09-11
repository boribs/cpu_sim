# CPU simulation

A 16-bit simulated CPU, inspired by the Intel 8088.


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
| D        | General |
| IP       | Instruction Pointer |

## Instruction set

NOTE:
- `const` is a constant.
- `reg` is a register.
- `mem` is a memory address.

> Every operation that modifies a value must be done within registers.
> Eventually the compiler will allow for certain accomodations such as adding a value from memory.

### Base Instructions

| instruction | description | syntax |
| ----------- | ----------- | ------ |
| ld          | Loads a value into either memory or a register | ld `<const>` `<reg/mem>` |
| sum         | Adds the values of a and b and stores the result in b | sum `<reg a>` `<reg b>` |
| sub         | Subtracts the values of a and b and stores the result b | sub `<reg a>` `<reg b>`|
| mul         | Multiplies the values of a and b and stores the result b | mul `<reg a>` `<reg b>` |
| div         | Divides the values of a and b and stores the result b | div `<reg a>` `<reg b>` |
||||
| and         | Binary and between a and b, stored into b | and `<reg a>` `<reg b>` |
| or          | Binary or between a and b, stored into b | or `<reg a>` `<reg b>` |
||||
| cmp         | Compares two values and sets respective comparative flags | cmp `<reg a>` `<reg b>` |
| jmp         | Inconditional jump | jmp `<reg/const>` |
| jeq         | Jump if equal | jeq `<reg/const>` |
| jne         | Jump if not equal | jne `<reg/const>` |
| jgt         | Jump if greater than | jgt `<reg/const>` |
| jlt         | Jump if lower than | jlt `<reg/const>` |
<!-- | push        | Pushes a value into the stack and increments the SP | push `<reg/const>` | -->
<!-- | pop         | Pops the last value from the stack and decrements SP | pop `<reg>` | -->
<!-- | call        | Pushes current instruction pointer to the stack and jumps to `tag`. | call `<tag>` | -->
<!-- | ret         | Pops value from stack and loads it into the instruction register | ret | -->
<!-- |||| -->

### Graphics Instructions
Unimplemented.

## Flags

Just like the 8088, this processor has a byte dedicated to storing flags.
<!-- - Carry flag : carry on arithmetic -->
<!-- - Sign       : to indicate whether a value is signed or unsigned -->
- Overflow   : indicate if theres overflow after some arithmetics
<!-- - Parity     : indicates whether a given number is odd or even -->
- Zero       : indicates division by zero
- Equal      : indicates if last comparison was with equal values
- Greater than : indicates if in the last comparison, the first value was greater than the other
- Less than : indicates if in the last comparison, the first value was less than the other

## Syscalls
