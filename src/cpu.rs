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

mod cpu {
    use crate::state::{*};

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

    impl CPU for State {
        // 0x00E0 - Clear the display
        fn cls(&mut self) {
            self.vram = [false; 64 * 32]
        }

        // 0x00EE - Return from a function
        fn ret(&mut self) {
            self.sp -= 1;
            self.pc = self.stack[self.sp];
        }

        // 0x1nnn - Jump to location nnn
        fn jp_addr(&mut self) {
            self.pc = self.get_u16();
        }

        // 0x2nnn - Call function at nnn
        fn call_addr(&mut self) {
            self.stack[self.sp] = self.pc;
            self.sp += 1;
            self.pc = self.get_u16();
        }

        // 0x3xkk - Skip next operation if register x is equal to kk
        fn se_vx_byte(&mut self) {
            if self.registers[self.get_vx() == self.get_u8()] {
                self.pc += 2;
            }
        }

        // 0x4xkk - Skip next operation if register x is not equal to kk
        fn sne_vx_byte(&mut self) {
            if self.registers[self.get_vx() != self.get_u8()] {
                self.pc += 2;
            }
        }

        // 0x5xy0 - Skip next operation if register x is equal to register y
        fn se_vx_vy(&mut self) {
            if self.get_vx() == self.get_vy() {
                self.pc += 2;
            }
        }

        // 0x6xkk - Load byte kk into register x
        fn ld_vx_byte(&mut self) {
            self.registers[self.get_vx()] = self.get_u8()
        }


        // 0x7xkk - Add byte kk to register x
        fn add_vx_byte(&mut self) {
            self.registers(self.get_vx()) += self.get_u8();
        }

        // 0x8xy0 - Load register y into register x
        fn ld_vx_vy(&mut self) {
            self.registers[self.get_vx()] = registers[self.get_vy()]
        }


        // 0x8xy1 - Set register x to bitwise or with register y
        fn or_vx_vy(&mut self) {
            self.registers[self.get_vx()] |= self.registers[self.get_vy()]
        }

        // 0x8xy2 - Set register x to bitwise and with register y
        fn and_vx_vy(&mut self) {
            self.registers[self.get_vx()] &= self.registers[self.get_vy()]
        }

        // 0x8xy3 - Set register x to bitwise xor with register y
        fn xor_vx_vy(&mut self) {
            self.registers[self.get_vx()] ^= self.registers[self.get_vy()]
        }

        // 0x8xy4 - Add register y to register x, set register F to 1 if carry
        fn add_vx_vy(&mut self) {
            let sum: u16 = self.registers[self.get_vx()] + self.registers[self.get_vy()];
            self.registers[0xf] = if sum > 255 { 1 } else { 0 };
            self.registers[self.get_vx()] = sum & 0xff
        }

        // 0x8xy5 - Subtract register y from register x, set register F to 1 if register x > register y
        fn sub_vx_vy(&mut self) {
            self.registers[0xf] = if self.get_vx() > self.get_vy() { 1 } else { 0 };
            self.registers[self.get_vx()] -= self.registers[self.get_vy()]
        }

        // 0x8xy6 - Set register f to lsb and shift register x right 1
        fn shr_vx_vy(&mut self) {
            self.registers[0xf] = self.registers[self.get_vx()] & 0x1;
            self.registers[self.get_vx()] >>= 1;
        }

        // 0x8xy7 - subtract Vx from Vy and store in Vx, if Vy > Vx then Vf = 1
        fn subn_vx_vy(&mut self) {
            self.registers[0xf] = if self.registers[self.get_vy()] > self.registers[self.get_vx()] { 1 } else { 0 };
            self.registers[self.get_vx()] = self.registers[self.get_vy()] - self.registers[self.get_vx()]
        }

        // 0x8xy8 - Set register f to msb and shift register x left 1
        fn shl_vx_vy(&mut self) {
            self.registers[0xf] = (self.registers[self.get_vx()] & 0x80) >> 7;
            self.registers[self.get_vx()] <<= 1;
        }


        // 0x9xy0 - Skip next operation if Vx != Vy
        fn sne_vx_vy(&mut self) {
            if self.registers[self.get_vx()] != self.registers[self.get_vy()] {
                self.pc += 2;
            }
        }

        // 0xAnnn - Index is set to nnn
        fn ld_i_addr(&mut self) {
            self.index = self.get_u16();
        }

        // 0xBnnn - PC is set to V0 + nnn
        fn jp_v0_addr(&mut self) {
            self.pc = self.registers[0x0] as u16 + self.get_u16();
        }


        // 0xCxkk - Generate random byte, AND with kk, and then store in Vx
        fn rnd_vx_byte(&mut self) {
            let mut rng = rand::thread_rng();
            self.registers[self.get_vx()] = rng.gen::<u8>() & self.get_u8()
        }

        fn drw_vx_vy_nibble(&mut self) {}

        // 0xEx9E - skip if key is pressed
        fn skp_vx(&mut self) {
            if self.keypad[self.get_vx()] {
                self.pc += 2
            }
        }

        // 0xExA1 - skip if key is not pressed
        fn sknp_vx(&mut self) {
            if !self.keypad[self.get_vx()] {
                self.pc += 2
            }
        }

        // 0xFx07 - load delay into register vx
        fn ld_vx_dt(&mut self) {
            self.registers[self.get_vx()] = self.delay
        }

        fn ld_vx_k(&mut self) {
            unimplemented!()
        }

        fn ld_dt_vx(&mut self) {
            unimplemented!()
        }

        fn ld_st_vx(&mut self) {
            unimplemented!()
        }

        fn add_i_vx(&mut self) {
            unimplemented!()
        }

        fn ld_f_vx(&mut self) {
            unimplemented!()
        }

        fn ld_b_vx(&mut self) {
            unimplemented!()
        }

        fn ld_mem_vx(&mut self) {
            unimplemented!()
        }

        fn ld_vx_mem(&mut self) {
            unimplemented!()
        }
    }
}