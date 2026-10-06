use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 360;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

// Display scale factors: 64x32 scaled 10x becomes 640x320, leaving letterbox bars
const SCALE_X: usize = WINDOW_WIDTH / CHIP8_WIDTH; // 10
const SCALE_Y: usize = 10; // 10 (total 320px high)
const OFFSET_Y: usize = (WINDOW_HEIGHT - (CHIP8_HEIGHT * SCALE_Y)) / 2; // 20px top/bottom padding

pub struct Platform {
    window: Window,
    buffer: Vec<u32>,
}

impl Platform {
    pub fn new() -> Self {
        let window = Window::new(
            "CHIP-8 Emulator",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create minifb window: {e}");
        });

        let buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

        Platform { window, buffer }
    }

    // Check if the window is open and user hasn't pressed Escape
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // Scale up the 64x32 video buffer into the 640x360 window buffer
    pub fn update(&mut self, chip8_video: &[u32; 64 * 32]) {
        // Clear window buffer (black background)
        self.buffer.fill(0);

        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                let pixel = chip8_video[y * CHIP8_WIDTH + x];
                let color = if pixel != 0 { 0x00FFFFFF } else { 0x00000000 };

                // Draw a 10x10 block for each CHIP-8 pixel with vertical centering
                for dy in 0..SCALE_Y {
                    let win_y = OFFSET_Y + (y * SCALE_Y) + dy;
                    for dx in 0..SCALE_X {
                        let win_x = (x * SCALE_X) + dx;
                        self.buffer[win_y * WINDOW_WIDTH + win_x] = color;
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
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
