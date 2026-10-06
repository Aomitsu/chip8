use crate::{chip8::Chip8, video::Platform};

mod chip8;
mod fontset;
mod opcode;
mod video;

fn main() {
    println!("Hello, chip8 world!");
    let mut chip8 = Chip8::new();
    let mut platform = Platform::new();

    let _ = chip8.load_rom("./test_opcode.ch8");

    while platform.is_open() {
        platform.update_keypad(&mut chip8.keypad);
        // TODO: make cycle delay
        chip8.tick();
        platform.update(&mut chip8.video);
    }
}
