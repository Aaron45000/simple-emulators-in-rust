use crate::memory::RawMemory;

enum PpuMode { HBlank = 0, VBlank = 1, OAMSearch = 2, PixelTransfer = 3 }

pub struct Ppu {
    ticks: u32,
}

impl Ppu {
    pub fn new() -> Self {
        return Ppu { ticks: 0 }
    }

    pub fn step(&mut self, m_cycles: u16, memory: &mut RawMemory) {
        self.ticks += m_cycles as u32;

        let mut ly = memory.address_bus[0xFF44];
        let mut stat = memory.address_bus[0xFF41];
        let old_mode = stat & 0x03; 
        
        let mut current_mode = old_mode;

        
        if self.ticks >= 114 {
            self.ticks -= 114;
            ly = ly.wrapping_add(1);

            if ly > 153 {
                ly = 0; 
            }
            
            
            if ly == 144 {
                memory.address_bus[0xFF0F] |= 0x01; 
            }
            
            memory.address_bus[0xFF44] = ly; 
        }

        
        if ly >= 144 
        {
            current_mode = PpuMode::VBlank as u8; 
        } 
        else 
        {
            
            if self.ticks <= 20 
            {
                current_mode = PpuMode::OAMSearch as u8;
            } 
            else if self.ticks <= 63 
            {
                current_mode = PpuMode::PixelTransfer as u8;
            } 
            else 
            {
                current_mode = PpuMode::HBlank as u8;
            }
        }

        
        if current_mode != old_mode 
        {
            
            if current_mode == PpuMode::HBlank as u8 
            {
                // renderiza linea
            }
            
        }

        stat = (stat & 0xFC) | current_mode;
        memory.address_bus[0xFF41] = stat;
        memory.ppu_mode = current_mode;
    }
}