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

pub mod cpu {
    use crate::emu::{*};
    use crate::constants::{FONTSET_START, DISPLAY_HEIGHT, DISPLAY_WIDTH};

    pub trait CPU {
        fn cls(&mut self);
        fn ret(&mut self);
        fn jp_addr(&mut self);
        fn call_addr(&mut self);
        fn se_vx_byte(&mut self);
        fn sne_vx_byte(&mut self);
        fn se_vx_vy(&mut self);
        fn ld_vx_byte(&mut self);
        fn add_vx_byte(&mut self);
        fn ld_vx_vy(&mut self);
        fn or_vx_vy(&mut self);
        fn and_vx_vy(&mut self);
        fn xor_vx_vy(&mut self);
        fn add_vx_vy(&mut self);
        fn sub_vx_vy(&mut self);
        fn shr_vx_vy(&mut self);
        fn subn_vx_vy(&mut self);
        fn shl_vx_vy(&mut self);
        fn sne_vx_vy(&mut self);
        fn ld_i_addr(&mut self);
        fn jp_v0_addr(&mut self);
        fn rnd_vx_byte(&mut self);
        fn drw_vx_vy_nibble(&mut self);
        fn skp_vx(&mut self);
        fn sknp_vx(&mut self);
        fn ld_vx_dt(&mut self);
        fn ld_vx_k(&mut self);
        fn ld_dt_vx(&mut self);
        fn ld_st_vx(&mut self);
        fn add_i_vx(&mut self);
        fn ld_f_vx(&mut self);
        fn ld_b_vx(&mut self);
        fn ld_mem_vx(&mut self);
        fn ld_vx_mem(&mut self);
    }

    impl CPU for Emulator {
        /// 0x00E0 - Clear the display
        fn cls(&mut self) {
            self.vram = [false; 64 * 32];
            self.pc += 2;
        }


        /// 0x00EE - Return from a function
        fn ret(&mut self) {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }

        /// 0x1nnn - Jump to location nnn
        fn jp_addr(&mut self) {
            self.pc = self.get_nnn();
        }

        /// 0x2nnn - Call function at nnn
        fn call_addr(&mut self) {
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = self.get_nnn();
        }

        /// 0x3xkk - Skip next operation if register x is equal to kk
        fn se_vx_byte(&mut self) {
            if self.registers[self.get_x() as usize] == self.get_nn() {
                self.pc += 2;
            }
            self.pc += 2;
        }

        /// 0x4xkk - Skip next operation if register x is not equal to kk
        fn sne_vx_byte(&mut self) {
            if self.registers[self.get_x() as usize] != self.get_nn() {
                self.pc += 2;
            }
            self.pc += 2;
        }

        /// 0x5xy0 - Skip next operation if register x is equal to register y
        fn se_vx_vy(&mut self) {
            if self.registers[self.get_x() as usize] == self.registers[self.get_y() as usize] {
                self.pc += 2;
            }
            self.pc += 2;
        }

        /// 0x6xkk - Load byte kk into register x
        fn ld_vx_byte(&mut self) {
            self.registers[self.get_x() as usize] = self.get_nn();
            self.pc += 2;
        }


        /// 0x7xkk - Add byte kk to register x
        fn add_vx_byte(&mut self) {
            self.registers[self.get_x() as usize] += self.get_nn();
            self.pc += 2;
        }

        /// 0x8xy0 - Load register y into register x
        fn ld_vx_vy(&mut self) {
            self.registers[self.get_x() as usize] = self.registers[self.get_y() as usize];
            self.pc += 2;
        }


        /// 0x8xy1 - Set register x to bitwise or with register y
        fn or_vx_vy(&mut self) {
            self.registers[self.get_x() as usize] |= self.registers[self.get_y() as usize];
            self.pc += 2;
        }

        /// 0x8xy2 - Set register x to bitwise and with register y
        fn and_vx_vy(&mut self) {
            self.registers[self.get_x() as usize] &= self.registers[self.get_y() as usize];
            self.pc += 2;
        }

        /// 0x8xy3 - Set register x to bitwise xor with register y
        fn xor_vx_vy(&mut self) {
            self.registers[self.get_x() as usize] ^= self.registers[self.get_y() as usize];
            self.pc += 2;
        }

        /// 0x8xy4 - Add register y to register x, set register F to 1 if carry
        fn add_vx_vy(&mut self) {
            let sum: u16 = self.registers[self.get_x() as usize] as u16 + self.registers[self.get_y() as usize] as u16;
            self.registers[0xf] = if sum > 255 { 1 } else { 0 };
            self.registers[self.get_x() as usize] = (sum & 0xff) as u8;
            self.pc += 2;
        }

        /// 0x8xy5 - Subtract register y from register x, set register F to 1 if register x > register y
        fn sub_vx_vy(&mut self) {
            self.registers[0xf] = if self.registers[self.get_x() as usize] > self.registers[self.get_y() as usize] { 1 } else { 0 };
            self.registers[self.get_x() as usize] -= self.registers[self.get_y() as usize];
            self.pc += 2;
        }

        /// 0x8xy6 - Set register f to lsb and shift register x right 1
        fn shr_vx_vy(&mut self) {
            self.registers[0xf] = self.registers[self.get_x() as usize] & 0x1;
            self.registers[self.get_x() as usize] >>= 1;
            self.pc += 2;
        }

        /// 0x8xy7 - subtract Vx from Vy and store in Vx, if Vy > Vx then Vf = 1
        fn subn_vx_vy(&mut self) {
            self.registers[0xf] = if self.registers[self.get_y() as usize] > self.registers[self.get_x() as usize] { 1 } else { 0 };
            self.registers[self.get_x() as usize] = self.registers[self.get_y() as usize] - self.registers[self.get_x() as usize];
            self.pc += 2;
        }

        /// 0x8xy8 - Set register f to msb and shift register x left 1
        fn shl_vx_vy(&mut self) {
            self.registers[0xf] = (self.registers[self.get_x() as usize] & 0x80) >> 7;
            self.registers[self.get_x() as usize] <<= 1;
            self.pc += 2;
        }


        /// 0x9xy0 - Skip next operation if Vx != Vy
        fn sne_vx_vy(&mut self) {
            if self.registers[self.get_x() as usize] != self.registers[self.get_y() as usize] {
                self.pc += 2;
            }
            self.pc += 2;
        }

        /// 0xAnnn - Index is set to nnn
        fn ld_i_addr(&mut self) {
            self.index = self.get_nnn();
            self.pc += 2;
        }

        /// 0xBnnn - PC is set to V0 + nnn
        fn jp_v0_addr(&mut self) {
            self.pc = self.registers[0x0] as u16 + self.get_nnn();
        }


        /// 0xCxkk - Generate random byte, AND with kk, and then store in Vx
        fn rnd_vx_byte(&mut self) {
            self.registers[self.get_x() as usize] = rand::random::<u8>() & self.get_nn();
            self.pc += 2;
        }
        /// 0XDxyn - Display n length sprite at memory location I at (Vx, Vy)
        /// VF is set if there is a collision
        fn drw_vx_vy_nibble(&mut self) {
            self.registers[0xf] = 0;
            let mut loc_x = self.get_x();
            let mut loc_y = self.get_y();
            for byte in 0..self.get_n() {
                let y = (self.registers[self.get_y() as usize] + byte) % DISPLAY_HEIGHT;
                for bit in 0..8u8 {
                    let x = (self.registers[self.get_x() as usize] + bit) % DISPLAY_WIDTH;
                    let value = (self.memory[self.index as usize + byte as usize] >> (7 - bit)) & 1;
                    self.registers[0xf] |= value & self.memory[y as usize * DISPLAY_HEIGHT as usize + x as usize];
                    self.memory[y as usize * DISPLAY_HEIGHT as usize + x as usize] ^= value;
                }
            }
            self.vram_dirty = true;
            self.pc += 2;
        }

        /// 0xEx9E - skip if key is pressed
        fn skp_vx(&mut self) {
            if self.keyboard.keypad[self.get_x() as usize] {
                self.pc += 2
            }
            self.pc += 2;
        }

        /// 0xExA1 - skip if key is not pressed
        fn sknp_vx(&mut self) {
            if !self.keyboard.keypad[self.get_x() as usize] {
                self.pc += 2
            }
            self.pc += 2;
        }

        /// 0xFx07 - load delay into register vx
        fn ld_vx_dt(&mut self) {
            self.registers[self.get_x() as usize] = self.delay;
            self.pc += 2;
        }

        /// 0xFx0A - wait for keypress and store value in Vx
        fn ld_vx_k(&mut self) {
            self.registers[self.get_x() as usize] = self.keyboard.wait_for_keypress();
            self.pc += 2;
        }

        /// 0xFx15 - set delay timer to value in Vx
        fn ld_dt_vx(&mut self) {
            self.delay = self.registers[self.get_nn() as usize];
            self.pc += 2;
        }

        /// 0xFx18 - set sound timer to value in Vx
        fn ld_st_vx(&mut self) {
            self.sound = self.registers[self.get_nn() as usize];
            self.pc += 2;
        }
        /// 0xFx1E - add Vx to I
        fn add_i_vx(&mut self) {
            self.index += self.registers[self.get_x() as usize] as u16;
            self.pc += 2;
        }

        /// 0xFx29 - set index to location of (hex) digit sprite
        /// Sprites are stored at 0x50 and are 5 bytes long
        fn ld_f_vx(&mut self) {
            self.index = FONTSET_START + self.registers[self.get_x() as usize] as u16 * 5;
            self.pc += 2;
        }


        /// 0xFx33 - Store BCD representation of VX in memory locations I, I + 1, and I + 2
        /// I stores digit in hundreds place, I + 1 holds the tens place, and I + 2 holds the ones place
        fn ld_b_vx(&mut self) {
            let val: u8 = self.registers[self.get_x() as usize];
            self.memory[self.index as usize] = val / 100;
            self.memory[self.index as usize + 1] = (val % 100) / 10;
            self.memory[self.index as usize + 2] = (val % 10);
            self.pc += 2;
        }

        /// 0xFx55 - Stores registers 0 to x in memory beginning at I
        fn ld_mem_vx(&mut self) {
            for i in 0..self.get_x() as usize + 1 {
                self.memory[self.index as usize + i] = self.registers[i];
            }
            self.index += self.get_x() as u16 + 1;
            self.pc += 2;
        }

        /// 0xFx65 - reads memory into registers 0 to x beginning at I
        fn ld_vx_mem(&mut self) {
            for i in 0..self.get_x() as usize + 1 {
                self.registers[i] = self.memory[self.index as usize + i];
            }
            self.index += self.get_x() as u16 + 1;
            self.pc += 2;
        }
    }
}