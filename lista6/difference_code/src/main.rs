use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use pic_entropy::read_data;
use quantization::write_tga;
use colour_diff::*;
type DiffVec = Vec<ColourDiff>;

//use pic_entropy::colour::*;
use difference_code::*;

fn flatten<T: std::clone::Clone>(v: &Vec<Vec<T>>) -> Vec<T> {
    return v
        .iter()
        .flatten()
        .cloned()
        .collect();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("specify whether you want to encode or decode a picture");
        std::process::exit(1);
    }

    let action = &args[1];

    if action == "encode" {
        if args.len() < 5 {
            eprintln!("usage: path/to/programme encode <input-file> <output-file> <quantization-bits>");
            std::process::exit(1);
        }

        let (header, pixels, footer) = read_data(&args[2])?;
        let pixels = pixels.iter().rev().cloned().collect();
        let flattened_pixels = flatten_diagonal(&pixels);

        //println!("pixels = {:?}", flattened_pixels);

        // separate pixels into two bands

        let low_band = filter_low(&flattened_pixels);
        //println!("low band = {:?}", low_band);
        let high_band = filter_high(&flattened_pixels);
        //println!("high band = {:?}", high_band);

        // quantize the bands

        let no_bits = args[4].parse::<u8>().unwrap();

        let low_band_diffs = code_difference(&low_band);
        //println!("low band diffs = {:?}", low_band_diffs);
        let low_band_dictionary = create_quantization_dictionary(&low_band_diffs, no_bits);
        println!("low dict = {:?}", low_band_dictionary);
        let quantized_low_band_diffs = diff_quantize(&low_band, &low_band_dictionary);
        //println!("quantized low band = {:?}", quantized_low_band_diffs);

        let high_band_dictionary = create_quantization_dictionary(&high_band, no_bits);
        //println!("high dict = {:?}", high_band_dictionary);
        let quantized_high_band = scalar_quantize(&high_band, &high_band_dictionary);
        //println!("quantized high band = {:?}", quantized_high_band);

        let header_json = serde_json::to_string(&header).unwrap();
        let low_json = serde_json::to_string(&quantized_low_band_diffs).unwrap();
        let high_json = serde_json::to_string(&quantized_high_band).unwrap();
        let footer_json = serde_json::to_string(&footer).unwrap();

        let mut out = File::create(&args[3])?;
        writeln!(&mut out, "{}", &header_json)?;
        writeln!(&mut out, "{}", &low_json)?;
        writeln!(&mut out, "{}", &high_json)?;
        writeln!(&mut out, "{}", &footer_json)?;

    } else if action == "decode" {

        if args.len() < 4 {
            eprintln!("usage: path/to/programme decode <input-file> <output-file>");
            std::process::exit(1);
        }

        let data = fs::read_to_string(&args[2])?;
        let lines: Vec<&str> = data.split("\n").collect();
        let header = serde_json::from_str::<Vec<u8>>(lines[0])?;
        let low_band_diffs = serde_json::from_str::<DiffVec>(lines[1])?;
        let high_band = serde_json::from_str::<DiffVec>(lines[2])?;
        let footer = serde_json::from_str::<Vec<u8>>(lines[3])?;

        let width_spec = [header[12], header[13]];
        let height_spec = [header[14], header[15]];
        let width = u16::from_le_bytes(width_spec) as usize;
        let height = u16::from_le_bytes(height_spec) as usize;

        // reconstruct the original values

        let low_band = reconstruct_from_diff(&low_band_diffs);
        //println!("reconstructed low band = {:?}", low_band);

        let original_flattened_float = reconstruct_from_bands(&low_band, &high_band);
        //println!("original = {:?}", original_flattened_float);
        let original_flattened = round_to_colour(&original_flattened_float);
        //println!("original = {:?}", original_flattened);
        let original = restore_diagonal(&original_flattened, height, width);
        let original = flatten(&original);
        write_tga(&args[3], &header, &original, &footer);

        //let original_pixels = restore_diagonal(&flattened_pixels, pixels.len(), pixels[0].len());
        
    } else {
        eprintln!("invalid action; try 'encode' or 'decode'");
        std::process::exit(1);
    }

    Ok(())
}
