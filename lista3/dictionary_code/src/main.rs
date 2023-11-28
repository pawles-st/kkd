use dictionary_code::*;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        println!("Usage: path/to/programme action <input-file> <output-file> <code-type>");
        std::process::exit(1);
    }
    
    let text = fs::read(&args[2])?;

    let code = match args[4].as_str() {
        "gamma" => CodeType::GAMMA,
        "delta" => CodeType::DELTA,
        "omega" => CodeType::OMEGA,
        "fib" => CodeType::FIB,
        _ => {println!("Invalid code type"); std::process::exit(1);},
    };

    match args[1].as_str() {
        "compress" => {
            let compressed = compress_bytes(&text, &code);
            fs::write(&args[3], compressed)?;
        },
        "decompress" => {
            let decompressed = decompress_bytes(&text, &code);
            fs::write(&args[3], decompressed)?;
        },
        _ => {println!("Invalid action"); std::process::exit(1);},
    }

    Ok(())
}
