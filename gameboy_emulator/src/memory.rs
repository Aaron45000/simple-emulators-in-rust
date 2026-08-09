


pub struct RawMemory 
{
    pub address_bus: [u8; 0x10000],
    pub div_reset: bool,
    pub ppu_mode: u8
}

impl RawMemory
{
    pub fn new() -> Self
    {
        return RawMemory
        {
            address_bus: [0; 0x10000],
            div_reset: false,
            ppu_mode: 2
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

