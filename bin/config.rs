use std::{fs::File, io::{Read, Write}, path::PathBuf};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, new)]
pub struct Config {
    pub keys_path: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        let path = PathBuf::from("feistel.toml");
        let mut file = File::open(path.clone()).unwrap_or_else(move |_| {
            File::create_new(path).expect("couldn't create feistel.toml")
        });

        let mut file_str = String::new();
        file.read_to_string(&mut file_str).expect("couldn't convert feistel.toml to string");
        
        let config: Self = toml::from_str(&file_str).expect("couldn't deserialize toml file");
        config
    }
    pub fn save(&self) {
        let mut file = File::create("feistel.toml").expect("couldn't create or get file");
        file.write(toml::to_string(self)
            .expect("couldn't serialize toml")
                .as_bytes()
        ).expect("couldn't write to file");
    }
}