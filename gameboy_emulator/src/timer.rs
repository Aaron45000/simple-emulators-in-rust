use crate::memory::RawMemory;

pub struct Timer {
    pub internal_counter: u16,
    pub last_and_result: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            internal_counter: 0,
            last_and_result: false,
        }
    }

    pub fn step(&mut self, m_cycles: u8, memory: &mut RawMemory) {
        // Asumiendo que la CPU te está devolviendo M-cycles, los multiplicamos por 4
        // para operar en la resolución correcta de T-cycles.
        let t_cycles = (m_cycles as u16) * 4;

        for _ in 0..t_cycles {
            // Revisar si la CPU acaba de escribir en DIV (0xFF04)
            if memory.div_reset {
                self.internal_counter = 0;
                memory.div_reset = false;
            }

            // Incrementar el contador maestro
            self.internal_counter = self.internal_counter.wrapping_add(1);

            // El registro DIV siempre refleja los 8 bits superiores
            memory.address_bus[0xFF04] = (self.internal_counter >> 8) as u8;

            // Leer estado del TAC
            let tac = memory.read_byte(0xFF07);
            let timer_enable = (tac & 0b0000_0100) != 0;

            // Elegir qué bit monitorear según los bits 1-0 del TAC
            let bit_position = match tac & 0b11 {
                0b00 => 9, // 4096 Hz
                0b01 => 3, // 262144 Hz
                0b10 => 5, // 65536 Hz
                0b11 => 7, // 16384 Hz
                _ => unreachable!(),
            };

            // Evaluar el estado del bit seleccionado
            let bit_selected = (self.internal_counter & (1 << bit_position)) != 0;

            // La compuerta AND de nuestro detector de flancos
            let current_and_result = bit_selected && timer_enable;

            // Detectar flanco de bajada (de 1 a 0)
            if self.last_and_result && !current_and_result {
                let tima = memory.read_byte(0xFF05);

                if tima == 0xFF {
                    // Si hay Overflow:
                    // 1. Cargar TMA en TIMA
                    memory.address_bus[0xFF05] = memory.read_byte(0xFF06);
                    
                    // 2. Pedir interrupción del Timer (Bit 2 de IF en 0xFF0F)
                    let current_if = memory.read_byte(0xFF0F);
                    memory.address_bus[0xFF0F] = current_if | 0b0000_0100;
                } else {
                    // Solo incrementar
                    memory.address_bus[0xFF05] = tima + 1;
                }
            }

            // Guardar estado para el próximo T-cycle
            self.last_and_result = current_and_result;
        }
    }
}
