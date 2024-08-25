use std::{fs::{self, File}, io::Write, path::PathBuf};

use feistel::{utils::xor_with_key, Feistel, FeistelData, FeistelKeys};
use clap::Parser;
use config::Config;
use cli::*;

mod config;
mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command.expect("no command") {
        Commands::Keys { subcommands } => {
            match subcommands {
                KeysSubcommands::Generate { name, width, height } => {
                    create_keys(&name, width, height)
                },
            }
        },
        Commands::Config { keys_path } => {
            Config::new(keys_path).save();
        },
        Commands::Encrypt { file, key: key_name } => {
            encrypt(&file, &key_name);
        },
        Commands::Decrypt { file, key: key_name } => {
            decrypt(file, &key_name);
        },
    }
}

fn create_keys(name: &str, width: usize, height: usize) {
    let config = Config::load();
    let mut path = config.keys_path;
    path.push(name);
    path.set_extension("fstlk");

    let keys = FeistelKeys::generate_keys(width, height);

    let mut file = File::create(path).expect("couldn't create file");
    file.write(keys.to_string().as_bytes()).expect("couldn't write to file");
}

fn encrypt(file_path: &PathBuf, key_name: &str) {
    let keys = keys_from_name(key_name);

    let data = FeistelData::new(
        fs::read(file_path).expect("couldn't read provided file")
    );
    
    let mut feistel: Feistel = Feistel::new(data, obfuscate, keys.clone());
    feistel.encrypt().unwrap();

    let mut enc_file_path = file_path.clone();
    enc_file_path = PathBuf::from(format!("{}.feistel", enc_file_path.to_str().unwrap()));

    let mut enc_file = File::create(enc_file_path).expect("couldn't create file");
    enc_file.write(&feistel.data).expect("couldn't write to file");
}

fn decrypt(file_path: PathBuf, key_name: &str) {
    let keys = keys_from_name(key_name);

    let data = FeistelData::new(
        fs::read(&file_path).expect("couldn't read provided file")
    );

    let mut feistel = Feistel::new(data, obfuscate, keys.clone());
    feistel.decrypt().expect("couldn't decrypt");

    let dec_file_path = file_path.with_extension(String::new());
    let mut file = File::create(dec_file_path).expect("couldn't create file");
    file.write(&feistel.data).expect("couldn't write to file");
}

fn keys_from_name(name: &str) -> FeistelKeys {
    let config = Config::load();
    let mut path = config.keys_path;
    let mut filename = PathBuf::from(name);
    filename.set_extension("fstlk");
    path.push(filename);

    FeistelKeys::from_file(&path).expect("couldn't get keys, maybe wrong name")
}

fn obfuscate(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    xor_with_key(&data, &key)
}