# CPU simulation

This project aims to be a simplified version of the Intel 8088, 16-bit CPU.


## Roadmap

 - [ ] [Base](#Base-Instructions) set
 - [ ] [Syscalls](#Syscalls)
 - [ ] [Graphics](#Graphics-Instructions) set

## Instruction set

### Base Instructions

| instruction | description | syntax |
| ----------- | ----------- | ------ |
| ld          | Loads a value into a location in memory | ld `<val>` `<dest>` |
| sum         | Sums two values and stores the result into `dest`. | sum `<val>` `<val>` `<dest>` |
| sub         | Subtracts two values and stores the result into `dest`. | sub `<val>` `<val>` `<dest>` |
| mul         | Multiplies two values and stores the result into `dest`. | mul `<val>` `<val>` `<dest>` |
| div         | Divides two values and stores the result into `dest`. | div `<val>` `<val>` `<dest>` |
||||
| push        | Pushes a `val` into the stack and increments the SP | push `<val>` |
| pop         | Pops the last `val` from the stack and decrements SP | pop `<val>` |
| call        | Pushes current instruction pointer to the stack and jumps to `tag`. | call `<tag>` |
||||
| cmp | ... |
| jeq | ... |
| jne | ... |
| jgt | ... |
| jlt | ... |

### Graphics Instructions
Unimplemented.

## Flags

Just like the 8088, this processor has a byte dedicated to storing flags

## Syscalls
