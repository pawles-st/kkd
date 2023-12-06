use std::cmp::Ordering;
use std::env;
use std::error::Error;
use entropy::calculate_entropy_from_bytes;
use pic_entropy::*;
use pic_entropy::colour::*;

mod colour;

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
        eprintln!("please specify a file");
        std::process::exit(1);
    }

    let predictors = ["W", "N", "NW", "N + W - NW", "N + (W - NW) / 2", "W + (N - NW) / 2", "(N + W) / 2", "new"];

    let (_, mut pixels, _) = read_data(&args[1])?;
    let flattened_pixels = flatten(&pixels);
    
    println!("\n--- total ---\n");
    println!("total entropy = {}", calculate_entropy_from_bytes(&colour_to_bytes(&flattened_pixels)));
    println!("total blue entropy = {}", calculate_entropy_from_bytes(&extract_colour(&flattened_pixels, &Hue::BLUE)));
    println!("total green entropy = {}", calculate_entropy_from_bytes(&extract_colour(&flattened_pixels, &Hue::GREEN)));
    println!("total red entropy = {}", calculate_entropy_from_bytes(&extract_colour(&flattened_pixels, &Hue::RED)));

    add_black(&mut pixels);

    println!("\n--- predictor W ---\n");
    let code_1 = code_predictor_1(&pixels);
    let flattened_code_1 = flatten(&code_1);
    let code_1_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_1));
    println!("predictor W total entropy = {}", code_1_entropy);
    let code_1_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_1, &Hue::BLUE));
    println!("predictor W blue entropy = {}", code_1_blue_entropy);
    let code_1_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_1, &Hue::GREEN));
    println!("predictor W green entropy = {}", code_1_green_entropy);
    let code_1_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_1, &Hue::RED));
    println!("predictor W red entropy = {}", code_1_red_entropy);
    
    println!("\n--- predictor N ---\n");
    let code_2 = code_predictor_2(&pixels);
    let flattened_code_2 = flatten(&code_2);
    let code_2_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_2));
    println!("predictor N total entropy = {}", code_2_entropy);
    let code_2_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_2, &Hue::BLUE));
    println!("predictor N blue entropy = {}", code_2_blue_entropy);
    let code_2_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_2, &Hue::GREEN));
    println!("predictor N green entropy = {}", code_2_green_entropy);
    let code_2_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_2, &Hue::RED));
    println!("predictor N red entropy = {}", code_2_red_entropy);
    
    println!("\n--- predictor NW ---\n");
    let code_3 = code_predictor_3(&pixels);
    let flattened_code_3 = flatten(&code_3);
    let code_3_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_3));
    println!("predictor NW total entropy = {}", code_3_entropy);
    let code_3_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_3, &Hue::BLUE));
    println!("predictor NW blue entropy = {}", code_3_blue_entropy);
    let code_3_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_3, &Hue::GREEN));
    println!("predictor NW green entropy = {}", code_3_green_entropy);
    let code_3_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_3, &Hue::RED));
    println!("predictor NM red entropy = {}", code_3_red_entropy);
    
    println!("\n--- predictor N + W - NW ---\n");
    let code_4 = code_predictor_4(&pixels);
    let flattened_code_4 = flatten(&code_4);
    let code_4_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_4));
    println!("predictor N + W - NW total entropy = {}", code_4_entropy);
    let code_4_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_4, &Hue::BLUE));
    println!("predictor N + W - NW blue entropy = {}", code_4_blue_entropy);
    let code_4_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_4, &Hue::GREEN));
    println!("predictor N + W - NW green entropy = {}", code_4_green_entropy);
    let code_4_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_4, &Hue::RED));
    println!("predictor N + W - NW red entropy = {}", code_4_red_entropy);

    println!("\n--- predictor N + (W - NW) / 2 ---\n");
    let code_5 = code_predictor_5(&pixels);
    let flattened_code_5 = flatten(&code_5);
    let code_5_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_5));
    println!("predictor N + (W - NW) / 5 total entropy = {}", code_5_entropy);
    let code_5_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_5, &Hue::BLUE));
    println!("predictor N + (W - NW) / 5 blue entropy = {}", code_5_blue_entropy);
    let code_5_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_5, &Hue::GREEN));
    println!("predictor N + (W - NW) / 5 green entropy = {}", code_5_green_entropy);
    let code_5_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_5, &Hue::RED));
    println!("predictor N + (W - NW) / 5 red entropy = {}", code_5_red_entropy);
    
    println!("\n--- predictor W + (W - NW) / 2 ---\n");
    let code_6 = code_predictor_6(&pixels);
    let flattened_code_6 = flatten(&code_6);
    let code_6_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_6));
    println!("predictor W + (W - NW) / 6 total entropy = {}", code_6_entropy);
    let code_6_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_6, &Hue::BLUE));
    println!("predictor W + (W - NW) / 6 blue entropy = {}", code_6_blue_entropy);
    let code_6_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_6, &Hue::GREEN));
    println!("predictor W + (W - NW) / 6 green entropy = {}", code_6_green_entropy);
    let code_6_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_6, &Hue::RED));
    println!("predictor W + (W - NW) / 6 red entropy = {}", code_6_red_entropy);
    
    println!("\n--- predictor (N + W) / 2 ---\n");
    let code_7 = code_predictor_7(&pixels);
    let flattened_code_7 = flatten(&code_7);
    let code_7_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_7));
    println!("predictor (N + W) / 7 total entropy = {}", code_7_entropy);
    let code_7_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_7, &Hue::BLUE));
    println!("predictor (N + W) / 7 blue entropy = {}", code_7_blue_entropy);
    let code_7_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_7, &Hue::GREEN));
    println!("predictor (N + W) / 7 green entropy = {}", code_7_green_entropy);
    let code_7_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_7, &Hue::RED));
    println!("predictor (N + W) / 7 red entropy = {}", code_7_red_entropy);

    println!("\n--- predictor new ---\n");
    let code_new = code_predictor_new(&pixels);
    let flattened_code_new = flatten(&code_new);
    let code_new_entropy = calculate_entropy_from_bytes(&colour_to_bytes(&flattened_code_new));
    println!("predictor new total entropy = {}", code_new_entropy);
    let code_new_blue_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_new, &Hue::BLUE));
    println!("predictor new blue entropy = {}", code_new_blue_entropy);
    let code_new_green_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_new, &Hue::GREEN));
    println!("predictor new green entropy = {}", code_new_green_entropy);
    let code_new_red_entropy = calculate_entropy_from_bytes(&extract_colour(&flattened_code_new, &Hue::RED));
    println!("predictor new red entropy = {}", code_new_red_entropy);
    
    println!("\n--- comparison ---\n");
    let min_total = [code_1_entropy, code_2_entropy, code_3_entropy, code_4_entropy, code_5_entropy, code_6_entropy, code_7_entropy, code_new_entropy]
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    println!("best total entropy has predictor {}", predictors[min_total]);
    let min_blue = [code_1_blue_entropy, code_2_blue_entropy, code_3_blue_entropy, code_4_blue_entropy, code_5_blue_entropy, code_6_blue_entropy, code_7_blue_entropy, code_new_blue_entropy]
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    println!("best blue entropy has predictor {}", predictors[min_blue]);
    let min_green = [code_1_green_entropy, code_2_green_entropy, code_3_green_entropy, code_4_green_entropy, code_5_green_entropy, code_6_green_entropy, code_7_green_entropy, code_new_green_entropy]
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    println!("best green entropy has predictor {}", predictors[min_green]);
    let min_red = [code_1_red_entropy, code_2_red_entropy, code_3_red_entropy, code_4_red_entropy, code_5_red_entropy, code_6_red_entropy, code_7_red_entropy, code_new_red_entropy]
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    println!("best red entropy has predictor {}", predictors[min_red]);


    Ok(())
}
