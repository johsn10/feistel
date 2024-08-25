use crate::FeistelKeys;
use core::fmt;
use std::{fs::File, io::Read, ops::{Deref, DerefMut}, path::PathBuf};
use hex::decode;

impl FeistelKeys {
    pub fn from_file(path: &PathBuf) -> Option<Self> {
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

    pub fn generate_keys(width: usize, height: usize) -> Self {
        let mut keys = Self::new(vec![]);
        for _ in 0..height {
            let mut line: Vec<u8> = vec![];
            for _ in 0..width {
                let rand_num = rand::random::<u8>();
                line.push(rand_num);
            }
            keys.push(line);
        }
        keys
    }
}

impl fmt::Display for FeistelKeys {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.iter() {
            for byte in line {
                fmt.write_str(&format!("{:02x}", byte))?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
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