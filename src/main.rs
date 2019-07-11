extern crate ggez;

pub mod nesfrontend;
pub mod img;
pub mod nes;
pub mod cpu;
pub mod rom;

use std::env::args;
use ggez::ContextBuilder;
use ggez::event::{self};
use ggez::graphics::set_window_title;
use nesfrontend::NesFrontend;

fn main() {
    if let Some(rom_path) = &args().nth(1) {
        if let Some(palette_file) = &args().nth(2) {
            // Make a Context.
            let (context, event_loop) = &mut ContextBuilder::new("nes_frontend", "Daniel Lopez").window_mode(ggez::conf::WindowMode {
                    width: 256.0,
                    height: 240.0,
                    ..Default::default()
                })
                .build()
                .expect("Failed to create context variable for NES frontend!");
            //Set the window title.
            set_window_title(context, &format!("RustyNes - {}", rom_path));

            //Load NES ROM
            let rom = rom::Rom::load(rom_path.to_string(), palette_file.to_string()).expect("Failed to load ROM!");

            // Create an instance of your event handler.
            // Usually, you should provide it with the Context object to
            // use when setting your game up.
            match NesFrontend::new(rom) {
                Ok(mut nes_frontend) => {
                    // Run!
                    event::run(context, event_loop, &mut nes_frontend).expect("Failed to run event loop!");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        else {
            println!("No palette file given!");
        }
    }
    else {
        println!("Error: No ROM Path Given!");
    }
}

