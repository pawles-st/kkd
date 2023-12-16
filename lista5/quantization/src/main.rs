use std::env;
use std::error::Error;
use pic_entropy::read_data;
use quantization::colour_dict::ColourDict;
use quantization::*;

fn flatten<T: std::clone::Clone>(v: &Vec<Vec<T>>) -> Vec<T> {
    return v
        .iter()
        .flatten()
        .cloned()
        .collect();
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("usage: path/to/programme <input-file> <output-file> <r-pixels> <g-pixels> <b-pixels>");
        std::process::exit(1);
    }

    let (header, pixels, footer) = read_data(&args[1])?;
    let pixels = pixels.iter().rev().cloned().collect();
    let flattened_pixels = flatten(&pixels);
    let quantizer_dictionary = ColourDict::new(args[5].parse().unwrap(), args[4].parse().unwrap(), args[3].parse().unwrap());

    println!("{:?}", quantizer_dictionary);

    let quantized_pixels = quantize(&flattened_pixels, &quantizer_dictionary);
    
    //println!("{:?}", quantized_pixels);

    write_tga(&args[2], &header, &quantized_pixels, &footer);

    Ok(())
}
