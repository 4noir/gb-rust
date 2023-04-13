mod opcodes;
mod registers;

use super::memory::Memory;
use registers::Registers;

pub struct CPU<'a> {
    regs: Registers,
    sp: u16,
    pc: u16,
    mem: &'a mut Memory,
    current_ticks: u32,
}

impl<'a> CPU<'a> {
    pub fn new(mem: &'a mut Memory) -> Self {
        CPU {
            regs: Registers::new(),
            sp: 0,
            pc: 0,
            mem,
            current_ticks: 0,
        }
    }

    fn clock_4(&mut self) {
        self.current_ticks = self.current_ticks.wrapping_add(4);
    }

    fn clock_reset(&mut self) {
        self.current_ticks = 0;
    }

    fn fetch_8(&mut self) -> u8 {
        let ret = self.read_8(self.pc);
        self.pc += 1;
        ret
    }

    fn read_8(&mut self, addr: u16) -> u8 {
        self.clock_4();
        self.mem.read_8(addr)
    }

    fn write_8(&mut self, addr: u16, to_write: u8) {
        self.clock_4();
        self.mem.write_8(addr, to_write);
    }

    fn fetch_16(&mut self) -> u16 {
        let lo_byte = self.fetch_8();
        let hi_byte = self.fetch_8();

        ((hi_byte as u16) << 8) | lo_byte as u16
    }

    fn read_16(&mut self, addr: u16) -> u16 {
        let lo_byte = self.mem.read_8(addr);
        let hi_byte = self.mem.read_8(addr + 1);
        ((hi_byte as u16) << 8) | lo_byte as u16
    }

    fn write_16(&mut self, addr: u16, to_write: u16) {
        let lo_byte: u8 = (to_write & 0x00FF) as u8;
        let hi_byte = (to_write >> 8) as u8;
        self.write_8(addr, lo_byte);
        self.write_8(addr + 1, hi_byte);
    }

    pub fn step(&mut self) -> u32 {
        self.clock_reset();
        let opcode = self.fetch_8();
        self.exec_opcode(opcode);
        return self.current_ticks;
    }
}
