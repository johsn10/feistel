use derive_new::new;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, new)]
pub struct Feistel {
    pub data: FeistelInput,
    pub obfuscate: fn(&u8) -> u8,
}

impl Feistel {
    pub fn run(&self) -> Feistel {
        let feistel_output = feistel_iterator(&self);

        return feistel_output;
    }
}

fn feistel_iterator(feistel: &Feistel) -> Feistel {
    let (mut left, mut right) = feistel.data.split();

    for _ in 0..3 {
        let next_left = right;

        let right_obfuscated = (feistel.obfuscate)(&right);
        let next_right = left ^ right_obfuscated;

        (left, right) = (next_left, next_right);
    }

    let mut new_feistel = feistel.clone();
    new_feistel.data = FeistelInput::from_sides(right, left);
    new_feistel
}

#[derive(Debug, Clone, Copy, new)]
pub struct FeistelInput(u16);

impl FeistelInput {
    pub fn split(&self) -> (u8, u8) {
        let left = ((**self & 0xff_00) >> 8) as u8;
        let right = (**self & 0x00_ff) as u8;

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