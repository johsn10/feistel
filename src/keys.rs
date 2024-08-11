use crate::FeistelKeys;
use std::{fs::File, io::Read, ops::{Deref, DerefMut}};
use hex::decode;

impl FeistelKeys {
    pub fn from_file(path: &str) -> Option<Self> {
        let mut contents = String::new();
        let mut keys: Self = Self::new(vec![]);

        let file = File::open(path);
        let mut file = match file {
            Ok(v) => v,
            Err(_) => return None,
        };
        
        match file.read_to_string(&mut contents) {
            Ok(v) => v,
            Err(_) => return None,
        };
        
        for key_str in contents.lines() {
            let key = match decode(key_str) {
                Ok(v) => v,
                Err(_) => return None,
            };
            keys.push(key);
        }

        Some(keys)
    }
}

impl Deref for FeistelKeys {
    type Target = Vec<Vec<u8>>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FeistelKeys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}