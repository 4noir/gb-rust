use crate::cpu::CPU;

use super::registers::Flags;

impl<'a> CPU<'a> {
    pub fn exec_opcode(&mut self, opcode: u8) {
        println!("Current opcode: {:#X} ; pc : {:#X}", opcode, self.pc - 1);
        match opcode {
            // nop
            0x00 => return,
            // LD A,(FF00 + u8)
            0xF0 => {
                let addr = self.fetch_8();
                self.regs.a = self.read_8(0xFF00 | addr as u16);
            }
            // LD E, u8
            0x1E => self.regs.e = self.fetch_8(),
            // LD D,A
            0x57 => self.regs.d = self.regs.a,
            // JR i8
            0x18 => {
                let offset = self.fetch_8() as i8;
                self.cpu_jr(offset);
            }
            // LD H,A
            0x67 => self.regs.h = self.regs.a,
            // LD L,u8
            0x2E => self.regs.l = self.fetch_8(),
            // DEC A
            0x3d => self.regs.a = self.a_dec(self.regs.a),
            // DEC C
            0x0d => self.regs.c = self.a_dec(self.regs.c),
            // RET
            0xC9 => {
                self.pc = self.pop_stack_16();
                self.clock_4()
            }
            // POP bc
            0xC1 => {
                let v = self.pop_stack_16();
                self.regs.set_bc(v)
            }
            // RLA
            0x17 => {
                self.regs.a = self.rsb_rl(self.regs.a);
                self.regs.f.set(Flags::Z, false)
            }
            // Push BC
            0xC5 => self.push_stack_16(self.regs.bc()),
            // LD C,A
            0x4F => self.regs.c = self.regs.a,
            // LD (BC),A
            0x02 => self.write_8(self.regs.bc(), self.regs.a),
            // INC D
            0x14 => self.regs.d = self.a_inc(self.regs.d),
            // INC B
            0x04 => self.regs.b = self.a_inc(self.regs.b),
            // INC BC
            0x03 => {
                self.regs.set_bc(self.regs.bc().wrapping_add(1));
                self.clock_4()
            }
            // LD C,E
            0x4B => self.regs.c = self.regs.e,
            // LD D,B =>
            0x50 => self.regs.d = self.regs.b,
            // LD (u16),A
            0xEA => {
                let addr = self.fetch_16();
                self.write_8(addr, self.regs.a);
            }
            // DEC B
            0x05 => self.regs.b = self.a_dec(self.regs.b),
            // INC hl
            0x23 => {
                self.regs.set_hl(self.regs.hl().wrapping_add(1));
                self.clock_4()
            }
            // LD b, u8
            0x06 => self.regs.b = self.fetch_8(),
            // CP u8
            0xFE => {
                let b = self.fetch_8();
                self.a_cp(b)
            }
            // LD a,e
            0x7b => self.regs.a = self.regs.e,
            // INC de
            0x13 => {
                self.regs.set_de(self.regs.de().wrapping_add(1));
                self.clock_4()
            }
            // CALL u16,
            0xCD => {
                let addr = self.fetch_16();
                self.push_stack_16(self.pc);
                self.pc = addr;
            }
            // LD A,(DE)
            0x1A => self.regs.a = self.read_8(self.regs.de()),
            // INC C
            0x0c => self.regs.c = self.a_inc(self.regs.c),
            // LD (HL), A
            0x77 => self.write_8(self.regs.hl(), self.regs.a),
            // LD ($FF00 + u8), A
            0xe0 => {
                let addr = self.fetch_8();
                self.write_8(0xFF00 | addr as u16, self.regs.a)
            }
            // LD DE,u16
            0x11 => {
                let v = self.fetch_16();
                self.regs.set_de(v)
            }
            // xor a,a
            0xAF => self.a_xor(self.regs.a),
            // ld sp,u16
            0x31 => self.sp = self.fetch_16(),
            // LD hl,u16
            0x21 => {
                let value = self.fetch_16();
                self.regs.set_hl(value);
            }
            // LD (HL-),A
            0x32 => {
                let a = self.regs.hld();
                self.write_8(a, self.regs.a);
            }
            // LD (HL+),A
            0x22 => {
                let a = self.regs.hli();
                self.write_8(a, self.regs.a);
            }
            // JR nz
            0x20 => {
                let offset = self.fetch_8() as i8;
                if !self.regs.f.contains(Flags::Z) {
                    self.cpu_jr(offset)
                }
            }
            // JR z
            0x28 => {
                let offset = self.fetch_8() as i8;
                if self.regs.f.contains(Flags::Z) {
                    self.cpu_jr(offset)
                }
            }
            // CB PREFIX
            0xCB => self.exec_cb(),
            // ld c,u8
            0x0E => self.regs.c = self.fetch_8(),
            // ld a,u8
            0x3E => self.regs.a = self.fetch_8(),
            // ld (ff00 + c), a
            0xE2 => self.write_8(0xFF00 | self.regs.c as u16, self.regs.a),
            _ => todo!("{:#X}", opcode),
        }
    }
}

impl<'a> CPU<'a> {
    // Arithmetic logic
    fn a_inc(&mut self, b: u8) -> u8 {
        let res = b.wrapping_add(1);
        self.regs.f.remove(Flags::N);
        self.regs.f.set(Flags::Z, res == 0);
        self.regs.f.set(Flags::H, (b & 0x0F) + 1 > 0x0F);
        res
    }
    fn a_dec(&mut self, b: u8) -> u8 {
        let r = b.wrapping_sub(1);
        self.regs.f.insert(Flags::N);
        self.regs.f.set(Flags::Z, r == 0);
        self.regs.f.set(Flags::H, (b & 0x0F) < (r & 0x0f));
        r
    }
    fn a_xor(&mut self, b: u8) {
        self.regs.a ^= b;
        self.regs.f.set(Flags::Z, self.regs.a == 0);
        self.regs.f.set(Flags::H | Flags::N | Flags::C, false);
    }
    fn a_sub(&mut self, b: u8) {
        let r = self.regs.a.wrapping_sub(b);
        self.regs.f.insert(Flags::N);
        self.regs.f.set(Flags::Z, r == 0);
        self.regs.f.set(Flags::H, (self.regs.a & 0x0F) < (b & 0x0F));
        self.regs.f.set(Flags::C, (self.regs.a) < (b));
        self.regs.a = r;
    }
    fn a_cp(&mut self, b: u8) {
        let r = self.regs.a.wrapping_sub(b);
        self.regs.f.insert(Flags::N);
        self.regs.f.set(Flags::Z, r == 0);
        self.regs.f.set(Flags::H, (self.regs.a & 0x0F) < (b & 0x0F));
        self.regs.f.set(Flags::C, (self.regs.a) < (b));
    }
}
impl<'a> CPU<'a> {
    // RSB (shifts and rotates)
    fn rsb_rl(&mut self, b: u8) -> u8 {
        let c = if self.regs.f.contains(Flags::C) { 0 } else { 1 };
        let msb = b & 0b1000_0000;
        let b = b << 1;
        self.regs.f.set(Flags::C, msb == 0b1000_0000);
        self.regs.f.set(Flags::N | Flags::H, false);
        self.regs.f.set(Flags::Z, b == 0);
        b | c
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
        let opcode_full: u16 = 0xCB00 | opcode as u16;
        match opcode {
            // BIT 7, H
            0x7C => self.test_bit(self.regs.h, 7),
            0x11 => self.regs.c = self.rsb_rl(self.regs.c),
            _ => todo!("Unimplemented: ${:#X}", opcode_full),
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
