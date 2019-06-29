use ggez::{Context, GameResult,GameError};
use ggez::event::EventHandler;
use crate::img::Bitmap;

pub struct NesFrontend {
    // Your state here...
    bmp : Bitmap
}

impl NesFrontend {
    pub fn new() -> GameResult<NesFrontend> {
        if let Ok(bmp) = Bitmap::new(256, 240) {
            bmp.fill_color((0, 255, 0, 255));
            let nes_frontend = NesFrontend { bmp: bmp  };
            Ok(nes_frontend)
        }
        else {
            Err(GameError::ConfigError("Failed to create bitmap!".to_string()))
        }
    }
}

impl EventHandler for NesFrontend {

    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult<()> {
        // Draw code here...
        self.bmp.draw(_context)
    }
}