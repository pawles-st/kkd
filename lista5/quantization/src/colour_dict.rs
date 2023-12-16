const BYTE_SIZE: usize = 256;

#[derive(Debug)]
pub struct ColourDict {
    pub blue_values: Vec<u8>,
    pub green_values: Vec<u8>,
    pub red_values: Vec<u8>,
}

impl ColourDict {
    pub fn new(blue_bits: u8, green_bits: u8, red_bits: u8) -> Self {
        let no_blues = 2_i32.pow(blue_bits as u32) as usize;
        let bval = (0..no_blues)
            .map(|i| (BYTE_SIZE / no_blues * i as usize + BYTE_SIZE / no_blues / 2) as u8)
            .collect::<Vec<u8>>();
        let no_greens = 2_i32.pow(green_bits as u32) as usize;
        let gval = (0..no_greens)
            .map(|i| (BYTE_SIZE / no_greens * i as usize + BYTE_SIZE / no_greens / 2) as u8)
            .collect::<Vec<u8>>();
        let no_reds = 2_i32.pow(red_bits as u32) as usize;
        let rval = (0..no_reds)
            .map(|i| (BYTE_SIZE / no_reds * i as usize + BYTE_SIZE / no_reds / 2) as u8)
            .collect::<Vec<u8>>();
        Self{blue_values: bval, green_values: gval, red_values: rval}
    }
}
