use std::ops::{Deref, DerefMut};

use crate::{FeistelData, FeistelError};

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

    pub(crate) fn bytes_even(&self) -> bool {
        if self.len() % 2 == 0 || self.len() <= 1 {
            true
        } else {
            false
        }
    }

    pub(crate) fn padd(&mut self) {
        self.insert(0, 0x00);

        if !self.bytes_even() {
            self[0] = 0x01;
            self.push(0x00)
        }
    }

    pub(crate) fn unpadd(&mut self) -> Result<(), FeistelError> {
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
