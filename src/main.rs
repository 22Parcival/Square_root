use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(shorT, long)]
    output: String,
}

#[derive(Debug)]
struct MascotHeader {
    magic_number: [u8; 4],
    version: u8,
    widtg: u16,
    height: u16,
}

#[derive(Debug)]
struct MascotPixel {
    r: u8,
    g: u8,
    b: u8,
}