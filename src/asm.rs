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
}

impl cpu::Instruction {
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

        // these bits are set if the parameters A/B are registers.
        // TODO: Call them something else.
        const A_REG_MASK: u8 = 0b00000010;
        const B_REG_MASK: u8 = 0b00000001;

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
                        instr |= A_REG_MASK;
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
                        instr |= B_REG_MASK;
                    }
                }
            }
            cpu::Instruction::Sum(a, b)
            | cpu::Instruction::Sub(a, b)
            | cpu::Instruction::Mul(a, b)
            | cpu::Instruction::Div(a, b)
            | cpu::Instruction::And(a, b)
            | cpu::Instruction::Or(a, b)
            | cpu::Instruction::Xor(a, b) => {
                instr |= B_REG_MASK;

                match a {
                    cpu::CR::Register(r) => {
                        instr |= A_REG_MASK;
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

            cpu::Instruction::Shr(a, b)
            | cpu::Instruction::Shl(a, b)
            | cpu::Instruction::Cmp(a, b) => {
                instr |= A_REG_MASK | B_REG_MASK;
                bit_count = 24;
                dest_a = (a.code() as u16) << 8;
                dest_a |= b.code() as u16;
            }
            cpu::Instruction::Not(a)
            | cpu::Instruction::Jmp(a)
            | cpu::Instruction::Jeq(a)
            | cpu::Instruction::Jne(a)
            | cpu::Instruction::Jgt(a)
            | cpu::Instruction::Jlt(a)
            | cpu::Instruction::Push(a)
            | cpu::Instruction::Pop(a) => {
                instr |= A_REG_MASK | B_REG_MASK;
                bit_count = 16;
                dest_a = (a.code() as u16) << 8;
            } // other => unimplemented!("{:?}", other),
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
            Instruction::Shr(Reg::A, Reg::B),
            Instruction::Shr(Reg::CH, Reg::AL),
        ];

        let expected = [
            [24, 0b01010011, Reg::A.code(), Reg::B.code(), 0, 0],
            [24, 0b01010011, Reg::CH.code(), Reg::AL.code(), 0, 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn shl_to_bytes() {
        let instrs = [
            Instruction::Shl(Reg::A, Reg::B),
            Instruction::Shl(Reg::CH, Reg::AL),
        ];

        let expected = [
            [24, 0b01011011, Reg::A.code(), Reg::B.code(), 0, 0],
            [24, 0b01011011, Reg::CH.code(), Reg::AL.code(), 0, 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn cmp_to_bytes() {
        let instrs = [
            Instruction::Cmp(Reg::A, Reg::B),
            Instruction::Cmp(Reg::CH, Reg::AL),
        ];

        let expected = [
            [24, 0b01100011, Reg::A.code(), Reg::B.code(), 0, 0],
            [24, 0b01100011, Reg::CH.code(), Reg::AL.code(), 0, 0],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i]);
        }
    }

    #[test]
    fn jmp_to_bytes() {
        assert_eq!(
            Instruction::Jmp(Reg::D).to_bytes(),
            [16, 0b01101011, Reg::D.code(), 0, 0, 0]
        );
    }

    #[test]
    fn jeq_to_bytes() {
        assert_eq!(
            Instruction::Jeq(Reg::D).to_bytes(),
            [16, 0b01110011, Reg::D.code(), 0, 0, 0]
        );
    }

    #[test]
    fn jne_to_bytes() {
        assert_eq!(
            Instruction::Jne(Reg::D).to_bytes(),
            [16, 0b01111011, Reg::D.code(), 0, 0, 0]
        );
    }

    #[test]
    fn jgt_to_bytes() {
        assert_eq!(
            Instruction::Jgt(Reg::D).to_bytes(),
            [16, 0b10000011, Reg::D.code(), 0, 0, 0]
        );
    }

    #[test]
    fn jlt_to_bytes() {
        assert_eq!(
            Instruction::Jlt(Reg::D).to_bytes(),
            [16, 0b10001011, Reg::D.code(), 0, 0, 0]
        );
    }

    #[test]
    fn push_to_bytes() {
        assert_eq!(
            Instruction::Push(Reg::C).to_bytes(),
            [16, 0b10010011, Reg::C.code(), 0, 0, 0]
        );
    }

    #[test]
    fn pop_to_bytes() {
        assert_eq!(
            Instruction::Pop(Reg::C).to_bytes(),
            [16, 0b10011011, Reg::C.code(), 0, 0, 0]
        );
    }
}
