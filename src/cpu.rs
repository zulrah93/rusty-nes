use crate::nes::Nes;
use std::cell::Cell;
use std::ops::Index;
use std::collections::HashMap;

type opcode_callback = fn(&Nes);

pub struct CPU {
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
        opcodes.insert(0xa2, instruction_ldx_immediate);
        opcodes.insert(0xa6, instruction_ldx_zero_page);
        opcodes.insert(0xb6, instruction_ldx_zero_page_x);
        opcodes.insert(0xae, instruction_ldx_absolute);
        opcodes.insert(0xbe, instruction_ldx_absolute_x);
        opcodes.insert(0xa0, instruction_ldy_immediate);
        opcodes.insert(0xa4, instruction_ldy_zero_page);
        opcodes.insert(0xb4, instruction_ldy_zero_page_x);
        opcodes.insert(0xac, instruction_ldy_absolute);
        opcodes.insert(0xbc, instruction_ldy_absolute_x);
        opcodes.insert(0x85, instruction_sta_zero_page);
        opcodes.insert(0x95, instruction_sta_zero_page_x);
        opcodes.insert(0x8d, instruction_sta_absolute);
        opcodes.insert(0x9d, instruction_sta_absolute_x);
        opcodes.insert(0x99, instruction_sta_absolute_y);
        opcodes.insert(0x81, instruction_sta_indirect_x);
        opcodes.insert(0x91, instruction_sta_indirect_indexed);
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
            processor_status_flag.set(processor_status_flag.get() | 0x7d); // Set the zero flag bit which is the second bit.
        }
        else if operand >= 0x80 && operand <= 0xff { // If negative set negative flag bit which is the 7th bit.
            processor_status_flag.set(processor_status_flag.get() | 0x40);
        }
}

#[test]
fn test_update_processor_status_for_zero() {
    let nes = Nes::new();
    update_processor_status_flag(0, &nes.processor_status_flag);
    assert_ne!(nes.processor_status_flag.get() & 0x7d, 0x0);
}

#[test]
fn test_update_processor_status_for_negative() {
    let nes = Nes::new();
    update_processor_status_flag(0x80, &nes.processor_status_flag);
    assert_ne!(nes.processor_status_flag.get() & 0x40, 0x0);
}

//LDA Opcodes
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
    let address = ((operand + X) % 256) as usize;
    nes.A.set(memory[((memory[address+1] as usize) << 8) | (memory[address] as usize)] as u8);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_indirect_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 1;
        memory[256] = 8;
        memory[2048] = 69;
        nes.X.set(254);
    }
    nes.program_counter.set(0);
    instruction_lda_indirect_x(&nes);
    assert_eq!(nes.A.get(), 69);
}

fn instruction_lda_indirect_indexed(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let Y = nes.Y.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = ((memory[(operand+1) as usize] as usize) << 8) | (memory[operand as usize] as usize); 
    nes.A.set(memory[(address + Y) as usize] as u8);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 253;
        memory[254] = 8;
        memory[2048+254] = 69;
        nes.Y.set(254);
    }
    nes.program_counter.set(0);
    instruction_lda_indirect_indexed(&nes);
    assert_eq!(nes.A.get(), 69);
}

// LDY Opcodes
fn instruction_ldy_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.Y.set(operand as u8);
   nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_ldy_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 69; // Set the operand to 69 😊
    }
    nes.program_counter.set(0);
    instruction_ldy_immediate(&nes);
    assert_eq!(nes.Y.get(), 69);
}

fn instruction_ldy_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.Y.set(memory[operand as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldy_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 255;
        memory[255] = 69;
    }
    nes.program_counter.set(0);
    instruction_ldy_zero_page(&nes);
    assert_eq!(nes.Y.get(), 69);
}

fn instruction_ldy_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let X = nes.X.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.Y.set(memory[((operand+X) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldy_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.X.set(1);
    }
    nes.program_counter.set(0);
    instruction_ldy_zero_page_x(&nes);
    assert_eq!(nes.Y.get(), 69);
}

fn instruction_ldy_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.Y.set(memory[operand as usize]);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_ldy_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 69;
    }
    nes.program_counter.set(0);
    instruction_ldy_absolute(&nes);
    assert_eq!(nes.Y.get(), 69);
}

fn instruction_ldy_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let X = nes.X.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.Y.set(memory[(operand + X) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_ldy_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.X.set(255);
    }
    nes.program_counter.set(0);
    instruction_ldy_absolute_x(&nes);
    assert_eq!(nes.Y.get(), 69);
}

// LDX Opcodes
fn instruction_ldx_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.X.set(operand as u8);
   nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_ldx_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 69; // Set the operand to 69 😊
    }
    nes.program_counter.set(0);
    instruction_ldx_immediate(&nes);
    assert_eq!(nes.X.get(), 69);
}

fn instruction_ldx_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.X.set(memory[operand as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldx_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 255;
        memory[255] = 69;
    }
    nes.program_counter.set(0);
    instruction_ldx_zero_page(&nes);
    assert_eq!(nes.X.get(), 69);
}

fn instruction_ldx_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let X = nes.X.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.X.set(memory[((operand+X) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldx_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.X.set(1);
    }
    nes.program_counter.set(0);
    instruction_ldx_zero_page_x(&nes);
    assert_eq!(nes.X.get(), 69);
}

fn instruction_ldx_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.X.set(memory[operand as usize]);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_ldx_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 69;
    }
    nes.program_counter.set(0);
    instruction_ldx_absolute(&nes);
    assert_eq!(nes.X.get(), 69);
}

fn instruction_ldx_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let X = nes.X.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.X.set(memory[(operand + X) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_ldx_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.X.set(255);
    }
    nes.program_counter.set(0);
    instruction_ldx_absolute_x(&nes);
    assert_eq!(nes.X.get(), 69);
}

//STA Opcodes
fn instruction_sta_zero_page(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.A.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sta_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.A.set(69); 
    instruction_sta_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_sta_zero_page_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let X = nes.X.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + X) % 256) as usize] = nes.A.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sta_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.A.set(69); 
    nes.X.set(10);
    instruction_sta_zero_page_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[20], 69);
}

fn instruction_sta_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.A.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.A.set(69); 
    instruction_sta_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}

fn instruction_sta_absolute_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let X = nes.X.get() as usize;
    memory[(operand as usize) + X] = nes.A.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.A.set(69);
    nes.X.set(10); 
    instruction_sta_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2058], 69);
}

fn instruction_sta_absolute_y(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let Y = nes.Y.get() as usize;
    memory[(operand as usize) + Y] = nes.A.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.A.set(69);
    nes.Y.set(10); 
    instruction_sta_absolute_y(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2058], 69);
}


fn instruction_sta_indirect_x(nes : &Nes) {
    let mut memory =  nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let X = nes.X.get() as u16;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = ((operand + X) % 256) as usize;
    let upper = memory[address+1] as usize;
    let lower = memory[address] as usize;
    let indirect_address = (upper << 8) | lower;
    memory[indirect_address] = nes.A.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_indirect_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 1;
        memory[256] = 8;
        nes.X.set(254);
    }
    nes.A.set(69);
    instruction_sta_indirect_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}

fn instruction_sta_indirect_indexed(nes : &Nes) {
    let mut memory =  nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let Y = nes.Y.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = (((memory[(operand+1) as usize] as usize) << 8) | (memory[operand as usize] as usize)) + Y; 
    memory[address] = nes.A.get() as u8;
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 253;
        memory[254] = 8;
        nes.Y.set(254);
    }
    nes.A.set(69);
    instruction_sta_indirect_indexed(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048+254], 69);
}