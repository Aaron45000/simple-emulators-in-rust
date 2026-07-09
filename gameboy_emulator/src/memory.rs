pub struct RawMemory 
{
    pub address_bus: [u8; 0x10000],
    pub div_reset: bool,
}

impl RawMemory
{
    pub fn new() -> Self
    {
        return RawMemory
        {
            address_bus: [0; 0x10000],
            div_reset: false,
        }
    }
    
    pub fn read_byte(&self, address: u16) -> u8 {
        self.address_bus[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => {
                // DIV: Resetea todo el contador
                self.address_bus[0xFF04] = 0;
                self.div_reset = true;
            },
            0xFF46 => {
                // DMA
                self.address_bus[0xFF46] = value;
            },
            _ => self.address_bus[address as usize] = value,
        }
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

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom00[address as usize],
            0x4000..=0x7FFF => self.rom01[(address - 0x4000) as usize],
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.ext_ram[(address - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram0[(address - 0xC000) as usize],
            0xD000..=0xDFFF => self.wram1[(address - 0xD000) as usize],
            0xE000..=0xFDFF => self.wram0[(address - 0xE000) as usize], // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, // Memoria no usable, usualmente devuelve 0xFF
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => *self.interrupt_register,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                // ROM: La escritura aquí generalmente se usa para comunicarse con el MBC (Memory Bank Controller)
                // Por ahora lo ignoramos.
            },
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.ext_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.wram0[(address - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wram1[(address - 0xD000) as usize] = value,
            0xE000..=0xFDFF => self.wram0[(address - 0xE000) as usize] = value, // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {}, // Memoria no usable
            0xFF04 => {
                // CASO ESPECIAL TIMER: Escritura en DIV (0xFF04) lo reinicia a 0
                self.io_registers[0x04] = 0;
            },
            0xFF46 => {
                // CASO ESPECIAL DMA: Iniciar transferencia OAM (Aquí se configuraría más adelante)
                self.io_registers[0x46] = value;
            },
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => *self.interrupt_register = value,
        }
    }
}

