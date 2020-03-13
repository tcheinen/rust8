use rust8_core::emu::Emulator;

fn main() {

    let rom: Vec<u8> = include_bytes!("../../roms/maze.ch8").to_vec();
    let mut emu: Emulator = Emulator::default();
    emu.load_rom(rom);

    println!("Hello, world!");
}
