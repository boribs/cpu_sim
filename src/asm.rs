#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cpu;

const REG_A: u8 = 01;
const REG_AH: u8 = 02;
const REG_AL: u8 = 03;
const REG_B: u8 = 04;
const REG_BH: u8 = 05;
const REG_BL: u8 = 06;
const REG_C: u8 = 07;
const REG_CH: u8 = 08;
const REG_CL: u8 = 09;
const REG_D: u8 = 10;
const REG_DH: u8 = 11;
const REG_DL: u8 = 12;

fn reg_match(reg: cpu::Reg) -> u8 {
    match reg {
        cpu::Reg::A => REG_A,
        cpu::Reg::AH => REG_AH,
        cpu::Reg::AL => REG_AL,
        cpu::Reg::B => REG_B,
        cpu::Reg::BH => REG_BH,
        cpu::Reg::BL => REG_BL,
        cpu::Reg::C => REG_C,
        cpu::Reg::CH => REG_CH,
        cpu::Reg::CL => REG_CL,
        cpu::Reg::D => REG_D,
        cpu::Reg::DH => REG_DH,
        cpu::Reg::DL => REG_DL,
    }
}

impl cpu::Instruction {
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
                instr = 0x1 << 3;
                match a {
                    cpu::Dest::Memory(m) => {
                        dest_a = *m;
                        bit_count += 16;
                    }
                    cpu::Dest::Register(r) => {
                        dest_a = reg_match(*r) as u16;
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
                            dest_a = (dest_a << 8) | (reg_match(*r) as u16);
                        } else {
                            dest_b = (reg_match(*r) as u16) << 8;
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
            [24, 0b00001011, REG_A, REG_B, 0, 0],
            [32, 0b00001001, 0, 0x11, REG_B, 0],
            [32, 0b00001010, REG_B, 0, 0xab, 0],
            [40, 0b00001000, 0xff, 0xfb, 0, 0xab],
        ];

        for i in 0..expected.len() {
            assert_eq!(instrs[i].to_bytes(), expected[i])
        }
    }
}
