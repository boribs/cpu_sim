use cpu_sim::*;

/// A simple program that creates the instructions for the CPU to write the string
/// "Hello, World!\0" in memory. Then checks if it's writter correctly.
#[test]
fn hello_world_without_syscalls_or_assembler() {
    let mut mem = cpu::Mem::new(300);

    // create instructions to write to memory
    let original_msg = "Hello, World!\0";
    let mut instructions: Vec<cpu::Instruction> = vec![];

    original_msg.chars()
    .enumerate()
    .for_each(|(i, c)| {
        instructions.push(cpu::Instruction::Set(c as u16, cpu::Reg::A));
        instructions.push(cpu::Instruction::Ld(
            cpu::CR::Register(cpu::Reg::AL),
            cpu::CR::Constant(200 + i as u16)
        ));
    });

    // write them to memory
    let mut index = 0;
    instructions.iter().for_each(|instr| {
        let [num_bits, instr_code, a_high, a_low, b_high, b_low] = instr.to_bytes();
        let mut num_bytes = num_bits / 8;

        mem.write(index, instr_code);
        mem.write(index + 1, a_high);
        mem.write(index + 2, a_low);
        num_bytes -= 3;
        index += 3;

        if num_bytes > 0 {
            mem.write(index, b_high);
            num_bytes -= 1;
            index += 1;
        }
        if num_bytes > 0 {
            mem.write(index, b_low);
            index += 1;
        }
    });

    // execute them
    let mut cpu = cpu::Cpu::default();
    let mut index = 0;

    loop {
        if mem.read(index) == 0 {
            break;
        }

        let instr = cpu::Instruction::from_mem(&mem, index);
        if instr.is_err() {
            break;
        }

        let (instr, offset) = instr.unwrap();
        cpu.execute(instr, &mut mem);
        index += offset;
    }

    // both strings match!
    let msg = String::from_iter(mem.array.iter()
        .skip(200)
        .take_while(|val| **val != 0)
        .map(|c| *c as u8 as char)
    ) + "\0";

    assert_eq!(original_msg, msg)
}
