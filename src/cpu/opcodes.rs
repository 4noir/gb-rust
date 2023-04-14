use crate::cpu::CPU;

use super::registers::Flags;

impl<'a> CPU<'a> {
    pub fn exec_opcode(&mut self, opcode: u8) {
        match opcode {
            // nop
            0x00 => return,
            // INC C
            0x0c => self.regs.c = self.a_inc(self.regs.c),
            // LD (HL), A
            0x77 => self.write_8(self.regs.hl(), self.regs.a),
            // LD ($FF00 + u8), A
            0xe0 => {
                let addr = self.fetch_8();
                self.write_8(0xFF00 | addr as u16, self.regs.a)
            }
            // LD DE, u16
            0x11 => {
                let v = self.fetch_16();
                self.regs.set_de(v)
            }
            // xor a,a
            0xAF => self.a_xor(self.regs.a),
            // ld sp, u16
            0x31 => self.sp = self.fetch_16(),
            // LD hl, u16
            0x21 => {
                let value = self.fetch_16();
                self.regs.set_hl(value);
            }
            // LD (HL-), A
            0x32 => {
                let a = self.regs.hld();
                self.write_8(a, self.regs.a)
            }
            0x20 => {
                let offset = self.fetch_8() as i8;
                if !self.regs.f.contains(Flags::Z) {
                    self.cpu_jr(offset)
                }
            }
            // CB PREFIX
            0xCB => self.exec_cb(),
            // ld c, u8
            0x0E => self.regs.c = self.fetch_8(),
            // ld a, u8
            0x3E => self.regs.a = self.fetch_8(),
            // ld (ff00 + c), a
            0xE2 => self.write_8(0xFF00 | self.regs.c as u16, self.regs.a),
            _ => unimplemented!(),
        }
    }
}

impl<'a> CPU<'a> {
    // Arithmetic logic
    fn a_inc(&mut self, n: u8) -> u8 {
        let res = n.wrapping_add(1);
        self.regs.f.remove(Flags::N);
        self.regs.f.set(Flags::Z, res == 0);
        self.regs.f.set(Flags::H, (n & 0x0F) + 1 > 0x0F);
        res
    }
    fn a_xor(&mut self, b: u8) {
        self.regs.a ^= b;
        self.regs.f.set(Flags::Z, self.regs.a == 0);
        self.regs.f.set(Flags::H | Flags::N | Flags::C, false);
    }
}

impl<'a> CPU<'a> {
    fn cpu_jr(&mut self, n: i8) {
        self.clock_4();
        self.pc = self.pc.wrapping_add_signed(n as i16);
    }
}

// PREFIX CB OPERATIONS
impl<'a> CPU<'a> {
    fn exec_cb(&mut self) {
        let opcode = self.fetch_8();
        match opcode {
            // BIT 7, H
            0x7C => self.test_bit(self.regs.h, 7),
            _ => unimplemented!(),
        }
    }

    fn test_bit(&mut self, v: u8, n: u8) {
        assert!(n < 8);
        let test = v >> n;
        self.regs.f.set(Flags::Z, test == 0);
        self.regs.f.remove(Flags::N);
        self.regs.f.insert(Flags::H);
    }
}
