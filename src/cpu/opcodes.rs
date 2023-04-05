use crate::cpu::CPU;

use super::registers::Flags;

impl<'a> CPU<'a> {
    fn exec_opcode(&mut self, opcode: u8) {
        match opcode {
            // nop
            0x00 => return,
            0x31 => self.op_0x31(),
            0x21 => self.op_0x21(),
            0xCB => self.op_0xCB(),
            _ => unimplemented!(),
        }
    }

    // ld sp, d16
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
    // CB prefix
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
        unsafe {
            self.af.split.lo.set(Flags::Z, h == 0)
        }
    }
}
