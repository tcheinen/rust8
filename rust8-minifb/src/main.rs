use rust8_core::emu::Emulator;
use minifb::{Window, WindowOptions, Key, KeyRepeat};
use rust8_core::constants::DISPLAY_WIDTH;
use std::thread;

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

        let key: Option<u8> = match  window.get_keys_pressed(KeyRepeat::Yes).unwrap().first() {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::Key4) => Some(0xC),
            Some(Key::Q) => Some(0x4),
            Some(Key::W) => Some(0x5),
            Some(Key::E) => Some(0x6),
            Some(Key::R) => Some(0xD),
            Some(Key::A) => Some(0x7),
            Some(Key::S) => Some(0x8),
            Some(Key::D) => Some(0x9),
            Some(Key::F) => Some(0xE),
            Some(Key::Z) => Some(0xA),
            Some(Key::X) => Some(0x0),
            Some(Key::C) => Some(0xB),
            Some(Key::V) => Some(0xF),
            _ => None,
        };

        if key.is_some() {
            emu.keyboard.press_key(key.unwrap());
        }

        // render
        if emu.display.dirty {
            let vram = emu.display.receive_change();
            for y in 0..height {
                for x in 0..width {
                    let index = (y / 10) * DISPLAY_WIDTH as usize + (x / 10);
                    framebuf[y * width + x] = if vram[index] {0xffffff} else { 0x0 };
                }
            }
            window.update_with_buffer(&framebuf, width, height);
        }
    }
}
