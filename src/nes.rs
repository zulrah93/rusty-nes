use crate::cpu;

use std::cell::{Cell,RefCell};



pub struct Nes {
    pub program_counter : Cell<u16>,
    pub stack_pointer : Cell<u8>,
    pub A : Cell<u8>, //Accumulator
    pub X : Cell<u8>, // Index-register
    pub Y : Cell<u8>, // Index-register
    pub processor_status_flag : Cell<u8>,
    pub memory : RefCell<[u8;65536]> // The 64k of memory first 2KB is NES RAM rest is from PPU and APU.
}

impl Nes {
    pub fn new() -> Self {
        Nes { A: Cell::new(0), X: Cell::new(0), Y: Cell::new(0), program_counter: Cell::new(0), stack_pointer: Cell::new(0), processor_status_flag: Cell::new(0), memory: RefCell::new([0 as u8;65536]) }
    }
}

