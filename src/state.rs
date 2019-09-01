use crate::constants;

pub struct State {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    delay: u8,
    sound: u8,
    keypad: [bool; 16],
    vram: [bool; 64 * 32],
}

impl Default for State {
    fn default() -> State {
        let mut state = State {
            registers: [0x0; 16],
            memory: [0x0; 4096],
            index: 0,
            pc: 0x200,
            sp: 0,
            stack: [0x0; 16],
            delay: 0,
            sound: 0,
            keypad: [false; 16],
            vram: [false; 64 * 32],
        };
        for i in 0..constants::FONTSET.len() {
            state.memory[constants::FONTSET_START + i as u16] = constants::FONTSET[i];
        }
        state;
    }
}
