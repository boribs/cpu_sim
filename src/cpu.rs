pub enum Dest {
    Memory(u16),
    RegA,
    RegB,
    RegC,
}

pub enum Instruction {
    Ld(i16, Dest),
    Sum(i16, i16, Dest),
    Sub(i16, i16, Dest),
    Mul(i16, i16, Dest),
    Div(i16, i16, Dest),
}

pub struct Cpu {
    a: i16,
    b: i16,
    c: i16,
    flags: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            flags: 0,
        }
    }
}

impl Cpu {
    pub const FLAG_OVERFLOW: u8 = 0b0001;

    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Ld(val, dest) => match dest {
                Dest::Memory(_) => todo!(),
                Dest::RegA => self.a = val,
                Dest::RegB => self.b = val,
                Dest::RegC => self.c = val,
            },
            Instruction::Sum(a, b, dest) => {
                let sum;

                let checksum = a as i32 + b as i32;
                if checksum > std::i16::MAX as i32 || checksum < std::i16::MIN as i32 {
                    sum = 0;
                    self.flags |= Self::FLAG_OVERFLOW;
                    // TOOD: propagate warning
                } else {
                    sum = a + b;
                }

                match dest {
                    Dest::Memory(_) => todo!(),
                    Dest::RegA => self.a = sum,
                    Dest::RegB => self.b = sum,
                    Dest::RegC => self.c = sum,
                }
            },
            Instruction::Sub(a, b, dest) => {
                let sub;

                let checksub = a as i32 - b as i32;
                if checksub > std::i16::MAX as i32 || checksub < std::i16::MIN as i32 {
                    sub = 0;
                    self.flags |= Self::FLAG_OVERFLOW;
                    // TOOD: propagate warning
                } else {
                    sub = a - b;
                }

                match dest {
                    Dest::Memory(_) => todo!(),
                    Dest::RegA => self.a = sub,
                    Dest::RegB => self.b = sub,
                    Dest::RegC => self.c = sub,
                }
            },
            Instruction::Mul(a, b, dest) => {
                let mul;

                let checkmul = a as i32 * b as i32;
                if checkmul > std::i16::MAX as i32 || checkmul < std::i16::MIN as i32 {
                    mul = 0;
                    self.flags |= Self::FLAG_OVERFLOW;
                    // TOOD: propagate warning
                } else {
                    mul = a * b;
                }

                match dest {
                    Dest::Memory(_) => todo!(),
                    Dest::RegA => self.a = mul,
                    Dest::RegB => self.b = mul,
                    Dest::RegC => self.c = mul,
                }
            },
            Instruction::Div(a, b, dest) => {
                let div = a / b;

                // division cant be overflown

                match dest {
                    Dest::Memory(_) => todo!(),
                    Dest::RegA => self.a = div,
                    Dest::RegB => self.b = div,
                    Dest::RegC => self.c = div,
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
        assert_eq!(cpu.flags, 0);
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
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sum_within_16_bits() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sum(3000, 3100, Dest::RegA));
        cpu.execute(Instruction::Sum(3000, -3100, Dest::RegB));

        assert_eq!(cpu.a, 6100);
        assert_eq!(cpu.b, -100);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sum_with_overflow() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sum(32767, 4, Dest::RegA));

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & (1 >> Cpu::FLAG_OVERFLOW - 1) != 0);
    }

    #[test]
    fn sum_of_negatives_with_overflow() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sum(-32767, -4, Dest::RegA));

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & (1 >> Cpu::FLAG_OVERFLOW - 1) != 0);
    }

    #[test]
    fn sub_within_16_bits() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sub(3000, 3100, Dest::RegA));
        cpu.execute(Instruction::Sub(3000, -3100, Dest::RegB));

        assert_eq!(cpu.a, -100);
        assert_eq!(cpu.b, 6100);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sub_with_overflow() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Sub(-32767, 4, Dest::RegA));

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & (1 >> Cpu::FLAG_OVERFLOW - 1) != 0);
    }

    #[test]
    fn mul_within_16_bits() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Mul(4, -4, Dest::RegA));

        assert_eq!(cpu.a, -16);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn mul_with_overflow() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Mul(-32767, 32767, Dest::RegA));

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & (1 >> Cpu::FLAG_OVERFLOW - 1) != 0);
    }

    #[test]
    fn div() {
        let mut cpu = Cpu::default();
        cpu.execute(Instruction::Div(-32767, 32767, Dest::RegA));
        cpu.execute(Instruction::Div(4, 2, Dest::RegB));

        assert_eq!(cpu.a, -1);
        assert_eq!(cpu.b, 2);
        assert_eq!(cpu.flags, 0);
    }
}
