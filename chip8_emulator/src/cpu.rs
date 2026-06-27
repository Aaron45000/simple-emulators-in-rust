use std::fs::File;
use std::io::Read;
use rand;

pub struct Chip8 
{
    pub registers: [u8; 16],      
    pub memory: [u8; 4096],       
    pub index_register: u16,      
    pub pc: u16,                  
    pub stack: [u16; 16],         
    pub sp: usize,                  
    pub delay_timer: u8,          
    pub sound_timer: u8,          
    pub display: [bool; 64 * 32], 
    pub keypad: [bool; 16],       
}

impl Chip8 
{
    pub fn new() -> Self 
    {
        Self {
            registers: [0; 16],       
            memory: [0; 4096],        
            index_register: 0,
            pc: 0x200,                
            stack: [0; 16],          
            sp: 0,                    
            delay_timer: 0,
            sound_timer: 0,
            display: [false; 64 * 32], 
            keypad: [false; 16],       
        }
    }

    pub fn load_fontset(&mut self) 
    {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..80 
        {
            self.memory[i] = fontset[i];
        }
    }

    pub fn load_rom(&mut self, path: &str) 
    {
        

        let mut file = File::open(path).expect("No se pudo abrir la ROM");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Error al leer la ROM");
        for (i, &byte) in buffer.iter().enumerate() 
        {
            self.memory[0x200 + i] = byte;
        }
    }


    pub fn execute_cycle(&mut self) -> bool
    {
        let pc_actual = self.pc as usize;
        if pc_actual >= 4094 
        {
            return false;
        }
        
        let high_byte = self.memory[pc_actual] as u16;
        let low_byte = self.memory[pc_actual + 1] as u16;
        let instruction = (high_byte << 8) | low_byte;

        let n1 = ((instruction & 0xF000) >> 12) as u8;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n4 = (instruction & 0x000F) as u8;
        let kk = (instruction & 0x00FF) as u8; 
        let nnn = instruction & 0x0FFF;

        let mut advanced_pc = false;
        let mut wait_vblank = false;

        match (n1, x, y, n4) 
        {
            (0, 0, 0xE, 0) => 
            {
                self.display.fill(false);
            }
            (0, 0, 0xE, 0xE) => 
            {
                if self.sp > 0 
                {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp];
                }
            }
            (1, _, _, _) => 
            {
                self.pc = nnn;
                advanced_pc = true;
            }
            (2, _, _, _) => 
            {
                if self.sp < 16 
                {
                    self.stack[self.sp] = self.pc;
                    self.sp += 1;
                    self.pc = nnn;
                    advanced_pc = true;
                }
            }
            (3, _, _, _) => 
            {
                if kk == self.registers[x] 
                {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => 
            {
                if !(kk == self.registers[x]) 
                {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => 
            {
                if self.registers[x] == self.registers[y]
                {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => 
            {
                self.registers[x] = kk;
            }
            (7, _, _, _) => 
            {
                self.registers[x] = self.registers[x].wrapping_add(kk);
            }
            (8, _, _, 0) => 
            {
                self.registers[x] = self.registers[y]
            }
            (8, _, _, 1) => 
            {
                self.registers[x] |= self.registers[y];
                self.registers[15] = 0;
            }
            (8, _, _, 2) => 
            {
                self.registers[x] &= self.registers[y];
                self.registers[15] = 0;
            }
            (8, _, _, 3) => 
            {
                self.registers[x] ^= self.registers[y];
                self.registers[15] = 0;
            }
            (8, _, _, 4) => 
            {
                let val_x = self.registers[x];
                let val_y = self.registers[y];
                    
                let sum = (val_x as u16) + (val_y as u16);
                let vf = if sum > 255 { 1 } else { 0 };

                self.registers[x] = val_x.wrapping_add(val_y);
                self.registers[15] = vf;
                
            }
            (8, _, _, 5) => 
            {
                let mut vf = 0 as u8;
                if self.registers[x] >= self.registers[y]
                {
                    vf = 1;
                }
                
                self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]); 
                self.registers[15] = vf;
            }
            (8, _, _, 7) => 
            {
                let mut vf = 0 as u8;
                if self.registers[y] >= self.registers[x]
                {
                    vf = 1;
                }
                
                self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]); 
                self.registers[15] = vf;
            }
            (8, _, _, 6) => 
                {
                    // COMPORTAMIENTO CLÁSICO (SHIFTING OFF): Lee de Y, desplaza y guarda en X
                    let val_y = self.registers[y]; 
                    let vf = val_y & 1;

                    self.registers[x] = val_y >> 1;
                    self.registers[15] = vf;
                }
            (8, _, _, 0xE) => 
                {
                    // COMPORTAMIENTO CLÁSICO (SHIFTING OFF): Lee de Y, desplaza e izquierda y guarda en X
                    let val_y = self.registers[y];
                    let vf = (val_y >> 7) & 1;

                    self.registers[x] = val_y.wrapping_shl(1);
                    self.registers[15] = vf;
                }
            (9, _, _, 0) => 
            {
                if !(self.registers[x] == self.registers[y])
                {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => 
            {
                self.index_register = nnn;
            }
            (0xB, _, _, _) => 
            {
                self.pc = nnn + (self.registers[0] as u16); 
                advanced_pc = true;
            }
            (0xC, _, _, _) => 
            {
                let random_byte: u8 = rand::random::<u8>();
                self.registers[x] = random_byte & kk;
            }
            (0xD, _, _, _) => 
            {
                wait_vblank = true;
                let start_x = self.registers[x] % 64;
                let start_y = self.registers[y] % 32;
                self.registers[15] = 0;

                for i in 0..(n4 as usize) 
                {
                    let pixel_y = start_y as usize + i;
                    if pixel_y >= 32 { break; }

                    let actual_byte = self.memory[(self.index_register as usize) + i];

                    for bit_index in 0..8 
                    {
                        let pixel_x = start_x as usize + bit_index;
                        if pixel_x >= 64 { break; }

                        let sprite_pixel = (actual_byte >> (7 - bit_index)) & 1;
                        
                        if sprite_pixel == 1 
                        {
                            let screenindex = (pixel_y * 64) + pixel_x;

                            if self.display[screenindex] 
                            {
                                self.registers[15] = 1;
                            }

                            self.display[screenindex] ^= true;
                        }
                    }
                }
            }
            (0xE, _, 9, 0xE) => 
            {
                let key = self.registers[x] as usize;
                if self.keypad[key] 
                {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => 
            {
                let key = self.registers[x] as usize;
                if !(self.keypad[key]) 
                {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => 
            {
                self.registers[x] = self.delay_timer;                
            }
            (0xF, _, 0, 0xA) => 
            {
                let mut key_pressed = false;
                for i in 0..16 
                {
                    if self.keypad[i] 
                    {
                        self.registers[x] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed 
                {
                    self.pc -= 2;
                }     
            }
            (0xF, _, 1, 5) => 
            {
                self.delay_timer = self.registers[x];
            }
            (0xF, _, 1, 8) => 
            {
                self.sound_timer = self.registers[x];       
            }
            (0xF, _, 1, 0xE) => 
            {
                self.index_register += self.registers[x] as u16;
            }
            (0xF, _, 2, 9) => 
            {
                let digit = self.registers[x] as u16;
                self.index_register = digit * 5;      
            }
            (0xF, _, 3, 3) => 
            {
                let value = self.registers[x];
                let hundreds = value / 100;
                let tens = (value / 10) % 10;
                let ones = value % 10;
                
                let i_idx = self.index_register as usize;
                self.memory[i_idx] = hundreds;
                self.memory[i_idx + 1] = tens;
                self.memory[i_idx + 2] = ones;
            }
            (0xF, _, 5, 5) => 
                {
                    for i in 0..=x 
                    {
                        self.memory[self.index_register as usize + i] = self.registers[i];
                    }     
                    // COMPORTAMIENTO CLÁSICO (MEMORY ON): Incrementa el registro I
                    self.index_register = self.index_register + (x as u16) + 1;
                }
            (0xF, _, 6, 5) => 
                {
                    for i in 0..=x 
                    {
                        self.registers[i] = self.memory[self.index_register as usize + i];
                    }     
                    // COMPORTAMIENTO CLÁSICO (MEMORY ON): Incrementa el registro I
                    self.index_register = self.index_register + (x as u16) + 1;
                }
            _ => println!("Instrucción no reconocida: {:#X}", instruction),
        }

        if !advanced_pc 
        {
            self.pc += 2;
        }

        wait_vblank
    }
}