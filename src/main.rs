use clap::Parser;
use image::imageops::FilterType;

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
    let args = Cli::parse();

    println!("start");
    println!("source : {}", args.input);
    println!("cible   : {}", args.output);
   
    let img = image::open(&args.input).expect("erreur");
    println!("dimensions image: {}x{}", img.width(), img.height());


    let target_width = 100;
    let target_height = 100;
    println!("creat grille ({}x{})...", target_width, target_height);

    let resized_img = img.resize_exact(target_width, target_height, FilterType::Nearest);
    println!("img redimentioner");

    let _header = MascotHeader {
        magic_number: [b'R', b'O', b'O', b'T'],
        version: 1,
        width: target_width as u16,
        height: target_height as u16,
    };
    
    println!("AAA");
}