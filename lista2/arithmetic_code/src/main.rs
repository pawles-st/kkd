use std::error::Error;
use std::fs::File;

mod encoder;
mod decoder;

fn main() -> Result<(), Box<dyn Error>> {

    // read command line arguments

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Please specify if you want to encode or decode a file".into());
    }

    if args[1] == "encode" {
        if args.len() < 4 {
            return Err("Please specify the file to encode and the output file".into());
        }

        let mut source_file = File::open(&args[2])?;
        let mut coded_file = File::create(&args[3])?;
        let (text_len, entropy, code_len, compression_rate) = encoder::encode(&mut source_file, &mut coded_file)?;
        println!("text length = {}", text_len);
        println!("entropy = {}", entropy);
        println!("avg code length = {}", code_len);
        println!("compression rate = {}", compression_rate);
    } else if args[1] == "decode" {
        if args.len() < 4 {
            return Err("Please specify the file to decode and the output file".into());
        }

        let mut decoded_file = File::create(&args[3])?;
        decoder::decode(&args[2], &mut decoded_file)?;
    } else {
        return Err("Please specify what you want to do".into());
    }

    Ok(())
}
