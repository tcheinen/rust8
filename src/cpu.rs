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

use crate::State;

trait CPUOperationAppliable {
    fn clear(&mut self);
    fn ret(&mut self);
    fn jump(&mut self);
    fn call(&mut self);
    fn skip_equal_byte(&mut self);
    fn skip_neq_byte(&mut self);
    fn skip_equal(&mut self);
    fn load_byte(&mut self);
    fn add_byte(&mut self);
    fn load(&mut self);
    fn or(&mut self);
    fn and(&mut self);
    fn xor(&mut self);
    fn add(&mut self);
    fn sub(&mut self);
    fn shr(&mut self);
    fn subn(&mut self);
    fn shl(&mut self);
    fn skip_neq(&mut self);
    fn load_index_byte(&mut self);
    fn jump_r0_byte(&mut self);
    fn rand(&mut self);
    fn draw(&mut self);
    fn skip_key(&mut self);
    fn skip_neq_key(&mut self);
    fn load_reg_delay(&mut self);
    fn load_reg_key(&mut self);
    fn load_delay_reg(&mut self);
    fn load_sound_reg(&mut self);
    fn add_i_reg(&mut self);
    fn load_sprite(&mut self);
    fn load_bcd(&mut self);
    fn load_mem_registers(&mut self);
    fn load_registers_mem(&mut self);
}

impl CPUOperationAppliable for State {
    // 0x00E0 - Clear the display
    fn clear(&mut self) {
        vram = [false; 64 * 32]
    }

    // 0x00EE - Return from a function
    fn ret(&mut self) {
        sp -= 1;
        pc = stack[sp];
    }

    // 0x1nnn - Jump to location nnn
    fn jump(&mut self) {
        pc = get_u16();
    }

    // 0x2nnn - Call function at nnn
    fn call(&mut self) {
        stack[sp] = pc;
        sp += 1;
        pc = get_u16();
    }

    // 0x3xkk - Skip next operation if register x is equal to kk
    fn skip_equal_byte(&mut self) {
        if registers[get_vx() == get_u8()] {
            pc += 2;
        }
    }

    // 0x4xkk - Skip next operation if register x is not equal to kk
    fn skip_neq_byte(&mut self) {
        if registers[get_vx() != get_u8()] {
            pc += 2;
        }
    }

    // 0x5xy0 - Skip next operation if register x is equal to register y
    fn skip_equal(&mut self) {
        if get_vx() == get_vy() {
            pc += 2;
        }
    }

    // 0x6xkk - Load byte kk into register x
    fn load_byte(&mut self) {
        registers[get_vx()] = get_u8()
    }


    // 0x7xkk - Add byte kk to register x
    fn add_byte(&mut self) {
        registers(get_vx()) += get_u8();
    }

    // 0x8xy0 - Load register y into register x
    fn load(&mut self) {
        registers[get_vx()] = registers[get_vy()]
    }


    // 0x8xy1 - Set register x to bitwise or with register y
    fn or(&mut self) {
        registers[get_vx()] |= registers[get_vy()]
    }

    // 0x8xy2 - Set register x to bitwise and with register y
    fn and(&mut self) {
        registers[get_vx()] &= registers[get_vy()]
    }

    // 0x8xy3 - Set register x to bitwise xor with register y
    fn xor(&mut self) {
        registers[get_vx()] ^= registers[get_vy()]
    }

    // 0x8xy4 - Add register y to register x, set register F to 1 if carry
    fn add(&mut self) {
        let sum: u16 = registers[get_vx()] + registers[get_vy()];
        registers[0xf] = if sum > 255 { 1 } else { 0 };
        registers[get_vx()] = sum & 0xff
    }

    // 0x8xy5 - Subtract register y from register x, set register F to 1 if register x > register y
    fn sub(&mut self) {
        registers[0xf] = if get_vx() > get_vy() { 1 } else { 0 };
        registers[get_vx()] -= registers[get_vy()]
    }

    // 0x8xy6 - Set register f to lsb and shift register x right 1
    fn shr(&mut self) {
        registers[0xf] = registers[get_vx()] & 0x1;
        registers[get_vx()] >>= 1;
    }

    // 0x8xy7 - subtract Vx from Vy and store in Vx, if Vy > Vx then Vf = 1
    fn subn(&mut self) {
        registers[0xf] = if registers[get_vy()] > registers[get_vx()] { 1 } else { 0 };
        registers[get_vx()] = registers[get_vy()] - registers[get_vx()]
    }

    // 0x8xy8 - Set register f to msb and shift register x left 1
    fn shl(&mut self) {
        registers[0xf] = (registers[get_vx()] & 0x80) >> 7;
        registers[get_vx()] <<= 1;
    }


    // 0x9xy0 - Skip next operation if Vx != Vy
    fn skip_neq(&mut self) {
        if registers[get_vx()] != registers[get_vy()] {
            pc += 2;
        }
    }

    // 0xAnnn - Index is set to nnn
    fn load_index_byte(&mut self) {
       index = get_u16();
    }

    // 0xBnnn - PC is set to V0 + nnn
    fn jump_r0_byte(&mut self) {
        pc = registers[0x0] + get_u16();
    }


    // 0xCxkk - Generate random byte, AND with kk, and then store in Vx
    fn rand(&mut self) {
        let mut rng = rand::thread_rng();
        registers[self.get_vx()] = rng.gen::<u8>() & self.get_u8()
    }

    fn draw(&mut self) {
        unimplemented!()
    }

    fn skip_key(&mut self) {
        unimplemented!()
    }

    fn skip_neq_key(&mut self) {
        unimplemented!()
    }

    fn load_reg_delay(&mut self) {
        unimplemented!()
    }

    fn load_reg_key(&mut self) {
        unimplemented!()
    }

    fn load_delay_reg(&mut self) {
        unimplemented!()
    }

    fn load_sound_reg(&mut self) {
        unimplemented!()
    }

    fn add_i_reg(&mut self) {
        unimplemented!()
    }

    fn load_sprite(&mut self) {
        unimplemented!()
    }

    fn load_bcd(&mut self) {
        unimplemented!()
    }

    fn load_mem_registers(&mut self) {
        unimplemented!()
    }

    fn load_registers_mem(&mut self) {
        unimplemented!()
    }
}