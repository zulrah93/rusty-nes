extern crate ggez;

pub mod nesfrontend;
pub mod img;
pub mod nes;
pub mod cpu;

use ggez::ContextBuilder;
use ggez::event::{self};
use nesfrontend::NesFrontend;

fn main() {
    // Make a Context.
    let (context, event_loop) = &mut ContextBuilder::new("nes_frontend", "Daniel Lopez").window_mode(ggez::conf::WindowMode {
            width: 256.0,
            height: 240.0,
            ..Default::default()
        })
		.build()
		.expect("Failed to create context variable for NES frontend!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    match NesFrontend::new() {
        Ok(mut nes_frontend) => {
            // Run!
            event::run(context, event_loop, &mut nes_frontend).expect("Failed to run event loop!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

