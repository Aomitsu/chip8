#![allow(non_snake_case)]

use crate::{
    chip8::{Chip8, VIDEO_HEIGHT, VIDEO_WIDTH},
    fontset::FONTSET_START_ADDRESS,
};

impl Chip8 {
    /// 00E0: CLS - Clear the display
    pub fn OP_00E0(&mut self) {
        self.video.fill(0);
    }
    /// 00EE: RET - Return from a subroutine
    pub fn OP_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }
    /// 1nnn: JP addr - Jump to location nnn
    /// The interpreter sets the program counter to nnn.
    pub fn OP_1nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = address;
    }
    /// 2nnn: CALL addr - Call subroutine at nnn
    pub fn OP_2nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }
    /// 3xkk: SE Vx, byte - Skip next instruction if Vx == kk
    pub fn OP_3xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        if self.registers[vx] == byte {
            self.pc += 2;
        }
    }
    /// 4xkk: SNE Vx, byte - Skip next instruction if Vx != kk
    pub fn OP_4xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        if self.registers[vx] != byte {
            self.pc += 2;
        }
    }
    /// 5xy0: SE Vx, Vy - Skip next instruction if Vx == Vy.
    pub fn OP_5xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vx] == self.registers[vy] {
            self.pc += 2;
        }
    }
    /// 6xkk: LD Vx, byte - Set Vx = kk.
    pub fn OP_6xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = byte;
    }
    /// 7xkk: ADD Vx, byte - Set Vx = Vx + kk.
    pub fn OP_7xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = self.registers[vx].wrapping_add(byte);
    }
    /// 8xy0: LD Vx, Vy - Set Vx = Vy.
    pub fn OP_8xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] = self.registers[vy];
    }
    /// 8xy1: OR Vx, Vy - Set Vx = Vx OR Vy.
    pub fn OP_8xy1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] |= self.registers[vy];
    }
    /// 8xy2: AND Vx, Vy - Set Vx = Vx AND Vy.
    pub fn OP_8xy2(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] &= self.registers[vy];
    }
    /// 8xy3: XOR Vx, Vy - Set Vx = Vx AND Vy.
    pub fn OP_8xy3(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] ^= self.registers[vy];
    }
    /// 8xy4: ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    pub fn OP_8xy4(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        let sum: u16 = self.registers[vx] as u16 + self.registers[vy] as u16;

        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] = sum as u8;
    }
    /// 8xy5: SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    pub fn OP_8xy5(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        let carry = if self.registers[vx] >= self.registers[vy] {
            1
        } else {
            0
        };

        self.registers[vx] = self.registers[vx].wrapping_sub(self.registers[vy]);
        self.registers[0xF] = carry;
    }
    /// 8xy6: SHR Vx - Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    pub fn OP_8xy6(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[0xF] = self.registers[vx] & 0x1;
        self.registers[vx] >>= 1;
    }
    /// 8xy7: SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    pub fn OP_8xy7(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vy] >= self.registers[vx] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] = self.registers[vy].wrapping_sub(self.registers[vx]);
    }
    /// 8xyE: SHL Vx - Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    pub fn OP_8xyE(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;

        self.registers[vx] <<= 1;
    }
    /// 9xy0: SNE Vx, Vy - Skip next instruction if Vx != Vy.
    pub fn OP_9xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }
    /// Annn: LD I, addr - Set I = nnn.
    pub fn OP_Annn(&mut self) {
        let address = self.opcode & 0x0FFF;

        self.index = address;
    }
    /// Bnnn: JP V0, addr - Jump to location nnn + V0.
    pub fn OP_Bnnn(&mut self) {
        let address = self.opcode & 0x0FFF;

        self.pc = self.registers[0] as u16 + address;
    }
    /// Cxkk: RND Vx, byte - Set Vx = random byte AND kk.
    pub fn OP_Cxkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = Chip8::get_random_number() & byte;
    }
    /// Dxyn: DRW Vx, Vy, nibble - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    pub fn OP_Dxyn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let height = self.opcode & 0x000F;

        let xpos = (self.registers[vx] as usize) % VIDEO_WIDTH;
        let ypos = (self.registers[vy] as usize) % VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in 0..height as usize {
            let current_y = ypos + row;
            if current_y >= VIDEO_HEIGHT {
                break;
            }

            let sprite_byte = self.memory[(self.index as usize) + row];

            for col in 0..8 {
                let current_x = xpos + col;
                // Clip horizontally if sprite extends past the right edge
                if current_x >= VIDEO_WIDTH {
                    break;
                }

                if (sprite_byte & (0x80 >> col)) != 0 {
                    let idx = current_y * VIDEO_WIDTH + current_x;

                    if self.video[idx] != 0 {
                        self.registers[0xF] = 1;
                    }

                    self.video[idx] ^= 0xFFFFFFFF;
                }
            }
        }
    }
    /// Ex9E: SKP Vx - Skip next instruction if key with the value of Vx is pressed.
    pub fn OP_Ex9E(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx];

        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }
    /// ExA1: SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
    pub fn OP_ExA1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx];

        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }
    /// Fx07: LD Vx, DT - Set Vx = delay timer value.
    pub fn OP_Fx07(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[vx] = self.delay_timer;
    }
    /// Fx0A: LD Vx, K - Wait for a key press, store the value of the key in Vx.
    pub fn OP_Fx0A(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        // Find the index of the first currently pressed key
        if let Some(key_index) = self.keypad.iter().position(|&key| key != 0) {
            self.registers[vx] = key_index as u8;
        } else {
            // Repeat this instruction next cycle until a key is pressed
            self.pc -= 2;
        }
    }
    /// Fx15: LD DT, Vx - Set delay timer = Vx.
    pub fn OP_Fx15(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.delay_timer = self.registers[vx];
    }
    /// Fx18: LD ST, Vx - Set sound timer = Vx.
    pub fn OP_Fx18(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.sound_timer = self.registers[vx];
    }
    /// Fx1E: ADD I, Vx - Set I = I + Vx.
    pub fn OP_Fx1E(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.index = self.index.wrapping_add(self.registers[vx] as u16);
    }
    /// Fx29: LD F, Vx - Set I = location of sprite for digit Vx.
    pub fn OP_Fx29(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let digit = self.registers[vx];

        self.index = FONTSET_START_ADDRESS + (5 * digit as u16)
    }
    /// Fx33: LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    pub fn OP_Fx33(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let mut value = self.registers[vx];
        let idx = self.index as usize;

        // Ones-place
        self.memory[idx + 2] = value % 10;
        value /= 10;

        // Tens-place
        self.memory[idx + 1] = value % 10;
        value /= 10;

        // Hundreds-place
        self.memory[idx] = value % 10;
    }
    /// Fx55: LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
    pub fn OP_Fx55(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let idx = self.index as usize;

        for i in 0..=vx {
            self.memory[idx + i] = self.registers[i];
        }
    }
    /// Fx65: LD Vx, [I] - Read registers V0 through Vx from memory starting at location I.
    pub fn OP_Fx65(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let idx = self.index as usize;

        for i in 0..=vx {
            self.registers[i] = self.memory[idx + i];
        }
    }
}
