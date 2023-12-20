use std::env;
use std::error::Error;
use pic_entropy::read_data;
use pic_entropy::colour::*;
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
        eprintln!("usage: path/to/programme <input-file> <output-file> <bits> <max-repeats> <error-threshold>");
        std::process::exit(1);
    }

    let (header, pixels, footer) = read_data(&args[1])?;
    let pixels = pixels.iter().rev().cloned().collect();
    let flattened_pixels = flatten(&pixels);
    //let quantizer_dictionary = ColourDict::new(args[5].parse().unwrap(), args[4].parse().unwrap(), args[3].parse().unwrap());
    let quantizer_dictionary = create_lbg_dictionary(&flattened_pixels, args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap());

    //println!("{:?}", quantizer_dictionary);

    let quantized_pixels = vector_quantize(&flattened_pixels, &quantizer_dictionary);

    let mse = calculate_mse(&colour_to_bytes(&flattened_pixels), &colour_to_bytes(&quantized_pixels));
    let mse_blue = calculate_mse(&extract_colour(&flattened_pixels, &Hue::BLUE), &extract_colour(&quantized_pixels, &Hue::BLUE));
    let mse_green = calculate_mse(&extract_colour(&flattened_pixels, &Hue::GREEN), &extract_colour(&quantized_pixels, &Hue::GREEN));
    let mse_red = calculate_mse(&extract_colour(&flattened_pixels, &Hue::RED), &extract_colour(&quantized_pixels, &Hue::RED));
    println!("mse = {:?}", mse);
    println!("mse (blue) = {:?}", mse_blue);
    println!("mse (green) = {:?}", mse_green);
    println!("mse (red) = {:?}", mse_red);

    let snr = calculate_snr(&colour_to_bytes(&flattened_pixels), mse);
    let snr_blue = calculate_snr(&extract_colour(&flattened_pixels, &Hue::BLUE), mse);
    let snr_green = calculate_snr(&extract_colour(&flattened_pixels, &Hue::GREEN), mse);
    let snr_red = calculate_snr(&extract_colour(&flattened_pixels, &Hue::RED), mse);
    println!("snr = {:?} ({:?} dB)", snr, 10.0 * snr.log10());
    println!("snr (blue) = {:?} ({:?} dB)", snr_blue, 10.0 * snr_blue.log10());
    println!("snr (green) = {:?} ({:?} dB)", snr_green, 10.0 * snr_green.log10());
    println!("snr (red) = {:?} ({:?} dB)", snr_red, 10.0 * snr_red.log10());

    //println!("{:?}", quantized_pixels);

    write_tga(&args[2], &header, &quantized_pixels, &footer);

    Ok(())
}
