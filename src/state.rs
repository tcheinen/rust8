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
use crate::cpu::CPU;

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
    pub fn tick(&mut self) {
        let instruction: u16 = (self.memory[pc] << 8) | self.memory[pc + 1];
        let lower_4: u8 = (instruction & 0xff) as u8;
        let lower_8: u8 = (instruction & 0xff) as u8;
        let lower_8: u8 = (instruction & 0xff) as u8;

        let x: u8 = ((instruction >> 8) & 0xf0) as u8;
        let n: u8 = (instruction & 0x000f) as u8;
        let nn: u8 = (instruction & 0x00ff) as u8;

        match instruction >> 12 {
            0x0 => {
                match nn {
                    0xE0 => { self.clear() }
                    0xEE => { self.ret() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", instruction, self.pc) }
                }
            }
            0x1 => { self.jump() }
            0x2 => { self.call() }
            0x3 => { self.skip_equal_byte() }
            0x4 => { self.skip_neq_byte() }
            0x5 => { self.skip_equal() }
            0x6 => { self.load_byte() }
            0x7 => { self.add_byte() }
            0x8 => {
                match n {
                    0x0 => { self.load() }
                    0x1 => { self.or() }
                    0x2 => { self.and() }
                    0x3 => { self.xor() }
                    0x4 => { self.add() }
                    0x5 => { self.sub() }
                    0x6 => { self.shr() }
                    0x7 => { self.subn() }
                    0x8 => { self.shl() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", instruction, self.pc) }
                }
            }
            0x9 => { self.skip_neq() }
            0xA => { self.load_index_byte() }
            0xB => { self.jump_r0_byte() }
            0xC => { self.rand() }
            0xD => { self.draw() }
            0xE => {
                match nn {
                    0x9e => { self.skip_key() }
                    0xA1 => { self.skip_neq_key() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", instruction, self.pc) }
                }
            }
            0xF => {
                match nn {
                    0x07 => { self.load_reg_delay() }
                    0x0A => { self.load_reg_key() }
                    0x15 => { self.load_delay_reg() }
                    0x18 => { self.load_sound_reg() }
                    0x1E => { self.add_i_reg() }
                    0x29 => { self.load_sprite() }
                    0x33 => { self.load_bcd() }
                    0x55 => { self.load_mem_registers() }
                    0x65 => { self.load_registers_mem() }
                    _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", instruction, self.pc) }
                }
            }
            _ => { panic!("Invalid Opcode {:#X} at PC {:#x}", instruction, self.pc) }
        }
    }

    // Read u16 from memory and increment PC twice
    pub fn read_cycle(&mut self) -> u16 {
        opcode: u16 = self.memory[pc] | self.memory[pc + 1];
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
            opcode: 0,
        };
        for i in 0..constants::FONTSET.len() {
            state.memory[constants::FONTSET_START + i as u16] = constants::FONTSET[i];
        }
        state;
    }
}
