use derive_new::new;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, new)]
pub struct FeistelInput(u16);

impl FeistelInput {
    pub fn split(&self) -> (u8, u8) {
        let left = (**self & 0x11_00 >> 8) as u8;
        let right = (**self & 0x00_11) as u8;
        (left, right)
    }

    pub fn from_sides(left: u8, right: u8) -> Self {
        let combined: u16 = ((left as u16) << 8) | (right as u16);
        Self::new(combined)
    }
}

impl Deref for FeistelInput {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}