// Rust8
// Copyright (C) 2019 Teddy Heinen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::constants;

pub struct State {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay: u8,
    pub sound: u8,
    pub keypad: [bool; 16],
    pub vram: [bool; 64 * 32],
    pub opcode: u16,
}

impl State {

    // Read u16 from memory and increment PC twice
    pub fn read_cycle(&self) -> u16 {
        opcode: u16 = self.memory[pc] | self.memory[pc+1];
        self.pc += 2;
        opcode
    }

    // return short value for an operation
    pub fn get_u16(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    // return byte value for an operation
    pub fn get_u8(&self) -> u8 {
        (self.opcode & 0xFF) as u8
    }

    // return left register for an operation
    pub fn get_vx(&self) -> u8 {
        ((self.opcode & 0x0F00) >> 8) as u8
    }

    // return right register for an operation
    pub fn get_vy(&self) -> u8 {
        ((self.opcode & 0x00F0) >> 4) as u8
    }
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
            opcode: 0
        };
        for i in 0..constants::FONTSET.len() {
            state.memory[constants::FONTSET_START + i as u16] = constants::FONTSET[i];
        }
        state;
    }
}
