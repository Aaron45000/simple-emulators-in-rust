use std::fs;

use crate::cartrige;

pub struct RawMemory 
{
    pub address_bus: [u8; 0x10000],
    pub div_reset: bool,
    pub ppu_mode: u8,
    pub cartrige: cartrige::MBC1 
}

impl RawMemory
{
    pub fn new(path: &str, rom_banks: usize, ram_banks: usize) -> Self
    {
        return RawMemory
        {
            address_bus: [0; 0x10000],
            div_reset: false,
            ppu_mode: 2, //Averiguar como hacer una "interfaz" para MBCn
            cartrige: cartrige::MBC1::new(ram_banks, rom_banks,path)
        }
    }
    
    pub fn read_byte(&self, address: u16) -> u8 
    {
        if  (address >= 0xFE00 && address <= 0xFE9F) && (self.ppu_mode ==  2 || self.ppu_mode == 3)
        {

            return 0xFF;

        }
        if self.ppu_mode == 3 && (address >= 0x8000 && address <= 0x9FFF)
        {

            return 0xFF;
            
        }

        return self.address_bus[address as usize];

    }

    pub fn write_byte(&mut self, address: u16, value: u8) 
    {
                
                if address >= 0x0000 && address <= 0x1FFF 
                {

                    self.cartrige.RAM_Enable(value);

                }
                if address >= 0x2000 && address <= 0x3FFF
                {
 
                    self.cartrige.Low_Bank_Number(value);
                
                }

                if address >= 0x4000 && address <= 0x5FFF
                {

                    if self.cartrige.banking_mode
                    {

                        self.cartrige.RAM_Bank_Select(value);
                    
                    }
                    else
                    {

                        self.cartrige.High_Bank_Number(value);

                    }    
                }
        if  (address >= 0xFE00 && address <= 0xFE9F) && (self.ppu_mode ==  2 || self.ppu_mode == 3)
        {

            return;

        }
        if self.ppu_mode == 3 && (address >= 0x8000 && address <= 0x9FFF)
        {

            return;
            
        }

        match address 
        {
            0xFF04 => 
            {

                self.address_bus[0xFF04] = 0;
                self.div_reset = true;
            
            },
            0xFF46 => 
            {
                
                self.address_bus[0xFF46] = value;

                let copy_address = (value as usize) << 8;
                self.address_bus.copy_within(copy_address..copy_address + 160, 0xFE00);

            },
            _ => self.address_bus[address as usize] = value,
        }
    }
    
}

