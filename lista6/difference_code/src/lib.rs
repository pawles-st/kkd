use pic_entropy::colour::*;
use colour_diff::*;

pub mod colour_diff;

type PixelArray = Vec<Vec<Colour>>;
type PixelVec = Vec<Colour>;
type DiffVec = Vec<ColourDiff>;

pub fn flatten_diagonal(pixels: &PixelArray) -> PixelVec {
    let height = pixels.len();
    let width = pixels[0].len();

    let mut flattened_pixels = Vec::new();

    for k in 0..(height + width - 1) {
        if k % 2 == 0 {
            for i in 0..height {
                let j = k as i64 - i as i64;
                if j >= 0 && (j as usize) < width {
                    flattened_pixels.push(pixels[i][j as usize]);
                }
            }
        } else {
            for i in (0..height).rev() {
                let j = k as i64 - i as i64;
                if j >= 0 && (j as usize) < width {
                    flattened_pixels.push(pixels[i][j as usize]);
                }
            }
        }
    }

    return flattened_pixels;
}

pub fn restore_diagonal(flattened_pixels: &PixelVec, height: usize, width: usize) -> PixelArray {
    let mut pixels = PixelArray::new();
    pixels.resize(height, Vec::new());
    for i in 0..height {
        pixels[i].resize(width, Colour{blue: 0, green: 0, red: 0});
    }

    let mut idx = 0;
    for k in 0..(height + width - 1) {
        if k % 2 == 0 {
            for i in 0..height {
                let j = k as i64 - i as i64;
                if j >= 0 && (j as usize) < width {
                    pixels[i][j as usize] = flattened_pixels[idx];
                    idx += 1;
                }
            }
        } else {
            for i in (0..height).rev() {
                let j = k as i64 - i as i64;
                if j >= 0 && (j as usize) < width {
                    pixels[i][j as usize] = flattened_pixels[idx];
                    idx += 1;
                }
            }
        }
    }

    return pixels;
}

pub fn filter_low(pixels: &PixelVec) -> DiffVec {
    let mut pixels_avg = vec![ColourDiff::from_colour(pixels[0])];

    for i in 1..pixels.len() {
        let curr_pixel = ColourDiff::from_colour(pixels[i]);
        let prev_pixel = ColourDiff::from_colour(pixels[i - 1]);
        pixels_avg.push((curr_pixel + prev_pixel) / 2.0);
    }

    return pixels_avg;
}

pub fn filter_high(pixels: &PixelVec) -> DiffVec {
    let mut pixels_diff = vec![ColourDiff::zero()];

    for i in 1..pixels.len() {
        let curr_pixel = ColourDiff::from_colour(pixels[i]);
        let prev_pixel = ColourDiff::from_colour(pixels[i - 1]);
        pixels_diff.push((curr_pixel - prev_pixel) / 2.0);
    }

    return pixels_diff;
}

pub fn code_difference(filtered_pixels: &DiffVec) -> DiffVec {
    let mut differences = vec![filtered_pixels[0]];

    for i in 1..filtered_pixels.len() {
        let curr_pixel = filtered_pixels[i];
        let prev_pixel = filtered_pixels[i - 1];
        differences.push(curr_pixel - prev_pixel);
    }

    return differences;
}

pub fn reconstruct_from_diff(diff_pixels: &DiffVec) -> DiffVec {
    let mut original = vec![diff_pixels[0]];

    for i in 1..diff_pixels.len() {
        let curr_diff = diff_pixels[i];
        let prev_pixel = original[i - 1];
        original.push(prev_pixel + curr_diff);
    }

    return original;
}

pub fn reconstruct_from_bands(low_band: &DiffVec, high_band: &DiffVec) -> DiffVec {
    let mut original = vec![low_band[0] + high_band[0]];

    for i in 1..low_band.len() {
        let curr_low = low_band[i];
        let curr_high = high_band[i];
        original.push(curr_low + curr_high);
    }

    return original;
}

pub fn round_to_colour(colour_diffs: &DiffVec) -> PixelVec {
    return colour_diffs
        .iter()
        .map(|diff| Colour{
            blue: diff.blue.ceil().clamp(0.0, 255.0) as u8,
            green: diff.green.ceil().clamp(0.0, 255.0) as u8,
            red: diff.red.ceil().clamp(0.0, 255.0) as u8,
        })
        .collect();
}

pub fn create_quantization_dictionary(values: &DiffVec, bits: u8) -> DiffVec {
    let mut sorted = values.clone();
    let mut dictionary = DiffVec::new();
    
    let no_entries = 2usize.pow(bits as u32);
    let no_values = values.len();
    
    dictionary.resize(no_entries, ColourDiff{blue: 0.0, green: 0.0, red: 0.0});
    
    //dictionary[0] = ColourDiff{blue: -1.0, green: -1.0, red: -1.0};
    //dictionary[1] = ColourDiff{blue: -0.5, green: -0.5, red: -0.5};
    //dictionary[2] = ColourDiff{blue: 0.5, green: 0.5, red: 0.5};
    //dictionary[3] = ColourDiff{blue: 1.0, green: 1.0, red: 1.0};
    //dictionary.push(ColourDiff{blue: 0.0, green: 0.0, red: 0.0});

    // find the blue colour dictionary

    sorted.sort_unstable_by(|a, b| a.blue.partial_cmp(&b.blue).unwrap());
    for k in 0..no_entries {
        dictionary[k].blue = sorted[f64::floor((k as f64 + 0.5) / no_entries as f64 * no_values as f64) as usize].blue;
    }

    if no_entries > 1 {
        let mut k = 1;
        while k < no_entries && dictionary[k].blue <= 0.0 {
            let mut change = false;
            while k > 0 && dictionary[k - 1].blue == dictionary[k].blue {
                change = true;
                dictionary.swap(k - 1, k);
                k -= 1;
            }
            if change {
                dictionary[k].blue -= 1.0;
            } else {
                k += 1;
            }
        }
        let mut k = dictionary.len() - 2;
        while k > 0 && dictionary[k].blue > 0.0 {
            let mut change = false;
            while k + 1 < no_entries && dictionary[k].blue == dictionary[k + 1].blue {
                change = true;
                dictionary.swap(k, k + 1);
                k += 1;
            }
            if change {
                dictionary[k].blue += 1.0;
            } else {
                k -= 1;
            }
        }
    }
    
    // find the green colour dictionary

    sorted.sort_unstable_by(|a, b| a.green.partial_cmp(&b.green).unwrap());
    for k in 0..no_entries {
        dictionary[k].green = sorted[f64::floor((k as f64 + 0.5) / no_entries as f64 * no_values as f64) as usize].green;
    }
    
    if no_entries > 1 {
        let mut k = 1;
        while k < no_entries - 1 && dictionary[k].green <= 0.0 {
            let mut change = false;
            while k > 0 && dictionary[k - 1].green == dictionary[k].green {
                change = true;
                dictionary.swap(k - 1, k);
                k -= 1;
            }
            if change {
                dictionary[k].green -= 1.0;
            } else {
                k += 1;
            }
        }
        let mut k = dictionary.len() - 2;
        while k > 0 && dictionary[k].green > 0.0 {
            let mut change = false;
            while k + 1 < no_entries && dictionary[k].green == dictionary[k + 1].green {
                change = true;
                dictionary.swap(k, k + 1);
                k += 1;
            }
            if change {
                dictionary[k].green += 1.0;
            } else {
                k -= 1;
            }
        }
    }
    
    // find the red colour dictionary

    sorted.sort_unstable_by(|a, b| a.red.partial_cmp(&b.red).unwrap());
    for k in 0..no_entries {
        dictionary[k].red = sorted[f64::floor((k as f64 + 0.5) / no_entries as f64 * no_values as f64) as usize].red;
    }

    if no_entries > 1 {
        let mut k = 1;
        while k < no_entries - 1 && dictionary[k].red < 0.0 {
            let mut change = false;
            while k > 0 && dictionary[k - 1].red == dictionary[k].red {
                change = true;
                dictionary.swap(k - 1, k);
                k -= 1;
            }
            if change {
                dictionary[k].red -= 1.0;
            } else {
                k += 1;
            }
        }
        let mut k = dictionary.len() - 2;
        while k > 0 && dictionary[k].red >= 0.0 {
            let mut change = false;
            while k + 1 < no_entries && dictionary[k].red == dictionary[k + 1].red {
                change = true;
                dictionary.swap(k, k + 1);
                k += 1;
            }
            if change {
                dictionary[k].red += 1.0;
            } else {
                k -= 1;
            }
        }
    }

    return dictionary;
}

pub fn diff_quantize(filtered_pixels: &DiffVec, dictionary: &DiffVec) -> DiffVec {
    let mut quantized_diffs = vec![quantize_value(&filtered_pixels[0], dictionary)];
    let mut reconstructed = quantize_value(&filtered_pixels[0], dictionary);

    for i in 1..filtered_pixels.len() {
        let diff = filtered_pixels[i] - reconstructed;
        let quantized_diff = quantize_value(&diff, dictionary);
        reconstructed = reconstructed + quantized_diff;
        quantized_diffs.push(quantized_diff);
    }

    return quantized_diffs;
}

pub fn scalar_quantize(filtered_pixels: &DiffVec, dictionary: &DiffVec) -> DiffVec {
    let mut quantized_pixels = vec![quantize_value(&filtered_pixels[0], dictionary)];

    for i in 1..filtered_pixels.len() {
        quantized_pixels.push(quantize_value(&filtered_pixels[i], dictionary));
    }

    return quantized_pixels;
}

fn quantize_value(pixel: &ColourDiff, dictionary: &DiffVec) -> ColourDiff {
    let blue_val = dictionary
        .iter()
        .min_by(|x, y| (pixel.blue - x.blue).abs().partial_cmp(&(pixel.blue - y.blue).abs()).unwrap())
        .unwrap()
        .blue;
    let green_val = dictionary
        .iter()
        .min_by(|x, y| (pixel.green - x.green).abs().partial_cmp(&(pixel.green - y.green).abs()).unwrap())
        .unwrap()
        .green;
    let red_val = dictionary
        .iter()
        .min_by(|x, y| (pixel.red - x.red).abs().partial_cmp(&(pixel.red - y.red).abs()).unwrap())
        .unwrap()
        .red;
    return ColourDiff{blue: blue_val, green: green_val, red: red_val};
}
