#[derive(Copy, Clone)]
pub enum Reg {
    A,
    B,
    C,
    D,
}

pub enum Dest {
    Memory(u16),
    Register(Reg),
}

pub enum Inpt {
    Const(i16),
    Register(Reg),
}

pub enum GenerousInpt {
    Const(i16),
    Register(Reg),
    Memory(u16),
}

pub enum Instruction {
    Ld(GenerousInpt, Dest),
    // integer arithmetic
    Sum(Reg, Reg),
    Sub(Reg, Reg),
    Mul(Reg, Reg),
    Div(Reg, Reg),

    // binary operations
    And(Reg, Reg),
    Or(Reg, Reg),
    Not(Reg),
    Xor(Reg, Reg),
    Shr(Inpt, Reg),
    Shl(Inpt, Reg),

    // program flow
    Cmp(Reg, Reg),
    Jmp(Inpt),
    Jeq(Inpt),
    Jne(Inpt),
    Jgt(Inpt),
    Jlt(Inpt),
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
    d: i16,
    flags: u8,
    ip: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            flags: 0,
            ip: 0,
        }
    }
}

impl Cpu {
    pub const FLAG_OVERFLOW: u8 = 0b00000001;
    pub const FLAG_ZERO: u8 = 0b00000010;
    pub const FLAG_EQUAL: u8 = 0b00000100;
    pub const FLAG_GREATER_THAN: u8 = 0b00001000;
    pub const FLAG_LOWER_THAN: u8 = 0b00010000;

    pub fn reg_write(&mut self, reg: Reg, value: i16) {
        match reg {
            Reg::A => self.a = value,
            Reg::B => self.b = value,
            Reg::C => self.c = value,
            Reg::D => self.d = value,
        }
    }

    pub fn reg_read(&self, reg: Reg) -> i16 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
        }
    }

    pub fn flag_set(&mut self, flag: u8) {
        self.flags |= flag;
    }

    // pub fn flag_unset(&mut self, flag: u8) {
    //     self.flags &= !flag;
    // }

    fn instr_ld(&mut self, val: GenerousInpt, dest: Dest, mem: &mut Mem) {
        let val = match val {
            GenerousInpt::Const(c) => c,
            GenerousInpt::Register(r) => self.reg_read(r),
            GenerousInpt::Memory(i) => mem.read(i.into()),
        };

        match dest {
            Dest::Memory(i) => mem.write(i.into(), val),
            Dest::Register(r) => self.reg_write(r, val),
        }
    }

    fn instr_sum(&mut self, a: Reg, b: Reg) {
        let sum;

        {
            let a = self.reg_read(a);
            let b = self.reg_read(b);

            let checksum = a as i32 + b as i32;
            if checksum > std::i16::MAX as i32 || checksum < std::i16::MIN as i32 {
                sum = 0;
                self.flag_set(Self::FLAG_OVERFLOW);
                // TOOD: propagate warning
            } else {
                sum = a + b;
            }
        }

        self.reg_write(b, sum);
    }

    fn instr_sub(&mut self, a: Reg, b: Reg) {
        let sub;

        {
            let a = self.reg_read(a);
            let b = self.reg_read(b);

            let checksub = a as i32 - b as i32;
            if checksub > std::i16::MAX as i32 || checksub < std::i16::MIN as i32 {
                sub = 0;
                self.flag_set(Self::FLAG_OVERFLOW);
                // TOOD: propagate warning
            } else {
                sub = a - b;
            }
        }

        self.reg_write(b, sub);
    }

    fn instr_mul(&mut self, a: Reg, b: Reg) {
        let mul;

        {
            let a = self.reg_read(a);
            let b = self.reg_read(b);

            let checkmul = a as i32 * b as i32;
            if checkmul > std::i16::MAX as i32 || checkmul < std::i16::MIN as i32 {
                mul = 0;
                self.flag_set(Self::FLAG_OVERFLOW);
                // TOOD: propagate warning
            } else {
                mul = a * b;
            }
        }

        self.reg_write(b, mul);
    }

    fn instr_div(&mut self, a: Reg, b: Reg) {
        let div = {
            let a = self.reg_read(a);
            let b = self.reg_read(b);

            if b != 0 {
                a / b
            } else {
                self.flag_set(Self::FLAG_ZERO);
                0
            }
        };

        // division can't be overflown

        self.reg_write(b, div)
    }

    fn instr_and(&mut self, a: Reg, b: Reg) {
        let and = self.reg_read(a) & self.reg_read(b);
        self.reg_write(b, and);
    }

    fn instr_or(&mut self, a: Reg, b: Reg) {
        let or = self.reg_read(a) | self.reg_read(b);
        self.reg_write(b, or);
    }

    fn instr_not(&mut self, a: Reg) {
        let not = !self.reg_read(a);
        self.reg_write(a, not);
    }

    fn instr_xor(&mut self, a: Reg, b: Reg) {
        let xor = self.reg_read(a) ^ self.reg_read(b);
        self.reg_write(b, xor);
    }

    fn instr_shr(&mut self, sh: Inpt, a: Reg) {
        let sh = match sh {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        };
        let val = self.reg_read(a) >> sh;
        self.reg_write(a, val);
    }

    fn instr_shl(&mut self, sh: Inpt, a: Reg) {
        let sh = match sh {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        };
        let val = self.reg_read(a) << sh;
        self.reg_write(a, val);
    }

    fn instr_cmp(&mut self, a: Reg, b: Reg) {
        let a = self.reg_read(a);
        let b = self.reg_read(b);

        if a > b {
            self.flag_set(Self::FLAG_GREATER_THAN);
        } else if a < b {
            self.flag_set(Self::FLAG_LOWER_THAN);
        } else {
            self.flag_set(Self::FLAG_EQUAL);
        }
    }

    fn instr_jmp(&mut self, to: Inpt) {
        self.ip = match to {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        } as u16;
    }

    fn instr_jeq(&mut self, to: Inpt) {
        let to = match to {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        } as u16;

        if self.flags & Self::FLAG_EQUAL == Self::FLAG_EQUAL {
            self.ip = to;
        }
    }

    fn instr_jne(&mut self, to: Inpt) {
        let to = match to {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        } as u16;

        if self.flags & Self::FLAG_EQUAL == 0 {
            self.ip = to;
        }
    }

    fn instr_jgt(&mut self, to: Inpt) {
        let to = match to {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        } as u16;

        if self.flags & Self::FLAG_GREATER_THAN == Cpu::FLAG_GREATER_THAN {
            self.ip = to;
        }
    }

    fn instr_jlt(&mut self, to: Inpt) {
        let to = match to {
            Inpt::Const(c) => c,
            Inpt::Register(r) => self.reg_read(r),
        } as u16;

        if self.flags & Self::FLAG_LOWER_THAN == Cpu::FLAG_LOWER_THAN {
            self.ip = to;
        }
    }

    pub fn execute(&mut self, instr: Instruction, mem: &mut Mem) {
        match instr {
            Instruction::Ld(val, dest) => self.instr_ld(val, dest, mem),
            Instruction::Sum(a, b) => self.instr_sum(a, b),
            Instruction::Sub(a, b) => self.instr_sub(a, b),
            Instruction::Mul(a, b) => self.instr_mul(a, b),
            Instruction::Div(a, b) => self.instr_div(a, b),
            Instruction::And(a, b) => self.instr_and(a, b),
            Instruction::Or(a, b) => self.instr_or(a, b),
            Instruction::Not(a) => self.instr_not(a),
            Instruction::Xor(a, b) => self.instr_xor(a, b),
            Instruction::Shr(a, b) => self.instr_shr(a, b),
            Instruction::Shl(a, b) => self.instr_shl(a, b),
            Instruction::Cmp(a, b) => self.instr_cmp(a, b),
            Instruction::Jmp(to) => self.instr_jmp(to),
            Instruction::Jeq(to) => self.instr_jeq(to),
            Instruction::Jne(to) => self.instr_jne(to),
            Instruction::Jgt(to) => self.instr_jgt(to),
            Instruction::Jlt(to) => self.instr_jlt(to),
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    impl Cpu {
        fn vals(a: i16, b: i16, c: i16) -> Self {
            Cpu {
                a,
                b,
                c,
                ..Default::default()
            }
        }
    }

    #[test]
    fn load_a() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(
            Instruction::Ld(GenerousInpt::Const(-5), Dest::Register(Reg::A)),
            &mut mem,
        );

        assert_eq!(cpu.a, -5);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn load_abc() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(
            Instruction::Ld(GenerousInpt::Const(-5), Dest::Register(Reg::A)),
            &mut mem,
        );
        cpu.execute(
            Instruction::Ld(GenerousInpt::Const(1), Dest::Register(Reg::B)),
            &mut mem,
        );
        cpu.execute(
            Instruction::Ld(GenerousInpt::Const(2020), Dest::Register(Reg::C)),
            &mut mem,
        );

        assert_eq!(cpu.a, -5);
        assert_eq!(cpu.b, 1);
        assert_eq!(cpu.c, 2020);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn load_into_mem() {
        let mut cpu = Cpu::default();
        let mut mem = Mem::default();
        cpu.execute(
            Instruction::Ld(GenerousInpt::Const(-5), Dest::Memory(0)),
            &mut mem,
        );

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

    #[test]
    fn mul_within_16_bits() {
        let mut cpu = Cpu::vals(4, -5, 10);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Mul(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Mul(Reg::A, Reg::C), &mut mem);

        assert_eq!(cpu.b, -20);
        assert_eq!(cpu.c, 40);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn mul_with_overflow() {
        let mut cpu = Cpu::vals(-32767, 32767, 0);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Mul(Reg::A, Reg::B), &mut mem);

        assert_eq!(cpu.b, 0);
        assert!(cpu.flags & Cpu::FLAG_OVERFLOW != 0);
    }

    #[test]
    fn div() {
        let mut cpu = Cpu::vals(-32767, 1, 4);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Div(Reg::A, Reg::B), &mut mem);

        assert_eq!(cpu.b, -32767);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn div_by_0() {
        let mut cpu = Cpu::vals(0, -32767, 0);
        let mut mem = Mem::default();
        cpu.execute(Instruction::Div(Reg::B, Reg::A), &mut mem);

        assert_eq!(cpu.a, 0);
        assert!(cpu.flags & Cpu::FLAG_ZERO != 0);
    }

    #[test]
    fn compare_equal() {
        let mut cpu = Cpu::vals(0, 1, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::C), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_EQUAL == Cpu::FLAG_EQUAL);
    }

    #[test]
    fn compare_greater_than() {
        let mut cpu = Cpu::vals(0, 1, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::B, Reg::C), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_GREATER_THAN == Cpu::FLAG_GREATER_THAN);
    }

    #[test]
    fn compare_lower_than() {
        let mut cpu = Cpu::vals(0, 1, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_LOWER_THAN == Cpu::FLAG_LOWER_THAN);
    }

    #[test]
    fn jmp() {
        let mut cpu = Cpu::vals(0xff, 1, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Jmp(Inpt::Const(45)), &mut mem);
        assert_eq!(cpu.ip, 45);
        cpu.execute(Instruction::Jmp(Inpt::Register(Reg::A)), &mut mem);
        assert_eq!(cpu.ip, 0xff);
    }

    #[test]
    fn jeq_after_equal_number_comparison() {
        let mut cpu = Cpu::vals(3, 3, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jeq(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_EQUAL == Cpu::FLAG_EQUAL);
        assert_eq!(cpu.ip, 0xab);
    }

    #[test]
    fn jeq_doesnt_jmp_if_flag_eq_not_set() {
        let mut cpu = Cpu::vals(3, 4, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jeq(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_EQUAL == 0);
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn jne_after_not_equal_number_comparison() {
        let mut cpu = Cpu::vals(3, -3, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jne(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_EQUAL == 0);
        assert_eq!(cpu.ip, 0xab);
    }

    #[test]
    fn jne_doesnt_jmp_if_flag_eq_set() {
        let mut cpu = Cpu::vals(4, 4, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jne(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_EQUAL == Cpu::FLAG_EQUAL);
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn jgt_with_greater_than_flag_set() {
        let mut cpu = Cpu::vals(4, 7, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::B, Reg::A), &mut mem);
        cpu.execute(Instruction::Jgt(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_GREATER_THAN == Cpu::FLAG_GREATER_THAN);
        assert_eq!(cpu.ip, 0xab);
    }

    #[test]
    fn jgt_without_greater_than_flag_set() {
        let mut cpu = Cpu::vals(4, 4, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jgt(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_GREATER_THAN == 0);
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn jlt_with_lower_than_flag_set() {
        let mut cpu = Cpu::vals(4, 7, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jlt(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_LOWER_THAN == Cpu::FLAG_LOWER_THAN);
        assert_eq!(cpu.ip, 0xab);
    }

    #[test]
    fn jlt_with_greater_than_flag_set() {
        let mut cpu = Cpu::vals(6, 4, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Cmp(Reg::A, Reg::B), &mut mem);
        cpu.execute(Instruction::Jlt(Inpt::Const(0xab)), &mut mem);

        assert!(cpu.flags & Cpu::FLAG_LOWER_THAN == 0);
        assert_eq!(cpu.ip, 0);
    }

    #[test]
    fn and() {
        let mut cpu = Cpu::vals(0xffabu16 as i16, 0x00ff, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::And(Reg::A, Reg::B), &mut mem);
        assert_eq!(cpu.b, 0x00ab);
    }

    #[test]
    fn or() {
        let mut cpu = Cpu::vals(0xff00u16 as i16, 0x00ff, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Or(Reg::A, Reg::B), &mut mem);
        assert_eq!(cpu.b, 0xffffu16 as i16);
    }

    #[test]
    fn not() {
        let mut cpu = Cpu::vals(0xff00u16 as i16, 0, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Not(Reg::A), &mut mem);
        assert_eq!(cpu.a, 0x00ff);
    }

    #[test]
    fn xor() {
        let mut cpu = Cpu::vals(0b1001, 0, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Xor(Reg::A, Reg::B), &mut mem);
        assert_eq!(cpu.b, 0b1001 ^ 0);
    }

    #[test]
    fn shr() {
        let mut cpu = Cpu::vals(0b10, 0xff, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Shr(Inpt::Const(1), Reg::A), &mut mem);
        cpu.execute(Instruction::Shr(Inpt::Const(10), Reg::B), &mut mem);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.b, 0);
    }

    #[test]
    fn shl() {
        let mut cpu = Cpu::vals(0b10, 0xff, 0);
        let mut mem = Mem::default();

        cpu.execute(Instruction::Shl(Inpt::Const(1), Reg::A), &mut mem);
        cpu.execute(Instruction::Shl(Inpt::Const(10), Reg::B), &mut mem);
        assert_eq!(cpu.a, 4);
        assert_eq!(cpu.b, 0xff << 10);
    }
}
