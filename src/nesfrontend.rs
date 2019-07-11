use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use crate::rom::Rom;

pub struct NesFrontend {
    // Your state here...
    rom : Rom
}

impl NesFrontend {
    pub fn new(rom : Rom) -> GameResult<NesFrontend> {
        let nes_frontend = NesFrontend { rom: rom  };
        Ok(nes_frontend)
    }
}

impl EventHandler for NesFrontend {

    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        // Draw code here...
        let bmp = self.rom.vrom_bmps.first().expect("Empty VROM Bitmaps!");
        bmp.draw(context)
    }
}