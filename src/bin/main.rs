use feistel::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    hex: String,
}

fn main() {
    let args = Args::parse();
    let data = match u16::from_str_radix(&args.hex, 16) {
        Ok(v) => v,
        Err(_) => panic!("Invalid hex data"),
    };

    let feistel_input: FeistelInput = FeistelInput::new(data);
    let feistel: Feistel = Feistel::new(feistel_input, obfuscate);
    let run1 = feistel.run();
    let run2 = run1.run();

    println!("Original: {:04x}, Run 1: {:04x}, Run 2: {:04x}", *feistel_input, *run1.data, *run2.data);
}

fn obfuscate(data: &u8) -> u8 {
    data ^ 0x12
}