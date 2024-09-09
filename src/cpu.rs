pub enum Dest {
    Memory(u16),
    RegA,
    RegB,
    RegC,
}

pub enum Instruction {
    Ld(i16, Dest),
    Sum(i16, i16, Dest),
}

pub struct Cpu {
    a: i16,
    b: i16,
    c: i16,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu { a: 0, b: 0, c: 0 }
    }
}

impl Cpu {
    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Ld(val, dest) => match dest {
                Dest::Memory(_) => todo!(),
                Dest::RegA => self.a = val,
                Dest::RegB => self.b = val,
                Dest::RegC => self.c = val,
            },
            Instruction::Sum(a, b, dest) => {
                let sum = a + b;
                // TODO: Deal with possible overflow
                // TOOD: propagate warning
                match dest {
                    Dest::Memory(_) => todo!(),
                    Dest::RegA => self.a = sum,
                    Dest::RegB => self.b = sum,
                    Dest::RegC => self.c = sum,
                }
            }
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn load_a() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Ld(-5, Dest::RegA));

        assert_eq!(cpu.a, -5);
    }

    #[test]
    fn load_abc() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Ld(-5, Dest::RegA));
        cpu.execute(Instruction::Ld(1, Dest::RegB));
        cpu.execute(Instruction::Ld(2020, Dest::RegC));

        assert_eq!(cpu.a, -5);
        assert_eq!(cpu.b, 1);
        assert_eq!(cpu.c, 2020);
    }

    #[test]
    fn sum_within_16_bits() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sum(3000, 3100, Dest::RegA));
        cpu.execute(Instruction::Sum(3000, -3100, Dest::RegB));

        assert_eq!(cpu.a, 6100);
        assert_eq!(cpu.b, -100);
    }
}
