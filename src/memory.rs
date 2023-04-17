use paste::paste;
use std::fs;
use std::path::PathBuf;

macro_rules! setup_mapping {
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
setup_mapping!(GAME_CART, 0x0100, 0x7FFF);
setup_mapping!(HRAM, 0xFF80, 0xFFFE);
setup_mapping!(VRAM, 0x8000, 0x9FFF);
setup_mapping!(IO_REGISTER, 0xFF00, 0xFF7F);

pub struct Memory {
    boot_rom: Vec<u8>,
    temp_game_rom: Vec<u8>,
    hram: [u8; HRAM_SIZE],
    vram: [u8; VRAM_SIZE], // TODO: put this in a separate structure (PPU)
}

impl Memory {
    pub fn new(boot_rom: PathBuf, game: PathBuf) -> Self {
        let boot_rom = fs::read(boot_rom).unwrap_or_else(|err| panic!("{}", err));
        let temp_game_rom = fs::read(game).unwrap_or_else(|err| panic!("{}", err));
        Memory {
            temp_game_rom,
            boot_rom,
            hram: [0; HRAM_SIZE],
            vram: [0; VRAM_SIZE],
        }
    }
    pub fn read_8(&self, addr: u16) -> u8 {
        match addr {
            BOOTROM_START..=BOOTROM_END => self.boot_rom[addr as usize],
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize],
            VRAM_START..=VRAM_END => self.vram[(addr - VRAM_START) as usize],
            GAME_CART_START..=GAME_CART_END => self.temp_game_rom[addr as usize],
            IO_REGISTER_START..=IO_REGISTER_END => 0,
            _ => {
                eprintln!("address {:#X} not supported", addr);
                0
            }
        }
    }
    pub fn write_8(&mut self, addr: u16, to_write: u8) {
        match addr {
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize] = to_write,
            VRAM_START..=VRAM_END => self.vram[(addr - VRAM_START) as usize] = to_write,
            IO_REGISTER_START..=IO_REGISTER_END => return,
            _ => todo!("unimplemented memory zone: {:#X}", addr),
        }
    }
}
