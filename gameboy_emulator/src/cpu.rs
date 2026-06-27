use crate::memory::RawMemory;

pub struct Cpu 
{

    pub program_counter: u16,
    pub stack_pointer: u16,
    // A HIGH y F LOW (al ser little endian F tiene los bits mayores (4,5,6,7))
    pub a_register: u8,
    pub flag_register: u8, // bit 4 Carry, bit 5 Half Carry (BCD), bit 6 Substraction Flag (BCD), bit 7 zero flag
    // B high y C LOW
    pub b_register: u8,
    pub c_register: u8,
    // D high y E LOW
    pub d_register: u8,
    pub e_register: u8,
    // H high y L LOW
    pub h_register: u8,
    pub l_register: u8,
    pub raw_memory: RawMemory,

}

impl Cpu
{

    pub fn new() -> Self 
    {

        return Cpu
        {
            program_counter: 0,
            stack_pointer: 0,
            a_register: 0,
            flag_register: 0,
            b_register: 0,
            c_register: 0,
            d_register: 0,
            e_register: 0,
            h_register: 0,
            l_register: 0,
            raw_memory: RawMemory::new(),
        }
    }
}

