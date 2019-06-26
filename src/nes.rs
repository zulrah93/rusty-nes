use std::cell::{Cell,RefCell};
use std::collections::HashMap;
use std::ops::Index;

pub struct Nes {
    program_counter : Cell<u16>,
    stack_pointer : Cell<u8>,
    A : Cell<u8>, //Accumulator
    X : Cell<u8>, // Index-register
    Y : Cell<u8>, // Index-register
    processor_status_flag : Cell<u8>,
    memory : RefCell<[u8;65536]> // The 64k of memory first 2KB is NES RAM rest is from PPU and APU.
}

impl Nes {
    pub fn new() -> Self {
        Nes { A: Cell::new(0), X: Cell::new(0), Y: Cell::new(0), program_counter: Cell::new(0), stack_pointer: Cell::new(0), processor_status_flag: Cell::new(0), memory: RefCell::new([0 as u8;65536]) }
    }
}

type opcode_callback = fn(&Nes);

struct CPU {
   opcodes : HashMap<u8,opcode_callback> // Opcode mapped to function that executes 
}

impl CPU {
    pub fn new() -> Self {
        let mut opcodes : HashMap<u8,opcode_callback> = HashMap::new();
        /*Instruction Implementations*/
        opcodes.insert(0xa9, instruction_lda_immediate);
        opcodes.insert(0xa5, instruction_lda_zero_page);
        opcodes.insert(0xb5, instruction_lda_zero_page_x);
        opcodes.insert(0xad, instruction_lda_absolute);
        opcodes.insert(0xbd, instruction_lda_absolute_x);
        opcodes.insert(0xb9, instruction_lda_absolute_y);
        opcodes.insert(0xa1, instruction_lda_indirect_x);
        opcodes.insert(0xb1, instruction_lda_indirect_indexed);
        CPU {opcodes: opcodes}
    }

    pub fn execute(&self, opcode : u8) -> Option<opcode_callback> {
        if self.opcodes.contains_key(&opcode) {
            if let Some(callback) = self.opcodes.get(&opcode) {
                Some(*callback)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

fn update_processor_status_flag(operand : u16, processor_status_flag : &Cell<u8>) {
    
        if operand == 0 { 
            processor_status_flag.set(processor_status_flag.get() & 0x7d); // Set the zero flag bit which is the second bit.
        }
        else if operand >= 0x80 && operand <= 0xff { // If negative set negative flag bit which is the 7th bit.
            processor_status_flag.set(processor_status_flag.get() & 0x40);
        }
}

//LDA instructions
fn instruction_lda_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.A.set(operand as u8);
   nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_lda_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 69; // Set the operand to 69 😊
    }
    nes.program_counter.set(0);
    instruction_lda_immediate(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[operand as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_lda_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 255;
        memory[255] = 69;
    }
    nes.program_counter.set(0);
    instruction_lda_zero_page(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let X = nes.X.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[((operand+X) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_lda_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.X.set(1);
    }
    nes.program_counter.set(0);
    instruction_lda_zero_page_x(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[operand as usize]);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_lda_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 69;
    }
    nes.program_counter.set(0);
    instruction_lda_absolute(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let X = nes.X.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[(operand + X) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.X.set(255);
    }
    nes.program_counter.set(0);
    instruction_lda_absolute_x(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_absolute_y(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let Y = nes.Y.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[(operand + Y) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_absolute_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.Y.set(255);
    }
    nes.program_counter.set(0);
    instruction_lda_absolute_y(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_indirect_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let X = nes.X.get() as u16;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[memory[((operand + X) % 256) as usize] as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

fn instruction_lda_indirect_indexed(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let Y = nes.Y.get();
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.A.set(memory[(memory[operand as usize] + Y) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}



