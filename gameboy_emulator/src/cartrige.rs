use std::fs;
use std::collections::HashMap;
use crate::memory::RawMemory;
 
pub enum MBC_Type { RomOnly = 0,MBC1 = 1, MBC2 = 2, MBC3 = 3, MBC5 = 4, MBC6 = 5, MBC7 = 6, MMM01 = 7, HuC1 = 8, HuC3 = 9 }



pub struct MBC1 {
    pub rom_data: HashMap<usize, Vec<u8>>, // Se cambia banco usando el usize del hashmap
    pub ram_data: HashMap<usize, Vec<u8>>, // convertir en hashmap
    pub current_rom: u8,
    pub current_ram: u8,
    pub banking_mode: bool,
    pub ram_enable: bool
}

impl MBC1 {

    pub fn new(ram_banks: usize, rom_banks: usize, path: &str) -> Self 
    {
                
        let mut rom_data: HashMap<usize, Vec<u8>> = HashMap::with_capacity(rom_banks-1);

        // implementar carga de datos de la rom a el HashMap (bucle for con la cantidad de bancos)
        
        let mut raw_data = fs::read(path )
        .expect("No se pudo abrir el archivo");

        
        for i in 1..(rom_banks-1)
        {

            let bank_i = raw_data.split_off(0x4000*i);

            rom_data.insert(i as usize, raw_data);

            raw_data = bank_i;

        }

        let ram_data: HashMap<usize, Vec<u8>> = HashMap::with_capacity(ram_banks);
        return MBC1 { rom_data: rom_data, ram_data: ram_data, current_rom: 0, current_ram: 0, banking_mode: false, ram_enable: false }

    }

    pub fn RAM_Enable (&mut self, value: u8)
    {

        self.ram_enable = (value & 0x0F) == 0x0A;

    }
    
    
    pub fn Low_Bank_Number(&mut self, value: u8)
    {

        let mut new_bank = value & 0x1F;

            if (new_bank & 0x0F) == 0
            {

                new_bank |= 0x01;

            } 

        self.current_rom |= 0x1F & new_bank;

    }

    pub fn High_Bank_Number(&mut self, value: u8)
    {

        self.current_rom |= (0x03 & value) << 5;

    }
    
}


fn fetch_ram_size(romdata: &Vec<u8>) -> usize 
    {

    let ram_size_code = romdata[0x149];

    match ram_size_code {
            0x00 => return 0, // No RAM
            0x02 => return 8 * 1024, // 8 KB RAM
            0x03 => return 32 * 1024, // 32 KB RAM
            0x04 => return 128 * 1024, // 128 KB RAM
            0x05 => return 64 * 1024, // 64 KB RAM
            _ => panic!("Unknown Cartridge RAM size code"),
    }
}

fn fetch_rom_banks(romdata: &[u8]) -> usize
{

    let rom_size_code = romdata[0x148];

    match rom_size_code {
            0x00 => return 2, // 32 KB ROM (2 banks)
            0x01 => return 4, // 64 KB ROM (4 banks)
            0x02 => return 8, // 128 KB ROM (8 banks)
            0x03 => return 16, // 256 KB ROM (16 banks)
            0x04 => return 32, // 512 KB ROM (32 banks)
            0x05 => return 64, // 1 MB ROM (64 banks)
            0x52 => return 72, // 1.1 MB ROM (72 banks)
            0x53 => return 80, // 1.2 MB ROM (80 banks)
            0x54 => return 96, // 1.5 MB ROM (96 banks)
            _ => panic!("Unknown Cartridge ROM size code"),
    }
}

fn fetch_mbc_type(romdata: &[u8]) -> MBC_Type 
{

        let mbc_type_code = romdata[0x147];

        match mbc_type_code {
            0x00 => return MBC_Type::RomOnly, // No MBC
            0x01 | 0x02 | 0x03 => return MBC_Type::MBC1, // MBC1
            0x0F | 0x10 | 0x11 | 0x12 | 0x13 => return MBC_Type::MBC3, // MBC3
            // Los demas no se si valen la pena
            _ => panic!("Unknown/Unimplemented Cartridge MBC type"),
    }
}