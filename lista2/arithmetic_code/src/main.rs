use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

mod encoder;
mod decoder;

const BUFFER_SIZE: usize = 256;
const BYTES_RANGE: usize = 256;
const MAX_HIGH: u128 = 0x00000001000000000000000000000000;
const MIN_LOW: u128 = 0x00000000000000000000000000000000;
const INTERVAL_BITS: u32 = 128 - MAX_HIGH.leading_zeros() - 1;

#[derive(Debug)]
#[derive(Clone)]
struct Endpoints {
    left: u128,
    right: u128,
}

fn int_to_bin(n: u128) -> VecDeque<char> {
    let mut bin_rep = VecDeque::new();
    let mut bin_value = 0;
    let mut two = 2u128.pow(INTERVAL_BITS - 1);
    for _ in 0..INTERVAL_BITS {
        if n >= bin_value + two {
            bin_value += two;
            bin_rep.push_back('1');
        } else {
            bin_rep.push_back('0');
        }
        two /= 2;
    }
    return bin_rep;
}

fn int_to_bin2(n: u8) -> Vec<char> {
    let mut bin_rep = Vec::new();
    let mut bin_value = 0;
    let mut two = 2u8.pow(7);
    for _ in 0..8 {
        if n >= bin_value + two {
            bin_value += two;
            bin_rep.push('1');
        } else {
            bin_rep.push('0');
        }
        two /= 2;
    }
    return bin_rep;
}

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

    let text_len = encoder::encode(&mut source_file, &mut coded_file)?;
    decoder::decode(&args[2], &mut decoded_file, text_len)?;


    
    Ok(())
}
