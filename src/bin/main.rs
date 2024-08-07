use feistel::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    left: u8,

    #[arg(short, long, default_value_t = 1)]
    right: u8,
    
}

fn main() {
    let args = Args::parse();
    let feistel_input: FeistelInput = FeistelInput::from_sides(0x12, 0x34);
    let (left, right) = feistel_input.split();
    println!("Left: {:04x}, Right: {:04x}, Full: {:04x}", left, right, *feistel_input);
}