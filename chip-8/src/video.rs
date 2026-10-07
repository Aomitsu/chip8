use minifb::{Key, Window, WindowOptions};

use crate::chip8::{VIDEO_HEIGHT, VIDEO_WIDTH};

pub struct Platform {
    window: Window,
    buffer: Vec<u32>,
    canvas_size: usize,
}

impl Platform {
    pub fn new(canvas_size: usize) -> Self {
        let window = Window::new(
            "CHIP-8 Emulator",
            VIDEO_WIDTH * canvas_size,
            VIDEO_HEIGHT * canvas_size,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create minifb window: {e}");
        });

        let buffer = vec![0; VIDEO_WIDTH * canvas_size * VIDEO_HEIGHT * canvas_size];

        Platform {
            window,
            buffer,
            canvas_size,
        }
    }

    // Check if the window is open and user hasn't pressed Escape
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // Scale up the 64x32 video buffer into the 640x360 window buffer
    pub fn update(&mut self, chip8_video: &[u32; 64 * 32]) {
        // Clear window buffer (black background)
        self.buffer.fill(0);

        let window_width = VIDEO_WIDTH * self.canvas_size;

        for y in 0..VIDEO_HEIGHT {
            for x in 0..VIDEO_WIDTH {
                let pixel = chip8_video[y * VIDEO_WIDTH + x];
                let color = if pixel != 0 { 0x00FFFFFF } else { 0x00000000 };

                // Draw a canvas_size x canvas_size block for each CHIP-8 pixel
                for dy in 0..self.canvas_size {
                    let win_y = (y * self.canvas_size) + dy;
                    for dx in 0..self.canvas_size {
                        let win_x = (x * self.canvas_size) + dx;
                        self.buffer[win_y * window_width + win_x] = color;
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&self.buffer, window_width, VIDEO_HEIGHT * self.canvas_size)
            .unwrap();
    }

    // Map physical keyboard keys to CHIP-8 hexadecimal keypad (0x0 to 0xF)
    pub fn update_keypad(&self, keypad: &mut [u8; 16]) {
        const KEY_MAPPINGS: [(usize, Key); 16] = [
            (0x1, Key::Key1),
            (0x2, Key::Key2),
            (0x3, Key::Key3),
            (0xC, Key::Key4),
            (0x4, Key::Q),
            (0x5, Key::W),
            (0x6, Key::E),
            (0xD, Key::R),
            (0x7, Key::A),
            (0x8, Key::S),
            (0x9, Key::D),
            (0xE, Key::F),
            (0xA, Key::Z),
            (0x0, Key::X),
            (0xB, Key::C),
            (0xF, Key::V),
        ];

        for &(chip8_key, physical_key) in &KEY_MAPPINGS {
            keypad[chip8_key] = if self.window.is_key_down(physical_key) {
                1
            } else {
                0
            };
        }
    }
}
