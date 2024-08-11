use derive_new::new;
use core::fmt;
use std::{error::Error, fmt::{Display, Formatter}};
use utils::*;

pub mod data;
pub mod utils;
pub mod keys;


#[derive(Debug, Clone, new)]
pub struct Feistel {
    pub data: FeistelData,
    pub round_func: fn(&Vec<u8>, &Vec<u8>) -> Vec<u8>,
    pub keys: FeistelKeys,
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
        feistel.keys.reverse();

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

    for round in 0..feistel.keys.len() {
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

#[derive(Debug, Clone, new)]
pub struct FeistelKeys(Vec<Vec<u8>>);
