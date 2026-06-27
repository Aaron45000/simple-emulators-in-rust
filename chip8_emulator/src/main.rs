use minifb::{Key, Window, WindowOptions}; 
mod cpu; 

fn main() 
{
    let mut cpu = cpu::Chip8::new();
    cpu.load_fontset();

    cpu.load_rom("Tetris [Fran Dachille, 1991].ch8");

    let options = WindowOptions {

        scale: minifb::Scale::X16,
        ..WindowOptions::default()
    };

    let mut window = Window::new("Chip8 in Rust", 64, 32, options)
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });
    window.set_target_fps(60);

    let mut color_buffer = [0u32; 64 * 32];

    while window.is_open() && !window.is_key_down(Key::Escape)
    {

        // Fila 1: 1 2 3 C
        cpu.keypad[0x1] = window.is_key_down(Key::Key1);
        cpu.keypad[0x2] = window.is_key_down(Key::Key2);
        cpu.keypad[0x3] = window.is_key_down(Key::Key3);
        cpu.keypad[0xC] = window.is_key_down(Key::Key4);
        
        // Fila 2: 4 5 6 D
        cpu.keypad[0x4] = window.is_key_down(Key::Q);
        cpu.keypad[0x5] = window.is_key_down(Key::W);
        cpu.keypad[0x6] = window.is_key_down(Key::E);
        cpu.keypad[0xD] = window.is_key_down(Key::R);
        
        // Fila 3: 7 8 9 E
        cpu.keypad[0x7] = window.is_key_down(Key::A);
        cpu.keypad[0x8] = window.is_key_down(Key::S);
        cpu.keypad[0x9] = window.is_key_down(Key::D);
        cpu.keypad[0xE] = window.is_key_down(Key::F);
        
        // Fila 4: A 0 B F
        cpu.keypad[0xA] = window.is_key_down(Key::Z);
        cpu.keypad[0x0] = window.is_key_down(Key::X);
        cpu.keypad[0xB] = window.is_key_down(Key::C);
        cpu.keypad[0xF] = window.is_key_down(Key::V);

        for _ in 0..500 
        {

            let wait_vblank = cpu.execute_cycle();
            if wait_vblank { break; }
            
        }

        if cpu.delay_timer > 0 
        { 
            cpu.delay_timer -= 1; 
        }
        
        if cpu.sound_timer > 0 
        { 
            cpu.sound_timer -= 1; 
        }
        
        for i in 0..(64 * 32) 
        {
            // if cpu.display = true then the pixel is white if not then pixel is black
            color_buffer[i] = if cpu.display[i] { 0xFFFFFF } else { 0x000000 };
        }
        
        window.update_with_buffer(&color_buffer, 64, 32).unwrap();
        
    }
}

