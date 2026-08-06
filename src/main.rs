use clap::Parser;
use image::imageops::FilterType;
use std::fs::File;
use std::io::Write;

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


    let target_width: u32 = 200;
    let target_height: u32 = (img.height()*target_width/img.width());
    println!("creat grille ({}x{})...", target_width, target_height);

    let resized_img = img.resize_exact(target_width, target_height, FilterType::Nearest);
    println!("img redimentioner");

    let _header = MascotHeader {
        magic_number: [b'R', b'O', b'O', b'T'],
        version: 1,
        width: target_width as u16,
        height: target_height as u16,
    };
    
    let rgb_img = resized_img.to_rgb8();
    println!("create {}", args.output);

    let mut file = File::create(&args.output).expect("erreur");
    file.write_all(&_header.magic_number).unwrap();
    file.write_all(&[_header.version]).unwrap();
    file.write_all(&_header.width.to_be_bytes()).unwrap();
    file.write_all(&_header.height.to_be_bytes()).unwrap();

    for pixel in rgb_img.pixels() {
        let color_bytes = [pixel[0], pixel[1], pixel[2]];
        file.write_all(&color_bytes).unwrap();
    }

    println!("fichier {} generate", args.output);

}