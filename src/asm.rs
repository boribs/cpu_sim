#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cpu;

impl cpu::Reg {
    pub fn code(&self) -> u8 {
        match self {
            cpu::Reg::A => 1,
            cpu::Reg::AH => 2,
            cpu::Reg::AL => 3,
            cpu::Reg::B => 4,
            cpu::Reg::BH => 5,
            cpu::Reg::BL => 6,
            cpu::Reg::C => 7,
            cpu::Reg::CH => 8,
            cpu::Reg::CL => 9,
            cpu::Reg::D => 10,
            cpu::Reg::DH => 11,
            cpu::Reg::DL => 12,
        }
    }

    pub fn from_code(code: u8) -> Result<Self, ()> {
        if code > 12 {
            Err(())
        } else {
            Ok(match code {
                1 => cpu::Reg::A,
                2 => cpu::Reg::AH,
                3 => cpu::Reg::AL,
                4 => cpu::Reg::B,
                5 => cpu::Reg::BH,
                6 => cpu::Reg::BL,
                7 => cpu::Reg::C,
                8 => cpu::Reg::CH,
                9 => cpu::Reg::CL,
                10 => cpu::Reg::D,
                11 => cpu::Reg::DH,
                12 => cpu::Reg::DL,
                _ => unreachable!(),
            })
        }
    }
}

impl cpu::Instruction {
    // these bits are set if the parameters A/B are registers.
    // TODO: Call them something else.
    const A_REG_MASK: u8 = 0b00000010;
    const B_REG_MASK: u8 = 0b00000001;

    pub fn code(&self) -> u8 {
        match self {
            cpu::Instruction::Ld(_, _) => 1,
            cpu::Instruction::Sum(_, _) => 2,
            cpu::Instruction::Sub(_, _) => 3,
            cpu::Instruction::Mul(_, _) => 4,
            cpu::Instruction::Div(_, _) => 5,
            cpu::Instruction::And(_, _) => 6,
            cpu::Instruction::Or(_, _) => 7,
            cpu::Instruction::Not(_) => 8,
            cpu::Instruction::Xor(_, _) => 9,
            cpu::Instruction::Shr(_, _) => 10,
            cpu::Instruction::Shl(_, _) => 11,
            cpu::Instruction::Cmp(_, _) => 12,
            cpu::Instruction::Jmp(_) => 13,
            cpu::Instruction::Jeq(_) => 14,
            cpu::Instruction::Jne(_) => 15,
            cpu::Instruction::Jgt(_) => 16,
            cpu::Instruction::Jlt(_) => 17,
            cpu::Instruction::Push(_) => 18,
            cpu::Instruction::Pop(_) => 19,
            // other => unimplemented!("Code for {:?} not implemented.", other),
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        let mut bit_count: u8 = 8; // TODO: Change to byte count?
        let mut instr = self.code() << 3;
        let mut dest_a: u16;
        let mut dest_b: u16 = 0;

        match self {
            cpu::Instruction::Ld(a, b) => {
                match a {
                    cpu::CR::Constant(m) => {
                        dest_a = *m;
                        bit_count += 16;
                    }
                    cpu::CR::Register(r) => {
                        dest_a = r.code() as u16;
                        bit_count += 8;
                        instr |= Self::A_REG_MASK;
                    }
                }

                match b {
                    cpu::CR::Constant(m) => {
                        if bit_count == 16 {
                            // a dest was also a register
                            let b = m.to_be_bytes();
                            dest_a <<= 8;
                            dest_a |= b[0] as u16;
                            dest_b = (b[1] as u16) << 8;
                        } else {
                            dest_b = *m;
                        }

                        bit_count += 16;
                    }
                    cpu::CR::Register(r) => {
                        if bit_count == 16 {
                            // a dest was also a register
                            dest_a = (dest_a << 8) | (r.code() as u16);
                        } else {
                            dest_b = (r.code() as u16) << 8;
                        }
                        bit_count += 8;
                        instr |= Self::B_REG_MASK;
                    }
                }
            }
            cpu::Instruction::Sum(a, b)
            | cpu::Instruction::Sub(a, b)
            | cpu::Instruction::Mul(a, b)
            | cpu::Instruction::Div(a, b)
            | cpu::Instruction::And(a, b)
            | cpu::Instruction::Or(a, b)
            | cpu::Instruction::Xor(a, b)
            | cpu::Instruction::Shr(a, b)
            | cpu::Instruction::Shl(a, b)
            | cpu::Instruction::Cmp(a, b) => {
                instr |= Self::B_REG_MASK;

                match a {
                    cpu::CR::Register(r) => {
                        instr |= Self::A_REG_MASK;
                        dest_a = (r.code() as u16) << 8;
                        dest_a |= b.code() as u16;
                        bit_count = 24;
                    }
                    cpu::CR::Constant(c) => {
                        let c = c.to_be_bytes();
                        dest_a = ((c[0] as u16) << 8) | (c[1] as u16);
                        dest_b = (b.code() as u16) << 8;
                        bit_count = 32;
                    }
                };
            }
            cpu::Instruction::Not(a) | cpu::Instruction::Pop(a) => {
                instr |= Self::A_REG_MASK | Self::B_REG_MASK;
                bit_count = 16;
                dest_a = (a.code() as u16) << 8;
            } // other => unimplemented!("{:?}", other),

            cpu::Instruction::Jmp(a)
            | cpu::Instruction::Jeq(a)
            | cpu::Instruction::Jne(a)
            | cpu::Instruction::Jgt(a)
            | cpu::Instruction::Jlt(a)
            | cpu::Instruction::Push(a) => {
                match a {
                    cpu::CR::Register(r) => {
                        instr |= Self::A_REG_MASK;
                        bit_count = 16;
                        dest_a = (r.code() as u16) << 8;
                    }
                    cpu::CR::Constant(c) => {
                        bit_count = 24;
                        dest_a = *c;
                    }
                };
            }
        }

        [
            bit_count,
            instr,
            (dest_a >> 8) as u8,
            dest_a as u8,
            (dest_b >> 8) as u8,
            dest_b as u8,
        ]
    }

    pub fn from_mem(mem: &cpu::Mem, index: usize) -> Result<(cpu::Instruction, usize), ()> {
        fn get_cr(
            bytes: &mut usize,
            mem: &cpu::Mem,
            is_reg: bool,
            index: usize,
        ) -> Result<cpu::CR, ()> {
            Ok(if is_reg {
                *bytes += 1;
                let r = cpu::Reg::from_code(mem.read(index))?;
                cpu::CR::Register(r)
            } else {
                *bytes += 2;
                cpu::CR::Constant(mem.read_16(index))
            })
        }

        let mut bytes = 1;
        let instr = mem.read(index);
        let (a_reg, b_reg) = (instr & Self::A_REG_MASK != 0, instr & Self::B_REG_MASK != 0);
        let instr = instr >> 3;

        let out = match instr {
            1 => {
                let a = get_cr(&mut bytes, mem, a_reg, index + 1)?;
                let index = index + bytes;
                let b = get_cr(&mut bytes, mem, b_reg, index)?;
                cpu::Instruction::Ld(a, b)
            }
            2..=7 | 9..=11 => {
                let a = get_cr(&mut bytes, mem, a_reg, index + 1)?;
                let b = cpu::Reg::from_code(mem.read(index + bytes))?;
                bytes += 1;

                match instr {
                    2 => cpu::Instruction::Sum(a, b),
                    3 => cpu::Instruction::Sub(a, b),
                    4 => cpu::Instruction::Mul(a, b),
                    5 => cpu::Instruction::Div(a, b),
                    6 => cpu::Instruction::And(a, b),
                    7 => cpu::Instruction::Or(a, b),
                    9 => cpu::Instruction::Xor(a, b),
                    10 => cpu::Instruction::Shr(a, b),
                    11 => cpu::Instruction::Shl(a, b),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        };

        Ok((out, bytes))
    }
}

#[cfg(test)]
mod byte_conversion_test {
    use crate::cpu::*;

    #[test]
    fn ld_to_bytes() {
        let instrs = [
            Instruction::Ld(CR::Register(Reg::A), CR::Register(Reg::B)),
            Instruction::Ld(CR::Constant(0x11), CR::Register(Reg::B)),
            Instruction::Ld(CR::Register(Reg::B), CR::Constant(0xab)),
            Instruction::Ld(CR::Constant(0xfffb), CR::Constant(0xab)),
        ];

        let expected = [
            [24, 0b00001011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00001001, 0, 0x11, Reg::B.code(), 0],
            [32, 0b00001010, Reg::B.code(), 0, 0xab, 0],
            [40, 0b00001000, 0xff, 0xfb, 0, 0xab],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn sum_to_bytes() {
        let instrs = [
            Instruction::Sum(CR::Register(Reg::A), Reg::B),
            Instruction::Sum(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00010011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00010001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn sub_to_bytes() {
        let instrs = [
            Instruction::Sub(CR::Register(Reg::A), Reg::B),
            Instruction::Sub(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00011011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00011001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn mul_to_bytes() {
        let instrs = [
            Instruction::Mul(CR::Register(Reg::A), Reg::B),
            Instruction::Mul(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00100011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00100001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn div_to_bytes() {
        let instrs = [
            Instruction::Div(CR::Register(Reg::A), Reg::B),
            Instruction::Div(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00101011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00101001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn and_to_bytes() {
        let instrs = [
            Instruction::And(CR::Register(Reg::A), Reg::B),
            Instruction::And(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00110011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00110001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn or_to_bytes() {
        let instrs = [
            Instruction::Or(CR::Register(Reg::A), Reg::B),
            Instruction::Or(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b00111011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b00111001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn not_to_bytes() {
        let instrs = [Instruction::Not(Reg::A), Instruction::Not(Reg::CH)];

        let expected = [
            [16, 0b01000011, Reg::A.code(), 0, 0, 0],
            [16, 0b01000011, Reg::CH.code(), 0, 0, 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn xor_to_bytes() {
        let instrs = [
            Instruction::Xor(CR::Register(Reg::A), Reg::B),
            Instruction::Xor(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b01001011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b01001001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn shr_to_bytes() {
        let instrs = [
            Instruction::Shr(CR::Register(Reg::A), Reg::B),
            Instruction::Shr(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b01010011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b01010001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn shl_to_bytes() {
        let instrs = [
            Instruction::Shl(CR::Register(Reg::A), Reg::B),
            Instruction::Shl(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b01011011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b01011001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn cmp_to_bytes() {
        let instrs = [
            Instruction::Cmp(CR::Register(Reg::A), Reg::B),
            Instruction::Cmp(CR::Constant(0xab), Reg::AL),
        ];

        let expected = [
            [24, 0b01100011, Reg::A.code(), Reg::B.code(), 0, 0],
            [32, 0b01100001, 0, 0xab, Reg::AL.code(), 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jmp_to_bytes() {
        let instrs = [
            Instruction::Jmp(CR::Register(Reg::D)),
            Instruction::Jmp(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b01101010, Reg::D.code(), 0, 0, 0],
            [24, 0b01101000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jeq_to_bytes() {
        let instrs = [
            Instruction::Jeq(CR::Register(Reg::D)),
            Instruction::Jeq(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b01110010, Reg::D.code(), 0, 0, 0],
            [24, 0b01110000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jne_to_bytes() {
        let instrs = [
            Instruction::Jne(CR::Register(Reg::D)),
            Instruction::Jne(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b01111010, Reg::D.code(), 0, 0, 0],
            [24, 0b01111000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jgt_to_bytes() {
        let instrs = [
            Instruction::Jgt(CR::Register(Reg::D)),
            Instruction::Jgt(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b10000010, Reg::D.code(), 0, 0, 0],
            [24, 0b10000000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jlt_to_bytes() {
        let instrs = [
            Instruction::Jlt(CR::Register(Reg::D)),
            Instruction::Jlt(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b10001010, Reg::D.code(), 0, 0, 0],
            [24, 0b10001000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn push_to_bytes() {
        let instrs = [
            Instruction::Push(CR::Register(Reg::D)),
            Instruction::Push(CR::Constant(0xab)),
        ];

        let expected = [
            [16, 0b10010010, Reg::D.code(), 0, 0, 0],
            [24, 0b10010000, 0, 0xab, 0, 0],
        ];

        for i in 0..instrs.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn pop_to_bytes() {
        assert_eq!(
            Instruction::Pop(Reg::C).to_bytes(),
            [16, 0b10011011, Reg::C.code(), 0, 0, 0]
        );
    }
}

#[cfg(test)]
mod read_from_mem {
    use crate::cpu::*;

    #[test]
    fn read_ld() {
        let mem = Mem::set(vec![
            0b00001011,
            Reg::A.code(),
            Reg::B.code(),
            0b00001001,
            0,
            0x11,
            Reg::B.code(),
            0b00001000,
            0xff,
            0xfb,
            0,
            0xab,
        ]);

        let expected = [
            (
                Instruction::Ld(CR::Register(Reg::A), CR::Register(Reg::B)),
                3,
            ),
            (Instruction::Ld(CR::Constant(0x11), CR::Register(Reg::B)), 4),
            (Instruction::Ld(CR::Constant(0xfffb), CR::Constant(0xab)), 5),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
            Instruction::from_mem(&mem, 7),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_sum() {
        let mem = Mem::set(vec![
            0b00010011,
            Reg::A.code(),
            Reg::B.code(),
            0b00010001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Sum(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Sum(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_sub() {
        let mem = Mem::set(vec![
            0b00011011,
            Reg::A.code(),
            Reg::B.code(),
            0b00011001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Sub(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Sub(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_mul() {
        let mem = Mem::set(vec![
            0b00100011,
            Reg::A.code(),
            Reg::B.code(),
            0b00100001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Mul(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Mul(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_div() {
        let mem = Mem::set(vec![
            0b00101011,
            Reg::A.code(),
            Reg::B.code(),
            0b00101001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Div(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Div(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_and() {
        let mem = Mem::set(vec![
            0b00110011,
            Reg::A.code(),
            Reg::B.code(),
            0b00110001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::And(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::And(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_or() {
        let mem = Mem::set(vec![
            0b00111011,
            Reg::A.code(),
            Reg::B.code(),
            0b00111001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Or(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Or(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_xor() {
        let mem = Mem::set(vec![
            0b01001011,
            Reg::A.code(),
            Reg::B.code(),
            0b01001001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Xor(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Xor(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_shr() {
        let mem = Mem::set(vec![
            0b01010011,
            Reg::A.code(),
            Reg::B.code(),
            0b01010001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Shr(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Shr(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }

    #[test]
    fn read_shl() {
        let mem = Mem::set(vec![
            0b01011011,
            Reg::A.code(),
            Reg::B.code(),
            0b01011001,
            0,
            0xab,
            Reg::AL.code(),
        ]);

        let expected = [
            (Instruction::Shl(CR::Register(Reg::A), Reg::B), 3),
            (Instruction::Shl(CR::Constant(0xab), Reg::AL), 4),
        ];

        let actual = [
            Instruction::from_mem(&mem, 0),
            Instruction::from_mem(&mem, 3),
        ];

        for i in 0..expected.len() {
            assert!(actual[i].is_ok());
            let a = actual[i].unwrap();
            assert_eq!(a, expected[i]);
        }
    }
}
