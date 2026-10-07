use std::{
    process::exit,
    time::{Duration, Instant},
};

use crate::{chip8::Chip8, video::Platform};
use clap::Parser;

mod chip8;
mod fontset;
mod opcode;
mod video;

#[derive(Parser)]
struct Cli {
    /// Scale of the window
    video_scale: usize,
    /// Tickrate / second ( CPU Cycle )
    tickrate: usize,
    /// Path to the rom to launch
    rom: std::path::PathBuf,
}

fn main() {
    println!("Hello, chip8 world!");
    let args = Cli::parse();
    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom(args.rom) {
        println!("Error : {e}");
        exit(1)
    }

    let mut platform = Platform::new(args.video_scale);
    let mut last_tick = Instant::now();

    while platform.is_open() {
        let now = Instant::now();
        platform.update_keypad(&mut chip8.keypad);
        // TODO: make cycle delay
        if now.duration_since(last_tick) >= Duration::from_secs(1) / args.tickrate as u32 {
            chip8.tick();
            platform.update(&chip8.video);
            last_tick = now;
        }
    }
}
