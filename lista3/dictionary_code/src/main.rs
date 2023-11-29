use dictionary_code::*;
use entropy::calculate_entropy_from_bytes;
use std::fs;
use std::fs::File;
use std::error::Error;

fn print_statistics(text: &[u8], args: &[String]) -> Result<(), std::io::Error> {
    let input_file = File::open(&args[2])?;
    let input_file_len = input_file.metadata().unwrap().len();
    println!("input file length = {}", input_file_len);
    println!("entropy = {}", calculate_entropy_from_bytes(text));

    let output_file = File::open(&args[3])?;
    let output_file_len = output_file.metadata().unwrap().len();
    println!("output file length = {}", output_file_len);
    let code = fs::read(&args[3])?;
    println!("code entropy = {}", calculate_entropy_from_bytes(&code));

    let compression_ratio = input_file_len as f64 / output_file_len as f64;
    println!("compression ratio = {}", compression_ratio);

    Ok(())
}

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
            print_statistics(&text, &args)?;
        },
        "decompress" => {
            let decompressed = decompress_bytes(&text, &code);
            fs::write(&args[3], decompressed)?;
        },
        _ => {println!("Invalid action"); std::process::exit(1);},
    }

    Ok(())
}
