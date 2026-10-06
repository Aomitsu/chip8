use std::{fs, io, path::Path};

use rand::RngExt;

use crate::fontset::{FONTSET, FONTSET_SIZE, FONTSET_START_ADDRESS};

/// Standard CHIP-8 programs start at address 0x200 (512)
const START_ADDRESS: u16 = 0x200;

pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

const VIDEO_DISPLAY_SIZE: usize = VIDEO_WIDTH * VIDEO_HEIGHT;

/// Chip8 emulator
pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video: [u32; 64 * 32],
    pub opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; VIDEO_DISPLAY_SIZE],
            opcode: 0,
        };

        let fontset_end_address = FONTSET_START_ADDRESS + FONTSET_SIZE as u16;
        chip8.memory[FONTSET_START_ADDRESS as usize..fontset_end_address as usize]
            .copy_from_slice(&FONTSET);

        chip8
    }

    pub fn execute_instruction(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x00FF {
                0x00E0 => self.OP_00E0(),
                0x00EE => self.OP_00EE(),
                _ => {}
            },
            0x1000 => self.OP_1nnn(),
            0x2000 => self.OP_2nnn(),
            0x3000 => self.OP_3xkk(),
            0x4000 => self.OP_4xkk(),
            0x5000 => self.OP_5xy0(),
            0x6000 => self.OP_6xkk(),
            0x7000 => self.OP_7xkk(),
            0x8000 => match self.opcode & 0x000F {
                0x0 => self.OP_8xy0(),
                0x1 => self.OP_8xy1(),
                0x2 => self.OP_8xy2(),
                0x3 => self.OP_8xy3(),
                0x4 => self.OP_8xy4(),
                0x5 => self.OP_8xy5(),
                0x6 => self.OP_8xy6(),
                0x7 => self.OP_8xy7(),
                0xE => self.OP_8xyE(),
                _ => {}
            },
            0x9000 => self.OP_9xy0(),
            0xA000 => self.OP_Annn(),
            0xB000 => self.OP_Bnnn(),
            0xC000 => self.OP_Cxkk(),
            0xD000 => self.OP_Dxyn(),
            0xE000 => match self.opcode & 0x00FF {
                0x009E => self.OP_Ex9E(),
                0x00A1 => self.OP_ExA1(),
                _ => {}
            },
            0xF000 => match self.opcode & 0x00FF {
                0x0007 => self.OP_Fx07(),
                0x000A => self.OP_Fx0A(),
                0x0015 => self.OP_Fx15(),
                0x0018 => self.OP_Fx18(),
                0x001E => self.OP_Fx1E(),
                0x0029 => self.OP_Fx29(),
                0x0033 => self.OP_Fx33(),
                0x0055 => self.OP_Fx55(),
                0x0065 => self.OP_Fx65(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, file: P) -> io::Result<()> {
        let rom = fs::read(file)?;
        let max_size = self.memory.len() - START_ADDRESS as usize;

        if rom.len() > max_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM size exceeds available memory",
            ));
        }

        let end_address = START_ADDRESS + rom.len() as u16;
        self.memory[START_ADDRESS as usize..end_address as usize].copy_from_slice(&rom);

        Ok(())
    }

    pub fn get_random_number() -> u8 {
        let mut rng = rand::rng();
        rng.random()
    }

    pub fn tick(&mut self) {
        // Fetch opcode from memory
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc as usize) + 1] as u16);

        // Increment before we execute anything.
        self.pc += 2;

        self.execute_instruction();

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
