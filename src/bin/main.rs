use feistel::{utils::xor_with_key, Feistel, FeistelData, FeistelKeys};
use clap::Parser;
use hex::decode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    hex: String,
}

fn main() {
    let args = Args::parse();

    let data = match decode(args.hex) {
        Ok(v) => v,
        Err(e) => panic!("Error, Invalid hex data: {}", e),
    };
    let keys = match FeistelKeys::from_file("keys.txt") {
        Some(v) => v,
        None => panic!("couldn't get keys at path ./keys.txt"),
    };
    let feistel_input: FeistelData = FeistelData::new(data);
    let mut feistel: Feistel = Feistel::new(feistel_input.clone(), obfuscate, keys);

    feistel.encrypt().unwrap();
    let  run1 = feistel.clone();

    feistel.decrypt().unwrap();
    let run2 = feistel.clone();

    println!("Original: {:02x?}, Run 1: {:02x?}, Run 2: {:02x?}", *feistel_input, *run1.data, *run2.data);
}

fn obfuscate(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    xor_with_key(&data, &key)
}