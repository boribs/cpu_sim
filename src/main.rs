mod cpu;
use cpu::*;

fn main() {
    let mut cpu = Cpu::default();
    cpu.execute(Instruction::Ld(-5, Dest::RegA));
}
