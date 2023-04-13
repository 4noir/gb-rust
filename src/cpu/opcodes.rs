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
            0xCB => self.op_0xcb(),
            0x0E => self.op_0x0e(),
            0x3E => self.op_0x3e(),
            0xE2 => self.op_0xe2(),
            _ => unimplemented!(),
        }
    }
    // ld (ff00 + c), a
    fn op_0xe2(&mut self) {
        self.write_8(0xFF00 | self.regs.c as u16, self.regs.a);
    }
    // ld a, u8
    fn op_0x3e(&mut self) {
        let new_a = self.fetch_8();
        self.regs.a = new_a;
    }
    // ld c, u8
    fn op_0x0e(&mut self) {
        let new_c = self.fetch_8();
        self.regs.c = new_c;
    }
    // ld sp, u16
    fn op_0x31(&mut self) {
        self.sp = self.fetch_16();
    }

    // xor a
    fn op_0xaf(&mut self) {
        self.regs.a = 0;
        self.regs.set_flags(Flags::Z);
    }

    // LD hl, d16
    fn op_0x21(&mut self) {
        let value = self.fetch_16();
        self.regs.set_hl(value);
    }

    // ldd (hl)-, A
    fn op_0x32(&mut self) {
        self.write_8(self.regs.hl(), self.regs.a);
        self.regs.set_hl(self.regs.hl().wrapping_sub(1));
    }

    // JR nz, i8
    fn op_0x20(&mut self) {
        let offset = self.fetch_8() as i8;
        if !self.regs.get_flags().contains(Flags::Z) {
            self.clock_4();
            self.pc = self.pc.wrapping_add_signed(offset.into());
        }
    }
}

// PREFIX CB OPERATIONS
impl<'a> CPU<'a> {
    fn op_0xcb(&mut self) {
        let opcode = self.fetch_8();
        match opcode {
            0x7C => self.op_0xcb7c(),
            _ => unimplemented!(),
        }
    }

    // BIT 7, H
    fn op_0xcb7c(&mut self) {
        let h = self.regs.h >> 7;
        if h == 0 {
            self.regs.set_flags(Flags::Z);
        } else {
            self.regs.remove_flags(Flags::Z);
        }
    }
}
