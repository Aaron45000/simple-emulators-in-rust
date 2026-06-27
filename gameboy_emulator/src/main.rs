use std::fs;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

use crate::memory::RawMemory;
mod memory;


struct Joypad 
{
    right: bool,
    left: bool,
    up: bool,
    down: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool,
}

impl Joypad
{
    fn new() -> Self
    {
        return Joypad 
        {
            right: true,
            left: true,
            up: true,
            down: true,
            a: true,
            b: true,
            select: true,
            start: true,
        }
    }
}

struct Emulator
{
    raw_memory: RawMemory,
    joypad: Joypad,
}

impl Emulator
{
    fn new() -> Self
    {
        return Emulator
        {
            raw_memory: memory::RawMemory::new(),
            joypad: Joypad::new(),
        }
    }

    fn read_joypad(&mut self)
    {
        let joypad_register;

    
        self.raw_memory.address_bus[0xFF00] = (self.raw_memory.address_bus[0xFF00] & 0xF0) | 0x0F;

        if ((self.raw_memory.address_bus[0xFF00] >> 5) & 1) == 0
        {
            if ((self.raw_memory.address_bus[0xFF00] >> 4) & 1) == 0
            {
                // Ambos selectores activos: combinar accion + dpad (caso raro)
                joypad_register = ((self.joypad.a && self.joypad.right) as u8)
                                | ((self.joypad.b && self.joypad.left) as u8) << 1
                                | ((self.joypad.select && self.joypad.up) as u8) << 2
                                | ((self.joypad.start && self.joypad.down) as u8) << 3;

                self.raw_memory.address_bus[0xFF00] = (self.raw_memory.address_bus[0xFF00] & 0xF0) | joypad_register;
                return;
            }

            // Bit 5 = 0, Bit 4 = 1: leer botones de accion (A, B, Select, Start)
            joypad_register = (self.joypad.a as u8)
                            | (self.joypad.b as u8) << 1
                            | (self.joypad.select as u8) << 2
                            | (self.joypad.start as u8) << 3;

            self.raw_memory.address_bus[0xFF00] = (self.raw_memory.address_bus[0xFF00] & 0xF0) | joypad_register;
            return;
        }

        if !((self.raw_memory.address_bus[0xFF00] & 1<<4) == 1<<4)
        {
            // Bit 5 = 1, Bit 4 = 0: leer direcciones (Right, Left, Up, Down)
            joypad_register = (self.joypad.right as u8)
                            | (self.joypad.left as u8) << 1
                            | (self.joypad.up as u8) << 2
                            | (self.joypad.down as u8) << 3;

            self.raw_memory.address_bus[0xFF00] = (self.raw_memory.address_bus[0xFF00] & 0xF0) | joypad_register;
            return;
        }
    }
}


fn main() 
{
    // --- Leer ROM ---
    let romdata = fs::read("/home/aaron4500/Descargas/Pokemon - Edicion Roja (Spain) (SGB Enhanced).gb")
        .expect("No se pudo abrir el archivo");

    let new_liceense_code_low: u8 = romdata[0x0144];
    let new_liceense_code_high = romdata[0x0145]; 
    let old_liceense_code = romdata[0x014b];
    let cartridge_type = romdata[0x0147];
    let rom_size = romdata[0x0148]; 
    let ram_size = romdata[0x0149];
    let destination_code = romdata[0x014A];
    let mut chartitle: [char; 16] = [0 as char; 16];

    for i in 0..15
    {
        chartitle[i] = romdata[0x0134 + i] as char;
    }

    let title: String = chartitle.iter().collect();

    println!("title: {}", title);
    println!("new liceense code: {}{}", new_liceense_code_low as char, new_liceense_code_high as char);
    println!("old liceense code: {:X}", old_liceense_code);
    println!("cartridge type: {:X}", cartridge_type);
    println!("rom size: {:X}", rom_size);
    println!("ram size: {:X}", ram_size);
    println!("destination_code: {:X}", destination_code);

    
    let mut emulator = Emulator::new();

    
    emulator.raw_memory.address_bus[0xFF00] |= 0b11001111;

    // --- Crear ventana con winit 0.29 ---
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("prueba inputs")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    // --- Inicializar pixels ---
    // Resolución interna de Game Boy: 160x144
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(160, 144, surface_texture).unwrap();

    // Pintar fondo azul inicial
    for pixel in pixels.frame_mut().chunks_exact_mut(4) 
    {
        pixel[0] = 0x00; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0xFF; // B
        pixel[3] = 0xFF; // A
    }
    pixels.render().unwrap();

    
    let frame_duration = std::time::Duration::from_secs_f64(1.0 / 59.7275);
    let mut next_frame = std::time::Instant::now() + frame_duration;

    // --- Bucle principal ---
    // En winit 0.29, run() toma un closure y retorna Result. No retorna hasta que la app cierra.
    // En winit 0.29 el closure toma (event, event_loop_target): sin control_flow separado
    event_loop.run(move |event, event_loop_target|
    {
        event_loop_target.set_control_flow(ControlFlow::WaitUntil(next_frame));

        match event
        {
            // Cerrar ventana
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
            {
                event_loop_target.exit();
            }

            // Manejo de teclado
            Event::WindowEvent 
            { 
                event: WindowEvent::KeyboardInput 
                { 
                    event: KeyEvent { physical_key, state, .. }, .. 
                }, 
                .. 
            } =>
            {
                let presionado = state == ElementState::Pressed;

                match physical_key
                {
                    PhysicalKey::Code(KeyCode::KeyW)      => emulator.joypad.up     = !presionado,
                    PhysicalKey::Code(KeyCode::KeyA)      => emulator.joypad.left   = !presionado,
                    PhysicalKey::Code(KeyCode::KeyS)      => emulator.joypad.down   = !presionado,
                    PhysicalKey::Code(KeyCode::KeyD)      => emulator.joypad.right  = !presionado,
                    PhysicalKey::Code(KeyCode::KeyK)      => emulator.joypad.a      = !presionado,
                    PhysicalKey::Code(KeyCode::KeyL)      => emulator.joypad.b      = !presionado,
                    PhysicalKey::Code(KeyCode::Enter)     => emulator.joypad.start  = !presionado,
                    PhysicalKey::Code(KeyCode::Backspace) => emulator.joypad.select = !presionado,
                    _ => (),
                }
            }

            // Cuando ya no hay eventos pendientes: logica de frame
            Event::AboutToWait =>
            {
                let now = std::time::Instant::now();

                if now >= next_frame
                {
                    let ant_joypad_register = emulator.raw_memory.address_bus[0xFF00];

                    emulator.read_joypad();

                    if ant_joypad_register != emulator.raw_memory.address_bus[0xFF00]
                    {
                        println!("joypad_register: {:08b}", emulator.raw_memory.address_bus[0xFF00]);
                    }

                    // Aqui ira la logica de CPU: emulator.cpu.step(), emulator.ppu.step(), etc.

                    window.request_redraw();

                    next_frame = now + frame_duration;
                    event_loop_target.set_control_flow(ControlFlow::WaitUntil(next_frame));
                }
            }

            // Renderizar con pixels cuando la ventana lo pida
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } =>
            {
                // Aqui copiaras el framebuffer de la PPU a pixels
                // Por ahora dejamos el fondo azul sin cambios
                if pixels.render().is_err()
                {
                    event_loop_target.exit();
                }
            }

            _ => (),
        }
    }).unwrap();
}