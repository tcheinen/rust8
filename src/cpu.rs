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