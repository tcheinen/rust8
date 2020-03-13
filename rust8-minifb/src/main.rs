use rust8_core::emu::Emulator;
use minifb::{Window, WindowOptions, Key};
use rust8_core::constants::DISPLAY_WIDTH;

fn main() {
    let rom: Vec<u8> = include_bytes!("../../roms/maze.ch8").to_vec();
    let mut emu: Emulator = Emulator::default();
    emu.load_rom(rom);

    let width = 640;
    let height = 320;

    let mut framebuf: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Rusty_Chip8",
        width,
        height,
        WindowOptions::default(),
    ).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        emu.tick();
        if emu.vram_dirty {
            for y in 0..height {
                for x in 0..width {
                    let index = (y / 10) * DISPLAY_WIDTH as usize + (x / 10);
                    framebuf[y * width + x] = if emu.vram[index] {0xffffff} else { 0x0 };
                }
            }
            window.update_with_buffer(&framebuf, width, height);
            emu.vram_dirty = false;
        }
    }
}
