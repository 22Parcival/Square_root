use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

#[derive(Debug)]
struct MascotHeader {
    magic_number: [u8; 4],
    version: u8,
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct MascotPixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let args=Cli::parse();

    println!("demarrage");
    println!("source : {}", args.input);
    println!("cible : {}", args.output);

    let _header = MascotHeader {
        magic_number: [b'R', b'O', b'o', b'T'],
        version: 1,
        width: 0,
        height: 0,
    };

    println!("structure data et cli ready");
}