use std::ops::Add;

use crate::cpu::CPU;

use super::registers::Flags;

impl<'a> CPU<'a> {
    pub fn exec_opcode(&mut self, opcode: u8) {
        match opcode {
            // nop
            0x00 => return,
            0x31 => self.op_0x31(),
            0x21 => self.op_0x21(),
            0x32 => self.op_0x32(),
            0x20 => self.op_0x20(),
            0xCB => self.op_0xCB(),
            0x0E => self.op_0x0E(),
            0x3E => self.op_0x3E(),
            0xE2 => self.op_0xE2(),
            _ => unimplemented!(),
        }
    }
    // ld (ff00 + c), a
    fn op_0xE2(&mut self) {
        self.write_8(0xFF00 | self.bc.split.lo as u16, self.af.split.hi);
    }
    // ld a, u8
    fn op_0x3E(&mut self) {
        let newA = self.fetch_8();
        unsafe {
            self.af.split.hi = newA;
        }
    }
    // ld c, u8
    fn op_0x0E(&mut self) {
        let newC = self.fetch_8();
        unsafe {
            self.bc.split.lo = newC;
        }
    }
    // ld sp, u16
    fn op_0x31(&mut self) {
        self.sp = self.fetch_16();
    }

    // xor a
    fn op_0xAF(&mut self) {
        unsafe {
            self.af.split.hi = 0;
            self.af.split.lo |= Flags::Z;
        }
    }

    // LD hl, d16
    fn op_0x21(&mut self) {
        unsafe {
            self.hl.full = self.fetch_16();
        }
    }

    // ldd (hl)-, A
    fn op_0x32(&mut self) {
        unsafe {
            self.write_8(self.hl.full, self.af.split.hi);
            self.hl.full = self.hl.full.wrapping_sub(1);
        }
    }

    // JR nz, i8
    fn op_0x20(&mut self) {
        let offset = self.fetch_8() as i8;
        unsafe {
            if !self.af.split.lo.contains(Flags::Z) {
                self.clock_4();
                self.pc = self.pc.wrapping_add_signed(offset.into());
            }
        }
    }
}

// PREFIX CB OPERATIONS
impl<'a> CPU<'a> {
    fn op_0xCB(&mut self) {
        let opcode = self.fetch_8();
        match opcode {
            0x7C => self.op_0xCB7C(),
            _ => unimplemented!(),
        }
    }

    // BIT 7, H
    fn op_0xCB7C(&mut self) {
        let h = unsafe { self.hl.split.hi };
        let h = h >> 7;
        unsafe { self.af.split.lo.set(Flags::Z, h == 0) }
    }
}
