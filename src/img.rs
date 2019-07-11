extern crate ggez;

use ggez::{Context,GameResult,GameError};
use ggez::graphics::{Image,draw,DrawParam,present};
use ggez::nalgebra::Point2;
use std::cell::RefCell;



pub struct Bitmap {
    bytes : RefCell<Vec<u8>>,
    width : u16,
    height : u16
}

impl Bitmap {
    pub fn new(width : u16, height : u16) -> Result<Self,String> {
           let size : usize = ((width as u32) * (height as u32) * 4) as usize;
           if size == 0 {
               Err("Bitmap cannot have 0 as either width or height!".to_string())
           }
           else {
               Ok(Bitmap { bytes: RefCell::new(vec![0; size]), width: width, height: height })
           }
    }

    pub fn from_bytes(width : u16, height : u16, bytes : Vec<u8>) -> Result<Self,String> {
        if bytes.is_empty() {
            Err("Bitmap cannot accept an empty vector!".to_string())
        }
        else {
            Ok(Bitmap {bytes: RefCell::new(bytes), width: width, height: height})
        }
    }

    pub fn draw(&self, context : &mut Context) -> GameResult<()> {
        if let Ok(buffer) = &self.to_image(context) {
            draw(context, buffer, DrawParam::default().dest(Point2::new(0.0,0.0)))?;
            present(context)
        }
        else {
            Err(GameError::RenderError("Failed to render NES buffer!".to_string()))
        }
    }

    pub fn to_image(&self, context : &mut Context) -> GameResult<Image> {
        
        Image::from_rgba8(context, self.width, self.height, &self.bytes.borrow())
    } 

    pub fn set_color(&self, x : u32, y : u32, rgb : (u8,u8,u8,u8)) {
        let mut bytes = self.bytes.borrow_mut();
        let row = y * 4;
        let col = x * 4;
        let index : usize = ((row * (self.width as u32)) + col) as usize;
        bytes[index] = rgb.0;
        bytes[index+1] = rgb.1;
        bytes[index+2] = rgb.2;
        bytes[index+3] = rgb.3; 
    }

    pub fn get_color(&self, x : u32, y : u32) -> (u8,u8,u8,u8) {
        let bytes = self.bytes.borrow_mut();
        let index : usize = ((y * (self.width as u32)) + (x * 4)) as usize;
        (bytes[index], bytes[index+1], bytes[index+2], bytes[index+3])
    }

    pub fn fill_color(&self, rgb : (u8,u8,u8,u8)) {
        let width = self.width - 1;
        let height = self.height - 1;
        for y in 0..height
        {
            for x in 0..width
            {
               self.set_color(x as u32, y as u32, rgb); 
            }
        }
    }

}

impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes // Two bitmaps are equal if there bytes are equal. Context doesn't matter.
    }
}