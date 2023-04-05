use std::fs;
use std::path::PathBuf;
use paste::paste;

macro_rules! setup_mapping{
    ($mapping:ident, $start:literal, $end: literal) => {
        paste! {
            const [< $mapping _START >] : u16 = $start;
            const [< $mapping _END >] : u16 = $end;
            #[allow(dead_code)]
            const [< $mapping _SIZE >] : usize = $end - $start + 1;
        }

    };
}

setup_mapping!(BOOTROM, 0x0000, 0x00FF);
setup_mapping!(HRAM, 0xFF80, 0xFFFE);
setup_mapping!(VRAM, 0x8000, 0x9FFF);


pub struct Memory {
    boot_rom: Vec<u8>,
    hram: [u8; HRAM_SIZE],
    vram: [u8; VRAM_SIZE] // TODO: put this in a separate structure (PPU)
}

impl Memory {
    pub fn new(file: PathBuf) -> Self {
        let boot_rom = fs::read(file).unwrap_or_else(|err| panic!("{}", err));
        Memory {
            boot_rom,
            hram: [0; HRAM_SIZE],
            vram: [0; VRAM_SIZE]
        }
    }
    pub fn read_8(&self, addr: u16) -> u8 {
        match addr {
            BOOTROM_START..=BOOTROM_END => self.boot_rom[addr as usize],
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize],
            VRAM_START..=VRAM_END => self.vram[(addr - VRAM_START) as usize],
            _ => unimplemented!()
        }
    }
    pub fn write_8(&mut self, addr: u16, to_write: u8) {
        match addr {
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize] = to_write,
            VRAM_START..=VRAM_END => self.vram[(addr - VRAM_START) as usize] = to_write,
            _ => unimplemented!()
        }
    }
}
