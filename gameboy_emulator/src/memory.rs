pub struct RawMemory 
{

    pub address_bus: [u8; 0x10000],
    pub program_counter: u16,

}

impl RawMemory
{

    pub fn new() -> Self
    {

        return RawMemory
        {

            address_bus: [0; 0x10000],
            program_counter: 0 as u16,
        }
    }
    pub fn read(&mut self, address: usize) -> u8
    {

        return self.address_bus[address];
    }
    pub fn write(&mut self, address: usize, value: u8) -> bool
    {

        self.address_bus[address] = value;
        return true
    }
}


pub struct Memory<'a> 
{

    pub rom00: &'a mut [u8; 0x4000],             
    pub rom01: &'a mut [u8; 0x4000],             
    pub vram: &'a mut [u8; 0x2000],              
    pub ext_ram: &'a mut [u8; 0x2000],            
    pub wram0: &'a mut [u8; 0x1000],             
    pub wram1: &'a mut [u8; 0x1000],             
    pub oam: &'a mut [u8; 0xA0],                 
    pub io_registers: &'a mut [u8; 0x80],         
    pub hram: &'a mut [u8; 0x7F],                
    pub interrupt_register: &'a mut u8,           

}

impl<'a> Memory<'a> 
{

    pub fn new(address_bus: &'a mut [u8; 0x10000]) -> Self 
    {

        let init_address = address_bus.as_mut_ptr();
        unsafe 
        {
            Memory 
            {
                rom00: &mut *(init_address.add(0x0000) as *mut [u8; 0x4000]),             
                rom01: &mut *(init_address.add(0x4000) as *mut [u8; 0x4000]),             
                vram: &mut *(init_address.add(0x8000) as *mut [u8; 0x2000]),              
                ext_ram: &mut *(init_address.add(0xA000) as *mut [u8; 0x2000]),            
                wram0: &mut *(init_address.add(0xC000) as *mut [u8; 0x1000]),      
                wram1: &mut *(init_address.add(0xD000) as *mut [u8; 0x1000]),             
                oam: &mut *(init_address.add(0xFE00) as *mut [u8; 0xA0]),                 
                io_registers: &mut *(init_address.add(0xFF00) as *mut [u8; 0x80]),         
                hram: &mut *(init_address.add(0xFF80) as *mut [u8; 0x7F]),                
                interrupt_register: &mut *(init_address.add(0xFFFF)),   
            }
        }
    }
}

