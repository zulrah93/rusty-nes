use std::fs::File;
use std::cell::RefCell;
use std::io::Read;
use std::error::Error;
use std::fmt;

pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1
}

impl fmt::Display for Mirroring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
           Mirroring::Horizontal => "Horizontal Mirroing",
           Mirroring::Vertical => "Vertical Mirroing"
        })
       
    }
}

pub struct Rom {
    pub number_of_rom_banks : u8,
    pub number_of_vrom_banks : u8,
    pub mirroring : Mirroring,
    pub has_battery_packed_ram : bool,
    pub has_trainer : bool,
    pub has_four_screen_vram_layout : bool,
    pub rom_mapper_type : u8,
    pub is_vs_system_cartidge : bool,
    pub number_of_8k_ram_banks : u8,
    pub is_pal : bool,
    pub trainer : RefCell<Option<[u8;512]>>,
    pub rom_banks : Vec<[u8;16384]>, // PRG ROM banks
    pub vrom_banks : Vec<[u8;8192]> // CHR ROM banks


}

impl Rom {
    pub fn load(file_name : String) -> Result<Rom,String> {
        println!("Reading {}", file_name);
        if let Ok(file_handle) = &mut File::open(file_name) {
            let mut buffer : Vec<u8> = Vec::new();
            if let Err(e) = file_handle.read_to_end(&mut buffer) {
                Err(e.description().to_string())
            }
            else {
                let magic_string_valid = buffer[0] == 0x4e && buffer[1] == 0x45 && buffer[2] == 0x53 && buffer[3] == 0x1a;
                if magic_string_valid {
                        let number_of_rom_banks = buffer[4];
                        let number_of_vrom_banks = buffer[5];
                        let six_flag = buffer[6];
                        let mirroring = if six_flag & 0x1 == 0x1 {
                            Mirroring::Vertical
                        }
                        else {
                            Mirroring::Horizontal
                        };
                        let has_battery_packed_ram = six_flag & 0x2 == 0x2;
                        let has_trainer = six_flag & 0x4 == 0x4;
                        let has_four_screen_vram_layout = six_flag & 0x8 == 0x8;
                        let low_nibble = (six_flag & 0xf0) >> 4;
                        let seven_flag = buffer[7];
                        let is_vs_system_cartidge = seven_flag & 0x1 == 0x1;
                        let high_nibble = (seven_flag & 0xf0) >> 4;
                        let mapper_number = (high_nibble << 4) | low_nibble;
                        let number_of_8k_ram_banks = if buffer[8] == 0 {
                            1
                        }
                        else {
                            buffer[8]
                        };
                        let is_pal = buffer[9] == 1;
                        let trainer = if has_trainer {
                            let mut trainer_data : [u8;512] = [0;512];
                            trainer_data.copy_from_slice(&buffer[16..(16+512)]);
                            RefCell::from(Some(trainer_data))
                        }
                        else {
                            RefCell::new(None)
                        };
                        let mut rom_banks : Vec<[u8;16384]> = Vec::new();
                        for i in 0..(number_of_rom_banks as usize) {
                            let mut rom_bank : [u8;16384] = [0;16384];
                            if has_trainer {
                                let start = (i*16384)+16+512;
                                let end = (i*16384)+16+16384+512;
                                rom_bank.copy_from_slice(&buffer[start..end]);
                            }
                            else {
                                let start = ((i*16384)+16) as usize;
                                let end = ((i*16384)+16+16384) as usize;
                                rom_bank.copy_from_slice(&buffer[start..end]);
                            }
                            rom_banks.push(rom_bank);
                        }
                        let mut vrom_banks : Vec<[u8;8192]> = Vec::new();
                        let offset = (number_of_rom_banks as usize) * 16384;
                        for i in 0..(number_of_vrom_banks as usize) {
                            let mut vrom_bank : [u8;8192] = [0;8192];
                            if has_trainer {
                                let start = (i*8192)+16+512+offset;
                                let end = (i*8192)+16+8192+512+offset;
                                vrom_bank.copy_from_slice(&buffer[start..end]);
                            }
                            else {
                                let start = ((i*8192)+16+offset) as usize;
                                let end = ((i*8192)+16+8192+offset) as usize;
                                vrom_bank.copy_from_slice(&buffer[start..end]);
                            }
                            vrom_banks.push(vrom_bank);
                        }

                        println!("Successfully Loaded!\nROM Debug Info:\nNumber of PRG ROM Pages (16K each): {}\nNumber of CHR ROM Pages (8K each): {}\nMapper Type: {}\nPAL: {}\nHas Trainer: {}\nMirroring: {}", 
                        number_of_rom_banks, 
                        number_of_vrom_banks, 
                        mapper_number, is_pal,
                        has_trainer,
                        mirroring);

                        Ok(Rom {
                            number_of_rom_banks: number_of_rom_banks, number_of_vrom_banks: number_of_vrom_banks, 
                            mirroring: mirroring, has_battery_packed_ram: has_battery_packed_ram, has_trainer: has_trainer, 
                            has_four_screen_vram_layout: has_four_screen_vram_layout, rom_mapper_type: mapper_number,
                            is_vs_system_cartidge: is_vs_system_cartidge, number_of_8k_ram_banks: number_of_8k_ram_banks, is_pal: is_pal,
                            trainer: trainer, rom_banks: rom_banks, vrom_banks: vrom_banks 
                        })

                        
                }
                else {
                    Err(String::from("NES file magic 4 byte string is missing!"))
                }
            }
         }
        else {
            Err(String::from("Failed to open NES Rom!"))
        }
    }
}