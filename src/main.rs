use clap::Parser;
use gb_rust::cpu::CPU;
use gb_rust::memory::Memory;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gb-rust", author = "Raphael Leroy", version = "0.1.0", about)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    boot_rom: PathBuf,
}

pub fn main() {
    let args = Args::parse();
    let boot_rom = args.boot_rom;
    let mut memory = Memory::new(boot_rom);
    let CPU = CPU::new(&mut memory);
}
