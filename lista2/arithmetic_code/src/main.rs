use std::error::Error;
use std::fs::File;

mod encoder;
mod decoder;

fn main() -> Result<(), Box<dyn Error>> {

    // read command line arguments

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("invalid number of arguments; please run the programme as: /path/to/programme <file-to-compress> <output_file_after_compression> <output_file_after_decompression>");
        std::process::exit(1);
    }

    /* open the file and create the necessary structures */

    let mut source_file = File::open(&args[1])?;
    let mut coded_file = File::create(&args[2])?;
    let mut decoded_file = File::create(&args[3])?;

    // encode
    let (text_len, entropy, code_len, compression_rate) = encoder::encode(&mut source_file, &mut coded_file)?;
    println!("text length = {}", text_len);
    println!("entropy = {}", entropy);
    println!("avg code length = {}", code_len);
    println!("compression rate = {}", compression_rate);

    // decode
    decoder::decode(&args[2], &mut decoded_file, text_len)?;
    
    Ok(())
}
