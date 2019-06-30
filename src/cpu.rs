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
        opcodes.insert(0x86, instruction_stx_zero_page);
        opcodes.insert(0x96, instruction_stx_zero_page_y);
        opcodes.insert(0x8e, instruction_stx_absolute);
        opcodes.insert(0x84, instruction_sty_zero_page);
        opcodes.insert(0x94, instruction_sty_zero_page_x);
        opcodes.insert(0x8c, instruction_sty_absolute);
        opcodes.insert(0xaa, instruction_tax);
        opcodes.insert(0xa8, instruction_tay);
        opcodes.insert(0xba, instruction_tsx);
        opcodes.insert(0x8a, instruction_txa);
        opcodes.insert(0x9a, instruction_txs);
        opcodes.insert(0x98, instruction_tya);
        opcodes.insert(0x18, instruction_clc);
        opcodes.insert(0xd8, instruction_cld);
        opcodes.insert(0x58, instruction_cli);
        opcodes.insert(0xb8, instruction_clv);
        opcodes.insert(0x38, instruction_sec);
        opcodes.insert(0xf8, instruction_sed);
        opcodes.insert(0x78, instruction_sei);
        opcodes.insert(0x4c, instruction_jmp_absolute);
        opcodes.insert(0x6c, instruction_jmp_indirect);
        opcodes.insert(0xe6, instruction_inc_zeropage);
        opcodes.insert(0xf6, instruction_inc_zeropage_x);
        opcodes.insert(0xee, instruction_inc_absolute);
        opcodes.insert(0xfe, instruction_inc_absolute_x);
        opcodes.insert(0xe8, instruction_inx);
        opcodes.insert(0xc8, instruction_iny);
        opcodes.insert(0x20, instruction_jsr);
        opcodes.insert(0x60, instruction_rts);
        opcodes.insert(0x69, instruction_adc_immediate);
        opcodes.insert(0x65, instruction_adc_zeropage);
        opcodes.insert(0x75, instruction_adc_zeropage_x);
        opcodes.insert(0x6d, instruction_adc_absolute);
        opcodes.insert(0x7d, instruction_adc_absolute_x);
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

//STX Opcodes
fn instruction_stx_zero_page(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.X.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_stx_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.X.set(69); 
    instruction_stx_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_stx_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.X.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_stx_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.X.set(69); 
    instruction_stx_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}


fn instruction_stx_zero_page_y(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let Y = nes.Y.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + Y) % 256) as usize] = nes.X.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_stx_zero_page_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.X.set(69); 
    nes.Y.set(10);
    instruction_stx_zero_page_y(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[20], 69);
}

// STY Opcodes
fn instruction_sty_zero_page(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.Y.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sty_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.Y.set(69); 
    instruction_sty_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_sty_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.Y.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sty_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.Y.set(69); 
    instruction_sty_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}


fn instruction_sty_zero_page_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let X = nes.X.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + X) % 256) as usize] = nes.Y.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sty_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.Y.set(69); 
    nes.X.set(10);
    instruction_sty_zero_page_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[20], 69);
}

// TAX opcode

fn instruction_tax(nes : &Nes) {
    let A = nes.A.get();
    update_processor_status_flag(A as u16, &nes.processor_status_flag);
    nes.X.set(A);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tax() {
     let nes = Nes::new();
     nes.A.set(69);
     instruction_tax(&nes);
     assert_eq!(nes.X.get(), 69);
}


// TAY opcode

fn instruction_tay(nes : &Nes) {
    let A = nes.A.get();
    update_processor_status_flag(A as u16, &nes.processor_status_flag);
    nes.Y.set(A);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tay() {
     let nes = Nes::new();
     nes.A.set(69);
     instruction_tay(&nes);
     assert_eq!(nes.Y.get(), 69);
}

//TSX Opcode

fn instruction_tsx(nes : &Nes) {
    let stack_ptr = nes.stack_pointer.get();
    update_processor_status_flag(stack_ptr as u16, &nes.processor_status_flag);
    nes.X.set(stack_ptr);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tsx() {
     let nes = Nes::new();
     nes.stack_pointer.set(69);
     instruction_tsx(&nes);
     assert_eq!(nes.X.get(), 69);
}


// TXA opcode

fn instruction_txa(nes : &Nes) {
    let X = nes.X.get();
    update_processor_status_flag(X as u16, &nes.processor_status_flag);
    nes.A.set(X);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_txa() {
     let nes = Nes::new();
     nes.X.set(69);
     instruction_txa(&nes);
     assert_eq!(nes.A.get(), 69);
}


// TXS opcode

fn instruction_txs(nes : &Nes) {
    let X = nes.X.get();
    update_processor_status_flag(X as u16, &nes.processor_status_flag);
    nes.stack_pointer.set(X);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_txs() {
     let nes = Nes::new();
     nes.X.set(69);
     instruction_txs(&nes);
     assert_eq!(nes.stack_pointer.get(), 69);
}


// TYA opcode

fn instruction_tya(nes : &Nes) {
    let Y = nes.Y.get();
    update_processor_status_flag(Y as u16, &nes.processor_status_flag);
    nes.A.set(Y);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tya() {
     let nes = Nes::new();
     nes.Y.set(69);
     instruction_tya(&nes);
     assert_eq!(nes.A.get(), 69);
}

//Clear opcodes (these modify the processor status flag)

fn instruction_clc(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() & 0xfe);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_clc() {
    let nes = Nes::new();
    nes.processor_status_flag.set(1);
    instruction_clc(&nes);
    assert_eq!(nes.processor_status_flag.get(), 0);
}

fn instruction_cld(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() & 0xf7);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_cld() {
    let nes = Nes::new();
    nes.processor_status_flag.set(8); // 8 == b1000
    instruction_cld(&nes);
    assert_eq!(nes.processor_status_flag.get(), 0);
}

fn instruction_cli(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() & 0xfb);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_cli() {
    let nes = Nes::new();
    nes.processor_status_flag.set(4); // 3 == b100
    instruction_cli(&nes);
    assert_eq!(nes.processor_status_flag.get(), 0);
}

fn instruction_clv(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() & 0xdf);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_clv() {
    let nes = Nes::new();
    nes.processor_status_flag.set(32); // 3 == b100000
    instruction_clv(&nes);
    assert_eq!(nes.processor_status_flag.get(), 0);
}

// Set flag opcodes (these also modify the processor status flag)
fn instruction_sec(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_sec() {
    let nes = Nes::new();
    instruction_sec(&nes);
    assert_eq!(nes.processor_status_flag.get(), 1);
}

fn instruction_sed(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_sed() {
    let nes = Nes::new();
    instruction_sed(&nes);
    assert_eq!(nes.processor_status_flag.get(), 8);
}

fn instruction_sei(nes : &Nes)  {
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b100);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_sei() {
    let nes = Nes::new();
    instruction_sei(&nes);
    assert_eq!(nes.processor_status_flag.get(), 4);
}

//Jump Opcodes

fn instruction_jmp_absolute(nes : &Nes) {
     let pc = nes.program_counter.get() as usize;
     let memory = nes.memory.borrow();
     let high_byte = memory[pc+2] as u16; 
     let target = (high_byte << 8) | (memory[pc+1] as u16);
     nes.program_counter.set(target);
}

#[test]
fn test_instruction_jmp_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    instruction_jmp_absolute(&nes);
    assert_eq!(nes.program_counter.get(), 2048);
}

fn instruction_jmp_indirect(nes : &Nes) {
     let pc = nes.program_counter.get() as usize;
     let memory = nes.memory.borrow();
     let high_byte = memory[pc+2] as usize; 
     let address = (high_byte << 8) | (memory[pc+1] as usize);
     let high_byte = memory[address+1] as u16;
     println!("high_byte = {}", high_byte);
     let target = (high_byte << 8) | (memory[address] as u16);
     nes.program_counter.set(target);
}

#[test]
fn test_instruction_jmp_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2049] = 8;
    }
    instruction_jmp_indirect(&nes);
    assert_eq!(nes.program_counter.get(), 2048);
}

// INC Opcodes

fn instruction_inc_zeropage(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as usize;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[operand] as u16) + 1;
    memory[operand] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 255;
    }
    instruction_inc_zeropage(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[255], 1);
}

fn instruction_inc_zeropage_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as usize;
    let X = nes.X.get() as usize;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[(operand+X) % 256] as u16) + 1;
    memory[(operand+X) % 256] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
    }
    nes.X.set(1);
    instruction_inc_zeropage_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[255], 1);
}


fn instruction_inc_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = (((memory[pc+2] as usize) << 8)) | (memory[pc+1] as usize);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[operand] as u16) + 1;
    memory[operand] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    instruction_inc_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 1);
}

fn instruction_inc_absolute_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = (((memory[pc+2] as usize) << 8)) | (memory[pc+1] as usize);
    let X = nes.X.get() as usize;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[operand+X] as u16) + 1;
    memory[operand+X] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.X.set(1);
    instruction_inc_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2049], 1);
}

// INX and INY Opcodes

fn instruction_inx(nes : &Nes) {
    let X = nes.X.get() as u16;
    update_processor_status_flag(X, &nes.processor_status_flag);
    nes.X.set(((X+1) % 256) as u8);
}

#[test]
fn test_instruction_inx() {
    let nes = Nes::new();
    instruction_inx(&nes);
    assert_eq!(nes.X.get(),1);
}

fn instruction_iny(nes : &Nes) {
    let Y = nes.Y.get() as u16;
    update_processor_status_flag(Y, &nes.processor_status_flag);
    nes.Y.set(((Y+1) % 256) as u8);
}

#[test]
fn test_instruction_iny() {
    let nes = Nes::new();
    instruction_iny(&nes);
    assert_eq!(nes.Y.get(),1);
}

//Call Opcode

fn instruction_jsr(nes : &Nes) {
    let pc = nes.program_counter.get();
    let sp = (nes.stack_pointer.get() as usize) + 0x100;
    let mut memory = nes.memory.borrow_mut();
    memory[sp] = ((pc-1) & 0xff) as u8; // Push the two bytes of the current addres minus 1
    memory[sp+1] = ((pc-1) >> 8 & 0xff) as u8;
    nes.stack_pointer.set((sp+2) as u8);
    let high_byte = memory[(pc as usize)+2] as u16; 
    let target = (high_byte << 8) | (memory[(pc+1) as usize] as u16); // Get the 
    nes.program_counter.set(target);
}

#[test]
fn test_instruction_jsr() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[4] = 8;
    }
    nes.program_counter.set(2);
    let sp = nes.stack_pointer.get();
    instruction_jsr(&nes);
    assert_eq!(nes.program_counter.get(), 2048);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x100], 1);
    assert_eq!(sp+2, nes.stack_pointer.get());
}

// Return opcode

fn instruction_rts(nes : &Nes) {
    let sp = (nes.stack_pointer.get()-2) as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[sp+0x101] as u16;
    let return_address = (high_byte << 8) | (memory[sp+0x100] as u16);
    nes.stack_pointer.set(sp as u8);
    nes.program_counter.set(return_address);
}

#[test]
fn test_instruction_rts() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x101] = 8;
    }
    nes.stack_pointer.set(2);
    instruction_rts(&nes);
    assert_eq!(nes.program_counter.get(), 2048);
    assert_eq!(nes.stack_pointer.get(), 0);
}

// Add With Carry Opcodes
fn instruction_adc_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let A = nes.A.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  A + operand + carry;
    if ((A ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.A.set(0xff);
    }
    else {
        nes.A.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_adc_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[3] = 128;
    }
    nes.A.set(64);
    instruction_adc_immediate(&nes);
    assert_eq!(nes.A.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_immediate(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_adc_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[memory[pc+1] as usize] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let A = nes.A.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  A + operand + carry;
    if ((A ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.A.set(0xff);
    }
    else {
        nes.A.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_adc_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[64] = 64;
        memory[3] = 128;
        memory[128] = 182;
    }
    nes.A.set(64);
    instruction_adc_zeropage(&nes);
    assert_eq!(nes.A.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_zeropage(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let X = nes.X.get() as usize;
    let address = ((memory[pc+1] as usize) + X) % 256;
    let operand = memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let A = nes.A.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  A + operand + carry;
    if ((A ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.A.set(0xff);
    }
    else {
        nes.A.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_adc_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 63;
        memory[64] = 64;
        memory[3] = 127;
        memory[128] = 128;
    }
    nes.A.set(64);
    nes.X.set(1);
    instruction_adc_zeropage_x(&nes);
    assert_eq!(nes.A.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_zeropage_x(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let address =  (high_byte << 8) | (memory[pc+1] as usize);
    let operand = memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let A = nes.A.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  A + operand + carry;
    if ((A ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.A.set(0xff);
    }
    else {
        nes.A.set(sum as u8);
    }
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_adc_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 64;
        memory[3] = 8;
        memory[8] = 128;
    }
    nes.A.set(64);
    instruction_adc_absolute(&nes);
    assert_eq!(nes.A.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_absolute(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let X = nes.X.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + X;
    let operand = memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let A = nes.A.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  A + operand + carry;
    if ((A ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.A.set(0xff);
    }
    else {
        nes.A.set(sum as u8);
    }
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_adc_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
        memory[255] = 64;
        memory[3] = 8;
        memory[8] = 128;
    }
    nes.A.set(64);
    nes.X.set(1);
    instruction_adc_absolute_x(&nes);
    assert_eq!(nes.A.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_absolute_x(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}