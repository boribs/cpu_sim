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
            cpu::Instruction::Ld(_, _) => 0x1,
            _ => unimplemented!(),
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        let mut bit_count: u8 = 8;
        let mut instr: u8;
        let mut dest_a: u16;
        let mut dest_b: u16 = 0;

        // these bits are set if the parameters A/B are registers.
        const A_REG_MASK: u8 = 0b00000010;
        const B_REG_MASK: u8 = 0b00000001;

        match self {
            cpu::Instruction::Ld(a, b) => {
                instr = self.code() << 3;
                match a {
                    cpu::Dest::Memory(m) => {
                        dest_a = *m;
                        bit_count += 16;
                    }
                    cpu::Dest::Register(r) => {
                        dest_a = r.code() as u16;
                        bit_count += 8;
                        instr |= A_REG_MASK;
                    }
                }

                match b {
                    cpu::Dest::Memory(m) => {
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
                    cpu::Dest::Register(r) => {
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
            other => panic!("{:?} not implemented", other),
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
mod test {
    use super::*;
    use crate::cpu::*;

    #[test]
    fn ld_to_bytes() {
        let instrs = [
            Instruction::Ld(Dest::Register(Reg::A), Dest::Register(Reg::B)),
            Instruction::Ld(Dest::Memory(0x11), Dest::Register(Reg::B)),
            Instruction::Ld(Dest::Register(Reg::B), Dest::Memory(0xab)),
            Instruction::Ld(Dest::Memory(0xfffb), Dest::Memory(0xab)),
        ];

        let expected = [
            [24, 0b00001011, cpu::Reg::A.code(), cpu::Reg::B.code(), 0, 0],
            [32, 0b00001001, 0, 0x11, cpu::Reg::B.code(), 0],
            [32, 0b00001010, cpu::Reg::B.code(), 0, 0xab, 0],
            [40, 0b00001000, 0xff, 0xfb, 0, 0xab],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i])
        }
    }
}
