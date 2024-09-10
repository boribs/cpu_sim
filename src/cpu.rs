#[derive(Copy, Clone)]
pub enum Reg {
    A,
    B,
    C,
}

pub enum Dest {
    Memory(u16),
    Register(Reg),
}

pub enum Instruction {
    Ld(i16, Dest),
    Sum(Reg, Reg),
    Sub(Reg, Reg),
    // Mul(i16, Dest),
    // Div(i16, Dest),
}

pub struct Mem {
    array: Vec<i16>,
    // store eventually
    // - program pointer
    // - data pointer
    // - stack pointer
    // to be able to warn the user
}

impl Default for Mem {
    fn default() -> Self {
        let mut arr = Vec::<i16>::with_capacity(10);
        for _ in 0..10 {
            arr.push(0);
        }

        Mem { array: arr }
    }
}

impl Mem {
    pub fn read(&self, index: usize) -> i16 {
        assert!(index < self.array.capacity());
        self.array[index]
    }

    pub fn write(&mut self, index: usize, val: i16) {
        assert!(index < self.array.capacity());
        self.array[index] = val;
    }
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
    pub const FLAG_ZERO: u8 = 0b0010;

    pub fn reg_write(&mut self, reg: Reg, value: i16) {
        match reg {
            Reg::A => self.a = value,
            Reg::B => self.b = value,
            Reg::C => self.c = value,
        }
    }

    pub fn reg_read(&self, reg: Reg) -> i16 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
        }
    }

    pub fn execute(&mut self, instr: Instruction, mem: &mut Mem) {
        match instr {
            Instruction::Ld(val, dest) => match dest {
                Dest::Memory(i) => mem.write(i.into(), val),
                Dest::Register(r) => self.reg_write(r, val),
            },
            Instruction::Sum(a, b) => {
                let sum;

                {
                    let a = self.reg_read(a);
                    let b = self.reg_read(b);

                    let checksum = a as i32 + b as i32;
                    if checksum > std::i16::MAX as i32 || checksum < std::i16::MIN as i32 {
                        sum = 0;
                        self.flags |= Self::FLAG_OVERFLOW;
                        // TOOD: propagate warning
                    } else {
                        sum = a + b;
                    }
                }

                self.reg_write(b, sum);
            },
            Instruction::Sub(a, b) => {
                let sub;

                {
                    let a = self.reg_read(a);
                    let b = self.reg_read(b);

                    let checksub = a as i32 - b as i32;
                    if checksub > std::i16::MAX as i32 || checksub < std::i16::MIN as i32 {
                        sub = 0;
                        self.flags |= Self::FLAG_OVERFLOW;
                        // TOOD: propagate warning
                    } else {
                        sub = a - b;
                    }
                }

                self.reg_write(b, sub);
            },
        //     Instruction::Mul(a, b, dest) => {
        //         let mul;

        //         let checkmul = a as i32 * b as i32;
        //         if checkmul > std::i16::MAX as i32 || checkmul < std::i16::MIN as i32 {
        //             mul = 0;
        //             self.flags |= Self::FLAG_OVERFLOW;
        //             // TOOD: propagate warning
        //         } else {
        //             mul = a * b;
        //         }

        //         match dest {
        //             Dest::Memory(i) => mem.write(i.into(), mul),
        //             Dest::RegA => self.a = mul,
        //             Dest::RegB => self.b = mul,
        //             Dest::RegC => self.c = mul,
        //         }
        //     },
        //     Instruction::Div(a, b, dest) => {
        //         let div = if b != 0 {
        //             a / b
        //         } else {
        //             self.flags |= Self::FLAG_ZERO;
        //             0
        //         };

        //         // division can't be overflown

        //         match dest {
        //             Dest::Memory(i) => mem.write(i.into(), div),
        //             Dest::RegA => self.a = div,
        //             Dest::RegB => self.b = div,
        //             Dest::RegC => self.c = div,
        //         }
        //     }
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    impl Cpu {
        fn vals(a: i16, b: i16, c: i16) -> Self {
            Cpu { a, b, c, flags: 0 }
        }
    }

    #[test]
    fn load_a() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(Instruction::Ld(-5, Dest::Register(Reg::A)), &mut mem);

        assert_eq!(cpu.a, -5);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn load_abc() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(Instruction::Ld(-5, Dest::Register(Reg::A)), &mut mem);
        cpu.execute(Instruction::Ld(1, Dest::Register(Reg::B)), &mut mem);
        cpu.execute(Instruction::Ld(2020, Dest::Register(Reg::C)), &mut mem);

        assert_eq!(cpu.a, -5);
        assert_eq!(cpu.b, 1);
        assert_eq!(cpu.c, 2020);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn load_into_mem() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(Instruction::Ld(-5, Dest::Memory(0)), &mut mem);

        assert_eq!(mem.read(0), -5);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sum_within_16_bits() {
        let mut cpu = Cpu::vals(0, -3, 4);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Sum(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Sum(Reg::C, Reg::A), &mut mem);

        assert_eq!(cpu.b, -3);
        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sum_with_overflow() {
        let mut cpu = Cpu::vals(32767, 4, 0);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Sum(Reg::B, Reg::A), &mut mem);

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & Cpu::FLAG_OVERFLOW != 0);
    }

    #[test]
    fn sum_of_negatives_with_overflow() {
        let mut cpu = Cpu::vals(-32767, -4, 0);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Sum(Reg::A, Reg::B), &mut mem);

        assert_eq!(cpu.b, 0);
        assert!(cpu.flags & Cpu::FLAG_OVERFLOW != 0);
    }

    #[test]
    fn sub_within_16_bits() {
        let mut cpu = Cpu::vals(3000, -3100, 15);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Sub(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Sub(Reg::A, Reg::C), &mut mem);

        assert_eq!(cpu.b, 6100);
        assert_eq!(cpu.c, 2985);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn sub_with_overflow() {
        let mut cpu = Cpu::vals(-32767, 4, 0);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Sub(Reg::A, Reg::B), &mut mem);

        assert_eq!(cpu.b, 0);
        assert!(cpu.flags & Cpu::FLAG_OVERFLOW != 0);
    }

    // #[test]
    // fn mul_within_16_bits() {
    //     let mut cpu = Cpu::default();
    //     let mut mem = Mem::default();
    //     cpu.execute(Instruction::Mul(4, -4, Dest::RegA), &mut mem);
    //     cpu.execute(Instruction::Mul(45, 10, Dest::Memory(0)), &mut mem);

    //     assert_eq!(cpu.a, -16);
    //     assert_eq!(mem.read(0), 450);
    //     assert_eq!(cpu.flags, 0);
    // }

    // #[test]
    // fn mul_with_overflow() {
    //     let mut cpu = Cpu::default();
    //     let mut mem = Mem::default();
    //     cpu.execute(Instruction::Mul(-32767, 32767, Dest::RegA), &mut mem);

    //     assert_eq!(cpu.a, 0);
    //     assert!(cpu.flags & Cpu::FLAG_OVERFLOW != 0);
    // }

    // #[test]
    // fn div() {
    //     let mut cpu = Cpu::default();
    //     let mut mem = Mem::default();
    //     cpu.execute(Instruction::Div(-32767, 32767, Dest::RegA), &mut mem);
    //     cpu.execute(Instruction::Div(4, 2, Dest::RegB), &mut mem);
    //     cpu.execute(Instruction::Div(10, 5, Dest::Memory(0)), &mut mem);

    //     assert_eq!(cpu.a, -1);
    //     assert_eq!(cpu.b, 2);
    //     assert_eq!(mem.read(0), 2);
    //     assert_eq!(cpu.flags, 0);
    // }

    // #[test]
    // fn div_by_0() {
    //     let mut cpu = Cpu::default();
    //     let mut mem = Mem::default();
    //     cpu.execute(Instruction::Div(-32767, 0, Dest::RegA), &mut mem);
    //     cpu.execute(Instruction::Div(4, 2, Dest::RegB), &mut mem);

    //     assert_eq!(cpu.a, 0);
    //     assert!(cpu.flags & Cpu::FLAG_ZERO != 0);
    // }
}
