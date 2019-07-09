use crate::nes::Nes;
use std::cell::Cell;
use std::collections::HashMap;

type OpcodeCallback = fn(&Nes);

pub struct CPU {
   opcodes : HashMap<u8,OpcodeCallback> // Opcode mapped to function that executes 
}

impl CPU {
    pub fn new() -> Self {
        let mut opcodes : HashMap<u8,OpcodeCallback> = HashMap::new();
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
        opcodes.insert(0x61, instruction_adc_index_indirect);
        opcodes.insert(0x71, instruction_adc_indirect_indexed);
        opcodes.insert(0xe9, instruction_sbc_immediate);
        opcodes.insert(0xe5, instruction_sbc_zeropage);
        opcodes.insert(0xf5, instruction_sbc_zeropage_x);
        opcodes.insert(0xed, instruction_sbc_absolute);
        opcodes.insert(0xfd, instruction_sbc_absolute_x);
        opcodes.insert(0xe1, instruction_sbc_index_indirect);
        opcodes.insert(0xf1, instruction_sbc_indirect_indexed);
        opcodes.insert(0xea, instruction_nop);
        opcodes.insert(0x48, instruction_pha);
        opcodes.insert(0x08, instruction_php);
        opcodes.insert(0x68, instruction_pla);
        opcodes.insert(0x28, instruction_plp);
        opcodes.insert(0x29, instruction_and_immediate);
        opcodes.insert(0x25, instruction_and_zeropage);
        opcodes.insert(0x35, instruction_and_zeropage_x);
        opcodes.insert(0x2d, instruction_and_absolute);
        opcodes.insert(0x3d, instruction_and_absolute_x);
        opcodes.insert(0x21, instruction_and_index_indirect);
        opcodes.insert(0x31, instruction_and_indirect_indexed);
        opcodes.insert(0x49, instruction_eor_immediate);
        opcodes.insert(0x45, instruction_eor_zeropage);
        opcodes.insert(0x55, instruction_eor_zeropage_x);
        opcodes.insert(0x4d, instruction_eor_absolute);
        opcodes.insert(0x5d, instruction_eor_absolute_x);
        opcodes.insert(0x41, instruction_eor_index_indirect);
        opcodes.insert(0x51, instruction_eor_indirect_indexed);
        opcodes.insert(0x09, instruction_ora_immediate);
        opcodes.insert(0x05, instruction_ora_zeropage);
        opcodes.insert(0x15, instruction_ora_zeropage_x);
        opcodes.insert(0x0d, instruction_ora_absolute);
        opcodes.insert(0x1d, instruction_ora_absolute_x);
        opcodes.insert(0x01, instruction_ora_index_indirect);
        opcodes.insert(0x11, instruction_ora_indirect_indexed);
        opcodes.insert(0x90, instruction_bcc);
        opcodes.insert(0xb0, instruction_bcs);
        opcodes.insert(0xf0, instruction_beq);
        opcodes.insert(0x24, instruction_bit_zeropage);
        opcodes.insert(0x2c, instruction_bit_absolute);
        opcodes.insert(0x30, instruction_bmi);
        opcodes.insert(0xd0, instruction_bne);
        opcodes.insert(0x10, instruction_bpl);
        opcodes.insert(0x50, instruction_bvc);
        opcodes.insert(0x70, instruction_bvs);
        opcodes.insert(0x0, instruction_brk);
        opcodes.insert(0x40, instruction_rti);
        opcodes.insert(0x0a, instruction_asl_accumulator);
        opcodes.insert(0x06, instruction_asl_zeropage);
        opcodes.insert(0x16, instruction_asl_zeropage_x);
        opcodes.insert(0x0e, instruction_asl_absolute);
        opcodes.insert(0x1e, instruction_asl_absolute_x);
        opcodes.insert(0xc9, instruction_cmp_immediate);
        opcodes.insert(0xc5, instruction_cmp_zeropage);
        opcodes.insert(0xd5, instruction_cmp_zeropage_x);
        opcodes.insert(0xcd, instruction_cmp_absolute);
        opcodes.insert(0xdd, instruction_cmp_absolute_x);
        opcodes.insert(0xd9, instruction_cmp_absolute_y);
        opcodes.insert(0xc1, instruction_cmp_index_indirect);
        opcodes.insert(0xe0, instruction_cpx_immediate);
        opcodes.insert(0xe4, instruction_cpx_zeropage);
        opcodes.insert(0xec, instruction_cpx_absolute);
        opcodes.insert(0xc0, instruction_cpy_immediate);
        opcodes.insert(0xc4, instruction_cpy_zeropage);
        opcodes.insert(0xcc, instruction_cpy_absolute);
        opcodes.insert(0xc6, instruction_dec_zeropage);
        opcodes.insert(0xd6, instruction_dec_zeropage_x);
        opcodes.insert(0xce, instruction_dec_absolute);
        opcodes.insert(0xde, instruction_dec_absolute_x);
        CPU {opcodes: opcodes}
    }

    pub fn execute(&self, opcode : u8) -> Option<OpcodeCallback> {
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
            processor_status_flag.set(processor_status_flag.get() | 2); // Set the zero flag bit which is the second bit.
        }
        else if operand >= 0x80 && operand <= 0xff { // If negative set negative flag bit which is the 7th bit.
            processor_status_flag.set(processor_status_flag.get() | 0x80);
        }
}

#[test]
fn test_update_processor_status_for_zero() {
    let nes = Nes::new();
    update_processor_status_flag(0, &nes.processor_status_flag);
    assert_ne!(nes.processor_status_flag.get() & 0x02, 0x0);
}

#[test]
fn test_update_processor_status_for_negative() {
    let nes = Nes::new();
    update_processor_status_flag(0x80, &nes.processor_status_flag);
    assert_ne!(nes.processor_status_flag.get() & 0x80, 0x0);
}

//LDa Opcodes
fn instruction_lda_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.a.set(operand as u8);
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
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.a.set(memory[operand as usize]);
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
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let x = nes.x.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.a.set(memory[((operand+x) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_lda_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.x.set(1);
    }
    nes.program_counter.set(0);
    instruction_lda_zero_page_x(&nes);
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.a.set(memory[operand as usize]);
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
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.a.set(memory[(operand + x) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.x.set(255);
    }
    nes.program_counter.set(0);
    instruction_lda_absolute_x(&nes);
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_absolute_y(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let y = nes.y.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.a.set(memory[(operand + y) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_lda_absolute_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.y.set(255);
    }
    nes.program_counter.set(0);
    instruction_lda_absolute_y(&nes);
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_indirect_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as u16;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = ((operand + x) % 256) as usize;
    nes.a.set(memory[((memory[address+1] as usize) << 8) | (memory[address] as usize)] as u8);
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
        nes.x.set(254);
    }
    nes.program_counter.set(0);
    instruction_lda_indirect_x(&nes);
    assert_eq!(nes.a.get(), 69);
}

fn instruction_lda_indirect_indexed(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let y = nes.y.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = ((memory[(operand+1) as usize] as usize) << 8) | (memory[operand as usize] as usize); 
    nes.a.set(memory[(address + y) as usize] as u8);
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
        nes.y.set(254);
    }
    nes.program_counter.set(0);
    instruction_lda_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 69);
}

// LDY Opcodes
fn instruction_ldy_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.y.set(operand as u8);
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
    assert_eq!(nes.y.get(), 69);
}

fn instruction_ldy_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.y.set(memory[operand as usize]);
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
    assert_eq!(nes.y.get(), 69);
}

fn instruction_ldy_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let x = nes.x.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.y.set(memory[((operand+x) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldy_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.x.set(1);
    }
    nes.program_counter.set(0);
    instruction_ldy_zero_page_x(&nes);
    assert_eq!(nes.y.get(), 69);
}

fn instruction_ldy_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.y.set(memory[operand as usize]);
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
    assert_eq!(nes.y.get(), 69);
}

fn instruction_ldy_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.y.set(memory[(operand + x) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_ldy_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.x.set(255);
    }
    nes.program_counter.set(0);
    instruction_ldy_absolute_x(&nes);
    assert_eq!(nes.y.get(), 69);
}

// LDX Opcodes
fn instruction_ldx_immediate(nes : &Nes) {
   let memory =  nes.memory.borrow();
   let pc = nes.program_counter.get() as usize;
   let operand = memory[pc+1] as u16;
   update_processor_status_flag(operand as u16, &nes.processor_status_flag);
   nes.x.set(operand as u8);
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
    assert_eq!(nes.x.get(), 69);
}

fn instruction_ldx_zero_page(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.x.set(memory[operand as usize]);
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
    assert_eq!(nes.x.get(), 69);
}

fn instruction_ldx_zero_page_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let x = nes.x.get() as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.x.set(memory[((operand+x) % 256) as usize]);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ldx_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 69;
        nes.x.set(1);
    }
    nes.program_counter.set(0);
    instruction_ldx_zero_page_x(&nes);
    assert_eq!(nes.x.get(), 69);
}

fn instruction_ldx_absolute(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.x.set(memory[operand as usize]);
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
    assert_eq!(nes.x.get(), 69);
}

fn instruction_ldx_absolute_x(nes : &Nes) {
    let memory =  nes.memory.borrow();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as u16;
    let operand : u16 = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16); // Convert the two bytes into a 16-bit unsigned integer
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    nes.x.set(memory[(operand + x) as usize]);
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_ldx_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048+255] = 69;
        nes.x.set(255);
    }
    nes.program_counter.set(0);
    instruction_ldx_absolute_x(&nes);
    assert_eq!(nes.x.get(), 69);
}

//STa Opcodes
fn instruction_sta_zero_page(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.a.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sta_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.a.set(69); 
    instruction_sta_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_sta_zero_page_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let x = nes.x.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + x) % 256) as usize] = nes.a.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sta_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.a.set(69); 
    nes.x.set(10);
    instruction_sta_zero_page_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[20], 69);
}

fn instruction_sta_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.a.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.a.set(69); 
    instruction_sta_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}

fn instruction_sta_absolute_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let x = nes.x.get() as usize;
    memory[(operand as usize) + x] = nes.a.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.a.set(69);
    nes.x.set(10); 
    instruction_sta_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2058], 69);
}

fn instruction_sta_absolute_y(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let y = nes.y.get() as usize;
    memory[(operand as usize) + y] = nes.a.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_absolute_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.a.set(69);
    nes.y.set(10); 
    instruction_sta_absolute_y(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2058], 69);
}


fn instruction_sta_indirect_x(nes : &Nes) {
    let mut memory =  nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as u16;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = ((operand + x) % 256) as usize;
    let upper = memory[address+1] as usize;
    let lower = memory[address] as usize;
    let indirect_address = (upper << 8) | lower;
    memory[indirect_address] = nes.a.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_indirect_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 1;
        memory[256] = 8;
        nes.x.set(254);
    }
    nes.a.set(69);
    instruction_sta_indirect_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}

fn instruction_sta_indirect_indexed(nes : &Nes) {
    let mut memory =  nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let y = nes.y.get() as usize;
    let operand = memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let address = (((memory[(operand+1) as usize] as usize) << 8) | (memory[operand as usize] as usize)) + y; 
    memory[address] = nes.a.get() as u8;
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sta_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 253;
        memory[254] = 8;
        nes.y.set(254);
    }
    nes.a.set(69);
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
    memory[operand as usize] = nes.x.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_stx_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.x.set(69); 
    instruction_stx_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_stx_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.x.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_stx_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.x.set(69); 
    instruction_stx_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}


fn instruction_stx_zero_page_y(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let y = nes.y.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + y) % 256) as usize] = nes.x.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_stx_zero_page_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.x.set(69); 
    nes.y.set(10);
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
    memory[operand as usize] = nes.y.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sty_zero_page() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xff;
    }
    nes.y.set(69); 
    instruction_sty_zero_page(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 69);
}

fn instruction_sty_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = ((memory[pc+2] as u16) << 8) | (memory[pc+1] as u16);
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[operand as usize] = nes.y.get();
    nes.program_counter.set(((pc+3) as u16) as u16);
}

#[test]
fn test_instruction_sty_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.y.set(69); 
    instruction_sty_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 69);
}


fn instruction_sty_zero_page_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let operand = memory[pc+1] as u16;
    let x = nes.x.get() as u16; 
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    memory[((operand + x) % 256) as usize] = nes.y.get();
    nes.program_counter.set(((pc+2) as u16) as u16);
}

#[test]
fn test_instruction_sty_zero_page_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.y.set(69); 
    nes.x.set(10);
    instruction_sty_zero_page_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[20], 69);
}

// TAX opcode

fn instruction_tax(nes : &Nes) {
    let a = nes.a.get();
    update_processor_status_flag(a as u16, &nes.processor_status_flag);
    nes.x.set(a);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tax() {
     let nes = Nes::new();
     nes.a.set(69);
     instruction_tax(&nes);
     assert_eq!(nes.x.get(), 69);
}


// TAY opcode

fn instruction_tay(nes : &Nes) {
    let a = nes.a.get();
    update_processor_status_flag(a as u16, &nes.processor_status_flag);
    nes.y.set(a);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tay() {
     let nes = Nes::new();
     nes.a.set(69);
     instruction_tay(&nes);
     assert_eq!(nes.y.get(), 69);
}

//TSX Opcode

fn instruction_tsx(nes : &Nes) {
    let stack_ptr = nes.stack_pointer.get();
    update_processor_status_flag(stack_ptr as u16, &nes.processor_status_flag);
    nes.x.set(stack_ptr);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tsx() {
     let nes = Nes::new();
     nes.stack_pointer.set(69);
     instruction_tsx(&nes);
     assert_eq!(nes.x.get(), 69);
}


// TXa opcode

fn instruction_txa(nes : &Nes) {
    let x = nes.x.get();
    update_processor_status_flag(x as u16, &nes.processor_status_flag);
    nes.a.set(x);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_txa() {
     let nes = Nes::new();
     nes.x.set(69);
     instruction_txa(&nes);
     assert_eq!(nes.a.get(), 69);
}


// TXS opcode

fn instruction_txs(nes : &Nes) {
    let x = nes.x.get();
    update_processor_status_flag(x as u16, &nes.processor_status_flag);
    nes.stack_pointer.set(x);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_txs() {
     let nes = Nes::new();
     nes.x.set(69);
     instruction_txs(&nes);
     assert_eq!(nes.stack_pointer.get(), 69);
}


// TYa opcode

fn instruction_tya(nes : &Nes) {
    let y = nes.y.get();
    update_processor_status_flag(y as u16, &nes.processor_status_flag);
    nes.a.set(y);
    let pc = nes.program_counter.get();
    nes.program_counter.set((pc+1) as u16);
}

#[test]
fn test_instruction_tya() {
     let nes = Nes::new();
     nes.y.set(69);
     instruction_tya(&nes);
     assert_eq!(nes.a.get(), 69);
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
    let x = nes.x.get() as usize;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[(operand+x) % 256] as u16) + 1;
    memory[(operand+x) % 256] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
    }
    nes.x.set(1);
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
    let x = nes.x.get() as usize;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let value = (memory[operand+x] as u16) + 1;
    memory[operand+x] = (value % 256) as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_inc_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
    }
    nes.x.set(1);
    instruction_inc_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2049], 1);
}

// INX and INY Opcodes

fn instruction_inx(nes : &Nes) {
    let x = nes.x.get() as u16;
    update_processor_status_flag(x, &nes.processor_status_flag);
    nes.x.set(((x+1) % 256) as u8);
}

#[test]
fn test_instruction_inx() {
    let nes = Nes::new();
    instruction_inx(&nes);
    assert_eq!(nes.x.get(),1);
}

fn instruction_iny(nes : &Nes) {
    let y = nes.y.get() as u16;
    update_processor_status_flag(y, &nes.processor_status_flag);
    nes.y.set(((y+1) % 256) as u8);
}

#[test]
fn test_instruction_iny() {
    let nes = Nes::new();
    instruction_iny(&nes);
    assert_eq!(nes.y.get(),1);
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
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
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
    nes.a.set(64);
    instruction_adc_immediate(&nes);
    assert_eq!(nes.a.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_immediate(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_adc_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[memory[pc+1] as usize] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
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
    nes.a.set(64);
    instruction_adc_zeropage(&nes);
    assert_eq!(nes.a.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_zeropage(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let address = ((memory[pc+1] as usize) + x) % 256;
    let operand = memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
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
    nes.a.set(64);
    nes.x.set(1);
    instruction_adc_zeropage_x(&nes);
    assert_eq!(nes.a.get(), 128);
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
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
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
        memory[4] = 8;
        memory[8] = 128;
    }
    nes.a.set(64);
    instruction_adc_absolute(&nes);
    assert_eq!(nes.a.get(), 128);
    instruction_adc_absolute(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let x = nes.x.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + x;
    let operand = memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
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
    nes.a.set(64);
    nes.x.set(1);
    instruction_adc_absolute_x(&nes);
    assert_eq!(nes.a.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_absolute_x(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_adc_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let address = ((zero_address + x) % 256) as usize;
    let low_byte = memory[address] as usize;
    let high_byte = memory[address+1] as usize;
    let operand = memory[(high_byte << 8) | low_byte] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_adc_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 64;
        memory[64] = 64;
        memory[3] = 102;
        memory[103] = 128;
        memory[128] = 128;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_adc_index_indirect(&nes);
    assert_eq!(nes.a.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_index_indirect(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_adc_indirect_indexed(nes : &Nes) {
 let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let operand = memory[((high_byte << 8) | low_byte) + y] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_adc_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[100] = 64;
        memory[65] = 64;
        memory[3] = 103;
        memory[103] = 127;
        memory[127] = 127;
        memory[128] = 128;
    }
    nes.a.set(64);
    nes.y.set(1);
    instruction_adc_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 128);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0b1000000);
    instruction_adc_indirect_indexed(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

// Subtact with carry operands
fn instruction_sbc_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = !memory[pc+1] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8)
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_sbc_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[3] = 128;
    }
    nes.a.set(64);
    instruction_sbc_immediate(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_immediate(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_sbc_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = !memory[memory[pc+1] as usize] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_sbc_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[64] = 64;
        memory[3] = 128;
        memory[128] = 128;
    }
    nes.a.set(64);
    instruction_sbc_zeropage(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_zeropage(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_sbc_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let address = ((memory[pc+1] as usize) + x) % 256;
    let operand = !memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_sbc_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 63;
        memory[64] = 64;
        memory[3] = 127;
        memory[128] = 128;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_sbc_zeropage_x(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_zeropage_x(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_sbc_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let address =  (high_byte << 8) | (memory[pc+1] as usize);
    let operand = !memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_sbc_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 64;
        memory[4] = 8;
        memory[8] = 128;
    }
    nes.a.set(64);
    instruction_sbc_absolute(&nes);
    assert_eq!(nes.a.get(), 0xff);
    assert_eq!(nes.processor_status_flag.get() & 0b1000000, 0);
    instruction_sbc_absolute(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}


fn instruction_sbc_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let x = nes.x.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + x;
    let operand = !memory[address] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_sbc_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
        memory[255] = 64;
        memory[3] = 8;
        memory[8] = 128;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_sbc_absolute_x(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_absolute_x(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_sbc_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let address = ((zero_address + x) % 256) as usize;
    let low_byte = memory[address] as usize;
    let high_byte = memory[address+1] as usize;
    let operand = !memory[(high_byte << 8) | low_byte] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_sbc_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 64;
        memory[64] = 64;
        memory[3] = 103;
        memory[103] = 128;
        memory[129] = 128;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_sbc_index_indirect(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_index_indirect(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

fn instruction_sbc_indirect_indexed(nes : &Nes) {
 let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let operand = !memory[((high_byte << 8) | low_byte) + y] as u16;
    update_processor_status_flag(operand as u16, &nes.processor_status_flag);
    let a = nes.a.get() as u16;
    let carry = if (nes.processor_status_flag.get() & 1) == 1 { // Check if there was carry from previous add operation
        1
    }
    else {
        0
    };
    let sum =  a + operand + carry;
    if ((a ^ sum) & (operand ^ sum) & 0x80) > 0 { // Overflow detected! 💥
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b1000000);
    }
    if sum > 0xff { // Carry detected!
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
        nes.a.set((sum & 0xff) as u8);
    }
    else {
        nes.a.set(sum as u8);
    }
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_sbc_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[100] = 64;
        memory[65] = 64;
        memory[3] = 103;
        memory[103] = 127;
        memory[127] = 127;
        memory[128] = 128;
    }
    nes.a.set(64);
    nes.y.set(1);
    instruction_sbc_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 0xff);
    instruction_sbc_indirect_indexed(&nes);
    assert_eq!(nes.processor_status_flag.get() & 1, 1);
}

// Nop Opcode
fn instruction_nop(nes : &Nes) {
    nes.program_counter.set(nes.program_counter.get()+1);
}

#[test]
fn test_instruction_nop() {
    let nes = Nes::new();
    instruction_nop(&nes);
    assert_eq!(nes.program_counter.get(), 1);
}

// Stack opcodes

fn instruction_pha(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    memory[nes.stack_pointer.get() as usize + 0x100] = nes.a.get();
    nes.stack_pointer.set(nes.stack_pointer.get()+1);
    nes.program_counter.set(nes.program_counter.get()+1);
}

#[test]

fn test_instruction_pha() {
    let nes = Nes::new();
    nes.a.set(69);
    instruction_pha(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x100], 69);
}

fn instruction_php(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    memory[nes.stack_pointer.get() as usize + 0x100] = nes.processor_status_flag.get();
    nes.stack_pointer.set(nes.stack_pointer.get()+1);
    nes.program_counter.set(nes.program_counter.get()+1);
}

#[test]
fn test_instruction_php() {
    let nes = Nes::new();
    nes.processor_status_flag.set(69);
    instruction_php(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x100], 69);
}

fn instruction_pla(nes : &Nes) {
    let memory = nes.memory.borrow_mut();
    nes.a.set(memory[nes.stack_pointer.get() as usize + 0x100]);
    update_processor_status_flag(nes.a.get() as u16, &nes.processor_status_flag);
    nes.stack_pointer.set(nes.stack_pointer.get()-1);
    nes.program_counter.set(nes.program_counter.get()+1);
}

#[test]
fn test_instruction_pla() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x101] = 69;
    }
    nes.stack_pointer.set(1);
    instruction_pla(&nes);
    assert_eq!(nes.a.get(), 69);
}

fn instruction_plp(nes : &Nes) {
    let memory = nes.memory.borrow();
    nes.processor_status_flag.set(memory[nes.stack_pointer.get() as usize + 0x100]);
    nes.stack_pointer.set(nes.stack_pointer.get()-1);
    nes.program_counter.set(nes.program_counter.get()+1);
}

#[test]
fn test_instruction_plp() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x101] = 69;
    }
    nes.stack_pointer.set(1);
    instruction_plp(&nes);
    assert_eq!(nes.processor_status_flag.get(), 69);
}

//And Opcodes

fn instruction_and_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get() as u16;
    let operand = memory[pc+1] as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_and_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0;
        memory[3] = 128;
    }
    nes.a.set(64);
    instruction_and_immediate(&nes);
    assert_eq!(nes.a.get(), 0);
}

fn instruction_and_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[memory[pc+1] as usize] as u16;
    let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_and_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[64] = 0;
    }
    nes.a.set(64);
    instruction_and_zeropage(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_and_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let address = ((memory[pc+1] as usize) + x) % 256;
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_and_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 63;
        memory[64] = 0;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_and_zeropage_x(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_and_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let address =  (high_byte << 8) | (memory[pc+1] as usize);
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_and_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 0;
    }
    nes.a.set(64);
    instruction_and_absolute(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_and_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let x = nes.x.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + x;
    let operand = memory[address] as u16;
     let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_and_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
        memory[255] = 0;
        memory[3] = 8;
        memory[8] = 128;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_and_absolute_x(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_and_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let address = ((zero_address + x) % 256) as usize;
    let low_byte = memory[address] as usize;
    let high_byte = memory[address+1] as usize;
    let operand = memory[(high_byte << 8) | low_byte] as u16;
    let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_and_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 64;
        memory[64] = 0;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_and_index_indirect(&nes);
    assert_eq!(nes.a.get(), 0);
}

fn instruction_and_indirect_indexed(nes : &Nes) {
 let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let operand = memory[((high_byte << 8) | low_byte) + y] as u16;
    let a = nes.a.get() as u16;
    let result = a & operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_and_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[100] = 64;
        memory[65] = 0;
    }
    nes.a.set(64);
    nes.y.set(1);
    instruction_and_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 0);
}

//EOR Opcodes

fn instruction_eor_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get() as u16;
    let operand = memory[pc+1] as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_eor_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
    }
    nes.a.set(64);
    instruction_eor_immediate(&nes);
    assert_eq!(nes.a.get(), 0);
}

fn instruction_eor_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[memory[pc+1] as usize] as u16;
    let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_eor_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[64] = 64;
    }
    nes.a.set(64);
    instruction_eor_zeropage(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_eor_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let address = ((memory[pc+1] as usize) + x) % 256;
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_eor_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 63;
        memory[64] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_eor_zeropage_x(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_eor_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let address =  (high_byte << 8) | (memory[pc+1] as usize);
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_eor_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 64;
    }
    nes.a.set(64);
    instruction_eor_absolute(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_eor_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let x = nes.x.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + x;
    let operand = memory[address] as u16;
     let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_eor_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
        memory[255] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_eor_absolute_x(&nes);
    assert_eq!(nes.a.get(), 0);
}


fn instruction_eor_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let address = ((zero_address + x) % 256) as usize;
    let low_byte = memory[address] as usize;
    let high_byte = memory[address+1] as usize;
    let operand = memory[(high_byte << 8) | low_byte] as u16;
    let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_eor_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 64;
        memory[64] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_eor_index_indirect(&nes);
    assert_eq!(nes.a.get(), 0);
}

fn instruction_eor_indirect_indexed(nes : &Nes) {
 let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let operand = memory[((high_byte << 8) | low_byte) + y] as u16;
    let a = nes.a.get() as u16;
    let result = a ^ operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_eor_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[100] = 64;
        memory[65] = 64;
    }
    nes.a.set(64);
    nes.y.set(1);
    instruction_eor_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 0);
}

// ORa Opcodes

fn instruction_ora_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get() as u16;
    let operand = memory[pc+1] as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ora_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
    }
    nes.a.set(64);
    instruction_ora_immediate(&nes);
    assert_eq!(nes.a.get(), 64);
}

fn instruction_ora_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let operand = memory[memory[pc+1] as usize] as u16;
    let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ora_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 64;
        memory[64] = 64;
    }
    nes.a.set(64);
    instruction_ora_zeropage(&nes);
    assert_eq!(nes.a.get(), 64);
}


fn instruction_ora_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let address = ((memory[pc+1] as usize) + x) % 256;
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ora_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 63;
        memory[64] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_ora_zeropage_x(&nes);
    assert_eq!(nes.a.get(), 64);
}


fn instruction_ora_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let address =  (high_byte << 8) | (memory[pc+1] as usize);
    let operand = memory[address] as u16;
    let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_ora_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 64;
    }
    nes.a.set(64);
    instruction_ora_absolute(&nes);
    assert_eq!(nes.a.get(), 64);
}


fn instruction_ora_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let high_byte = memory[pc+2] as usize;
    let x = nes.x.get() as usize;
    let address =  ((high_byte << 8) | (memory[pc+1] as usize)) + x;
    let operand = memory[address] as u16;
     let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_ora_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 254;
        memory[255] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_ora_absolute_x(&nes);
    assert_eq!(nes.a.get(), 64);
}


fn instruction_ora_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let address = ((zero_address + x) % 256) as usize;
    let low_byte = memory[address] as usize;
    let high_byte = memory[address+1] as usize;
    let operand = memory[(high_byte << 8) | low_byte] as u16;
    let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ora_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[101] = 64;
        memory[64] = 64;
    }
    nes.a.set(64);
    nes.x.set(1);
    instruction_ora_index_indirect(&nes);
    assert_eq!(nes.a.get(), 64);
}

fn instruction_ora_indirect_indexed(nes : &Nes) {
 let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get() as usize;
    let zero_address =  memory[pc+1] as usize;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let operand = memory[((high_byte << 8) | low_byte) + y] as u16;
    let a = nes.a.get() as u16;
    let result = a | operand;
    nes.a.set(result as u8);
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_ora_indirect_indexed() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 100;
        memory[100] = 64;
        memory[65] = 64;
    }
    nes.a.set(64);
    nes.y.set(1);
    instruction_ora_indirect_indexed(&nes);
    assert_eq!(nes.a.get(), 64);
}

// Branch Opcodes

fn instruction_bcc(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 1) == 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bcc() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    instruction_bcc(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}

fn instruction_bcs(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 1) == 1 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bcs() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    instruction_bcs(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}


fn instruction_beq(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 2) == 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_beq() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    instruction_beq(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}

fn instruction_bit_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let zero_address = memory[pc+1] as usize;
    let operand = memory[zero_address];
    let a = nes.a.get();
    let result = a & operand;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.processor_status_flag.set(nes.processor_status_flag.get() | (operand & 0b11000000));
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_bit_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
        memory[10] = 193;
    }
    instruction_bit_zeropage(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b11000010, 0);
}

fn instruction_bit_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let lowbyte = memory[pc+1] as usize;
    let high_byte = memory[pc+1] as usize;
    let operand = memory[(high_byte << 8) | lowbyte];
    let a = nes.a.get();
    let result = a & operand;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.processor_status_flag.set(nes.processor_status_flag.get() | (operand & 0b11000000));
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_bit_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 193;
    }
    instruction_bit_absolute(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b11000010, 0);
}


fn instruction_bmi(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 0b10000000) != 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bmi() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    instruction_bmi(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}


fn instruction_bne(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 2) != 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bne() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.processor_status_flag.set(2);
    instruction_bne(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}


fn instruction_bpl(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 0b10000000) == 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bpl() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    instruction_bpl(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}

fn instruction_bvc(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 0b01000000) == 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bvc() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    instruction_bvc(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}


fn instruction_bvs(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let offset = (pc + (memory[pc+1] as usize)) as u16;
    let status = nes.processor_status_flag.get();
    if (status & 0b01000000) != 0 {
        nes.program_counter.set(offset);
    }
    else {
        nes.program_counter.set((pc+2) as u16);
    }
}

#[test]
fn test_instruction_bvs() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 10;
    }
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b01000000);
    instruction_bvs(&nes);
    assert_eq!(nes.program_counter.get(), 10);
}

// IRQ Opcodes

fn instruction_brk(nes : &Nes) {
    let pc = nes.program_counter.get();
    let stack_ptr = nes.stack_pointer.get() as usize;
    if (nes.processor_status_flag.get() & 0b100) != 0 {
        nes.program_counter.set(pc+1);
        return;
    }
    nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000); // Set the break flag
    let mut memory = nes.memory.borrow_mut();
    memory[stack_ptr+0x100] = (pc & 0xff) as u8; // Stack starts in 0x100 in RAM
    memory[stack_ptr+0x101] = ((pc & 0xff00) >> 8) as u8;
    memory[stack_ptr+0x102] = nes.processor_status_flag.get();
    nes.stack_pointer.set((stack_ptr+3) as u8);
    let low_byte = memory[0xfffe] as u16; // Interrupt address is read starting at 0xfffe
    let high_byte = memory[0xffff] as u16; 
    let interrupt_address = (high_byte << 8) | low_byte;
    nes.program_counter.set(interrupt_address);
}

#[test]
fn test_instruction_brk() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0xfffe] = 0x1; 
    }
    nes.program_counter.set(2048);
    nes.processor_status_flag.set(8);
    instruction_brk(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x100], 0x0);
    assert_eq!(memory[0x101], 0x8);
    assert_eq!(memory[0x102], 0x8 | 0b10000);
    assert_eq!(nes.program_counter.get(), 0x1);
    assert_ne!(nes.processor_status_flag.get() & 0b10000, 0);
    assert_eq!(nes.stack_pointer.get(), 0x3);
}

fn instruction_rti(nes : &Nes) {
    let stack_pointer = nes.stack_pointer.get() as usize + 0x100;
    let memory = nes.memory.borrow();
    nes.processor_status_flag.set(memory[stack_pointer]);
    let high_byte = memory[stack_pointer - 1] as u16;
    let low_byte = memory[stack_pointer - 2] as u16;
    let return_address = (high_byte << 8) | low_byte;
    nes.stack_pointer.set(((stack_pointer - 0x100) - 3) as u8);
    nes.program_counter.set(return_address); 
}

#[test]
fn test_instruction_rti() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x103] = 8;
        memory[0x102] = 8;
    }
    nes.stack_pointer.set(3);
    instruction_rti(&nes);
    assert_eq!(nes.program_counter.get(), 2048);
    assert_eq!(nes.stack_pointer.get(), 0);
    assert_eq!(nes.processor_status_flag.get(), 8);
}

// Bit Shifting Opcodes

fn instruction_asl_accumulator(nes : &Nes) {
    let a = nes.a.get();
    if (a & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    }
    let result = a << 1;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    nes.a.set(result);
    nes.program_counter.set(nes.program_counter.get() + 1);
}

#[test]
fn test_instruction_asl_accumulator() {
    let nes = Nes::new();
    nes.a.set(0x01);
    instruction_asl_accumulator(&nes);
    assert_eq!(nes.a.get(), 0x02);
    nes.a.set(0xff);
    instruction_asl_accumulator(&nes);
    assert_eq!(nes.a.get(), 0xfe);
    assert_eq!(nes.processor_status_flag.get(), 0x81);
}


fn instruction_asl_zeropage(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let zero_address = memory[pc+1] as usize;
    let operand = memory[zero_address];
    if (operand & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    }
    let result = operand << 1;
    if (result & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    memory[zero_address] = result;
    nes.program_counter.set((pc as u16) + 2);
}

#[test]
fn test_instruction_asl_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xfe;
        memory[0xfe] = 1;
        memory[3] = 0xff;
        memory[0xff] = 0xff;  
    }
    instruction_asl_zeropage(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xfe], 0x02);
    drop(memory);
    instruction_asl_zeropage(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0xfe);
    assert_eq!(nes.processor_status_flag.get(), 0x81);
}

fn instruction_asl_zeropage_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as usize;
    let zero_address = memory[pc+1] as usize;
    let operand = memory[(zero_address + x) % 256];
    if (operand & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    }
    let result = operand << 1;
    if (result & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    memory[(zero_address + x) % 256] = result;
    nes.program_counter.set((pc as u16) + 2);
}

#[test]
fn test_instruction_asl_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[1] = 0xfd;
        memory[0xfe] = 1;
        memory[3] = 0xfe;
        memory[0xff] = 0xff;  
    }
    nes.x.set(1);
    instruction_asl_zeropage_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xfe], 0x02);
    drop(memory);
    instruction_asl_zeropage_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0xfe);
    assert_eq!(nes.processor_status_flag.get(), 0x81);
}

fn instruction_asl_absolute(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address] as usize;
    if (operand & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    }
    let result = operand << 1;
    if (result & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    memory[absolute_address] = result as u8;
    nes.program_counter.set((pc as u16) + 3);
}

#[test]
fn test_instruction_asl_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2048] = 1;
        memory[4] = 0xff;
        memory[0xff] = 0xff;
    }
    instruction_asl_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 2);
    drop(memory);
    instruction_asl_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0xfe);
    assert_eq!(nes.processor_status_flag.get(), 0x81);
}


fn instruction_asl_absolute_x(nes : &Nes) {
    let mut memory = nes.memory.borrow_mut();
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as usize;
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    println!("x = {} absolute_address = {}", x, absolute_address);
    let operand = memory[absolute_address+x] as usize;
    if (operand & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    }
    let result = (operand + x) << 1;
    if (result & 0b10000000) != 0 {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    println!("result = {}", result);
    memory[absolute_address+x] = result as u8;
    nes.program_counter.set((pc as u16) + 3);
}

#[test]
fn test_instruction_asl_absolute_x() {
   let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[2] = 8;
        memory[2409] = 1;
        memory[4] = 0xfe;
        memory[0xff] = 0xff;
    }
    nes.x.set(1);
    instruction_asl_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2049], 2);
    drop(memory);
    instruction_asl_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0x0);
    assert_eq!(nes.processor_status_flag.get(), 0x1);
}

// Compare Opcodes

fn instruction_cmp_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let operand = memory[pc+1];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cmp_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 1;
    }
    instruction_cmp_immediate(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_immediate(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}


fn instruction_cmp_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let zero_address = memory[pc+1] as usize;
    let operand = memory[zero_address];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cmp_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 0xff;
        memory[0xff] = 1;
    }
    instruction_cmp_zeropage(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_zeropage(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cmp_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let x = nes.x.get() as usize;
    let zero_address = memory[pc+1] as usize;
    let operand = memory[(zero_address+x) % 256];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cmp_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 0xfe;
        memory[0xff] = 1;
    }
    nes.x.set(1);
    instruction_cmp_zeropage_x(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_zeropage_x(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cmp_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_cmp_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 0x8;
        memory[0x800] = 1;
    }
    instruction_cmp_absolute(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_absolute(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cmp_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let x = nes.x.get() as usize;
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address+x];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_cmp_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 0x8;
        memory[0x801] = 1;
    }
    nes.x.set(1);
    instruction_cmp_absolute_x(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_absolute_x(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cmp_absolute_y(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let y = nes.y.get() as usize;
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address+y];
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_cmp_absolute_y() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 0x8;
        memory[0x801] = 1;
    }
    nes.y.set(1);
    instruction_cmp_absolute_y(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_absolute_y(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cmp_index_indirect(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let a = nes.a.get();
    let x = nes.x.get() as usize;
    let zero_address = (memory[pc+1] as usize + x) % 256;
    let low_byte = memory[zero_address] as usize;
    let high_byte = memory[zero_address+1] as usize;
    let indirect_address = (high_byte << 8) | low_byte;
    let operand = memory[indirect_address];
    println!("indirect_address = {} operand = {}", indirect_address, operand);
    if a == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if a < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if a >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cmp_index_indirect() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x03] = 0x08;
        memory[0xa] = 0x08;
        memory[0x800] = 0x01;
    }
    nes.x.set(1);
    instruction_cmp_index_indirect(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cmp_index_indirect(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

// Compare X Register Opcodes

fn instruction_cpx_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get();
    let operand = memory[pc+1];
    if x == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if x < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if x >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cpx_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 1;
    }
    instruction_cpx_immediate(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpx_immediate(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}


fn instruction_cpx_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get();
    let zero_address = memory[pc+1] as usize;
    let operand = memory[zero_address];
    if x == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if x < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if x >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cpx_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 0xff;
        memory[0xff] = 1;
    }
    instruction_cpx_zeropage(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpx_zeropage(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cpx_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let x = nes.x.get();
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address];
    if x == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if x < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if x >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_cpx_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 0x8;
        memory[0x800] = 1;
    }
    instruction_cpx_absolute(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpx_absolute(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

// Compare Y Register Opcodes

fn instruction_cpy_immediate(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get();
    let operand = memory[pc+1];
    if y == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if y < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if y >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cpy_immediate() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 1;
    }
    instruction_cpy_immediate(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpy_immediate(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}


fn instruction_cpy_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get();
    let zero_address = memory[pc+1] as usize;
    let operand = memory[zero_address];
    if y == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if y < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if y >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_cpy_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[3] = 0xff;
        memory[0xff] = 1;
    }
    instruction_cpy_zeropage(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpy_zeropage(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

fn instruction_cpy_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let memory = nes.memory.borrow();
    let y = nes.y.get();
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let operand = memory[absolute_address];
    if y == operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 2);
    }
    else if y < operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 0b10000000);
    }
    if y >= operand {
        nes.processor_status_flag.set(nes.processor_status_flag.get() | 1);
    } 
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_cpy_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 0x8;
        memory[0x800] = 1;
    }
    instruction_cpy_absolute(&nes); 
    assert_eq!(nes.processor_status_flag.get(), 3);
    nes.processor_status_flag.set(0);
    instruction_cpy_absolute(&nes);
    assert_ne!(nes.processor_status_flag.get() & 0b10000000, 0)

}

//Decrement opcodes

fn instruction_dec_zeropage(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let mut memory = nes.memory.borrow_mut();
    let zero_address = memory[pc+1] as usize;
    let result = (memory[zero_address] as usize) + 0xff;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    memory[zero_address] = result as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_dec_zeropage() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x1] = 0xff;
        memory[0x3] = 0xfe;
        memory[0xfe] = 2;
    }
    instruction_dec_zeropage(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0xff);
    drop(memory);
    instruction_dec_zeropage(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xfe], 0x1);
    drop(memory);
}

fn instruction_dec_zeropage_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as usize;
    let mut memory = nes.memory.borrow_mut();
    let zero_address = (memory[pc+1] as usize + x) % 256;
    let result = (memory[zero_address] as usize) + 0xff;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    memory[zero_address] = result as u8;
    nes.program_counter.set((pc+2) as u16);
}

#[test]
fn test_instruction_dec_zeropage_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[0x1] = 0xfe;
        memory[0x3] = 0xfd;
        memory[0xfe] = 2;
    }
    nes.x.set(1);
    instruction_dec_zeropage_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xff], 0xff);
    drop(memory);
    instruction_dec_zeropage_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0xfe], 0x1);
    drop(memory);
}

fn instruction_dec_absolute(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let mut memory = nes.memory.borrow_mut();
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = (high_byte << 8) | low_byte;
    let result = (memory[absolute_address] as usize) + 0xff;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    memory[absolute_address] = result as u8;
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_dec_absolute() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 8;
        memory[2048] = 2;
    }
    instruction_dec_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x0], 0xff);
    drop(memory);
    instruction_dec_absolute(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2048], 0x1);
    drop(memory);
}

fn instruction_dec_absolute_x(nes : &Nes) {
    let pc = nes.program_counter.get() as usize;
    let x = nes.x.get() as usize;
    let mut memory = nes.memory.borrow_mut();
    let low_byte = memory[pc+1] as usize;
    let high_byte = memory[pc+2] as usize;
    let absolute_address = ((high_byte << 8) | low_byte) + x;
    let result = (memory[absolute_address] as usize) + 0xff;
    update_processor_status_flag(result as u16, &nes.processor_status_flag);
    memory[absolute_address] = result as u8;
    nes.program_counter.set((pc+3) as u16);
}

#[test]
fn test_instruction_dec_absolute_x() {
    let nes = Nes::new();
    {
        let mut memory = nes.memory.borrow_mut();
        memory[5] = 8;
        memory[2049] = 2;
    }
    nes.x.set(1);
    instruction_dec_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[0x1], 0xff);
    drop(memory);
    instruction_dec_absolute_x(&nes);
    let memory = nes.memory.borrow();
    assert_eq!(memory[2049], 0x1);
    drop(memory);
}

