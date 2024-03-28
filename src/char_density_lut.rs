use std::ops::Index;

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct CharPixelDensityLut {
    char_lut: [char; 256],
}

impl CharPixelDensityLut {
    pub fn new() -> Self {
        todo!();
    }

    pub fn from_lut(lut: [char; 256]) -> Self {
        Self { char_lut: lut }
    }   
}

impl Index<u8> for CharPixelDensityLut {
    type Output = char;

    fn index(&self, index: u8) -> &Self::Output {
        &self.char_lut[index as usize]
    }  
}