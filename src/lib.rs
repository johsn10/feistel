use derive_new::new;
use core::fmt;
use std::{error::Error, fmt::{Display, Formatter}, ops::{Deref, DerefMut}};
use utils::*;

pub mod utils;

#[derive(Debug, Clone, new)]
pub struct Feistel {
    pub data: FeistelData,
    pub round_func: fn(&Vec<u8>, &Vec<u8>) -> Vec<u8>,
    pub keys: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum FeistelError {
    InvalidPadding,
}
impl Display for FeistelError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidPadding => write!(f, "invalid padding"),
        }
    }
}
impl Error for FeistelError {}

impl Feistel {
    pub fn reverse_keys(&mut self) {
        self.keys.reverse();
    }

    pub fn encrypt(&mut self) -> Result<(), FeistelError> {
        self.data.padd();

        match feistel_iterator(&self) {
            Ok(feistel) => {
                *self = feistel;
                Ok(())
            },
            Err(err) => Result::Err(err),
        }
    }

    pub fn decrypt(&mut self) -> Result<(), FeistelError> {
        let mut feistel = self.clone();
        feistel.reverse_keys();

        *self = match feistel_iterator(&feistel) {
            Ok(feistel) => {
                feistel
            },
            Err(err) => return Result::Err(err),
        };

        match self.data.unpadd() {
            Ok(()) => {},
            Err(err) => return Err(err),
        };


        Ok(())
    }
}

fn feistel_iterator(feistel: &Feistel) -> Result<Feistel, FeistelError> {
    let (mut left, mut right) = feistel.data.split();

    for round in 0..3 {
        let next_left = right.clone();

        let right_obfuscated = (feistel.round_func)(&right, &feistel.keys[round]);
        let next_right = xor_sides(&left, &right_obfuscated);

        (left, right) = (next_left, next_right);
    }

    let mut new_feistel = feistel.clone();
    new_feistel.data = FeistelData::from_sides(right, left);
    Ok(new_feistel)
}

#[derive(Debug, Clone, new)]
pub struct FeistelData(Vec<u8>);

impl FeistelData {
    pub fn split(&self) -> (Vec<u8>, Vec<u8>) {
        let data_center = (**self).len() / 2;
        let data_len = (**self).len();

        let left = &(**self)[0..data_center];
        let right = &(**self)[data_center..data_len];

        (left.to_vec(), right.to_vec())
    }

    pub fn from_sides(left: Vec<u8>, right: Vec<u8>) -> Self {
        let mut right = right;
        let mut combined = left;
        combined.append(&mut right);
        
        Self::new(combined)
    }

    fn bytes_even(&self) -> bool {
        if self.len() % 2 == 0 || self.len() <= 1 {
            true
        } else {
            false
        }
    }

    fn padd(&mut self) {
        self.insert(0, 0x00);

        if !self.bytes_even() {
            self[0] = 0x01;
            self.push(0x00)
        }
    }

    fn unpadd(&mut self) -> Result<(), FeistelError> {
        if self[0] == 0x00 {
            self.remove(0);
        } 
        else if self[0] == 0x01 {
            self.remove(0);
            
            let data_len = self.len();
            self.remove(data_len-1);
        }
        else {
            return Err(FeistelError::InvalidPadding);
        }
        Ok(())
    }
}

impl Deref for FeistelData {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}
impl DerefMut for FeistelData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}