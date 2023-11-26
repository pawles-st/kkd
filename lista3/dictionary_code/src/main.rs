use dictionary_code::*;
use std::fs;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: path/to/programme <file-to-compress> <code-type>");
        std::process::exit(1);
    }
    
    let message = fs::read(&args[1])?;

    let code = match args[2].as_str() {
        "gamma" => CodeType::GAMMA,
        "delta" => CodeType::DELTA,
        "omega" => CodeType::OMEGA,
        _ => {println!("Invalid code type."); std::process::exit(1);},
    };

    let mut compressed = compress_bytes(&message, CodeType::OMEGA);
    println!("{:?}", compressed);
    let decompressed = decompress_bytes(&mut compressed, CodeType::OMEGA);
    //println!("{:?}", decompressed);
    fs::write("result.txt", decompressed);

    Ok(())
}
