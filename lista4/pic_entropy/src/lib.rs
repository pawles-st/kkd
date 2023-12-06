use std::cmp::{min, max};
use std::fs;
use std::error::Error;
use crate::colour::*;

pub mod colour;

type PixelArray = Vec<Vec<Colour>>;

pub fn add_black(v: &mut PixelArray) {
    let height = v.len();
    let width = v[0].len();

    // add black to left

    for i in 0..height {
        v[i].insert(0, Colour{blue: 0, green: 0, red: 0});
    }

    // add black to above

    v.insert(0, 
        (0..=width)
        .map(|_| Colour{blue: 0, green: 0, red: 0})
        .collect()
    );
}

// an assumption is made that all pixel arrays thereon have extra black on the left and up

fn prepare_code_array(pixels: &PixelArray) -> PixelArray {
    
    // prepare the code array
    
    let height = pixels.len() - 1;
    let width = pixels[0].len() - 1;

    let mut code: PixelArray = Vec::new();
    code.resize(height, Vec::new());
    for i in 0..height {
        code[i].resize(width, Colour{blue: 0, green: 0, red: 0});
    }

    return code;
}

pub fn code_predictor_1(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i + 1][j];
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_2(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i][j + 1];
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_3(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i][j];
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}


pub fn code_predictor_4(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i][j + 1] + pixels[i + 1][j] - pixels[i][j];
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_5(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i][j + 1] + (pixels[i + 1][j] - pixels[i][j]) / 2;
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_6(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = pixels[i + 1][j] + (pixels[i][j + 1] - pixels[i][j]) / 2;
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_7(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];
            let prev_pixel = (pixels[i][j + 1] + pixels[i + 1][j]) / 2;
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn code_predictor_new(pixels: &PixelArray) -> PixelArray {

    let mut code = prepare_code_array(pixels);
    let height = code.len();
    let width = code[0].len();

    // code the sequence

    for i in 0..height {
        for j in 0..width {
            let curr_pixel = pixels[i + 1][j + 1];

            // blue
            
            let max_blue = max(pixels[i + 1][j].blue, pixels[i][j + 1].blue);
            let min_blue = min(pixels[i + 1][j].blue, pixels[i][j + 1].blue);
            let prev_blue = if pixels[i][j].blue >= max_blue {
                min_blue
            } else if pixels[i][j].blue <= min_blue {
                max_blue
            } else {
                pixels[i + 1][j].blue.wrapping_add(pixels[i][j + 1].blue.wrapping_sub(pixels[i][j].blue))
            };
            
            // green
            
            let max_green = max(pixels[i + 1][j].green, pixels[i][j + 1].green);
            let min_green = min(pixels[i + 1][j].green, pixels[i][j + 1].green);
            let prev_green = if pixels[i][j].green >= max_green {
                min_green
            } else if pixels[i][j].green <= min_green {
                max_green
            } else {
                pixels[i + 1][j].green.wrapping_add(pixels[i][j + 1].green.wrapping_sub(pixels[i][j].green))
            };
            
            // red
            
            let max_red = max(pixels[i + 1][j].red, pixels[i][j + 1].red);
            let min_red = min(pixels[i + 1][j].red, pixels[i][j + 1].red);
            let prev_red = if pixels[i][j].red >= max_red {
                min_red
            } else if pixels[i][j].red <= min_red {
                max_red
            } else {
                pixels[i + 1][j].red.wrapping_add(pixels[i][j + 1].red.wrapping_sub(pixels[i][j].red))
            };

            let prev_pixel = Colour{blue: prev_blue, green: prev_green, red: prev_red};
            
            code[i][j] = curr_pixel - prev_pixel;
        }
    }

    return code;
}

pub fn read_data(file: &str) -> Result<PixelArray, Box<dyn Error>> {

    /* assume no colour map for now */

    let bytes = fs::read(file)?;
    let header = &bytes[..18];
    let image = &bytes[18..];
    //println!("header = {:?}", header);
    //println!("after-header len = {:?}", image.len());

    let width_spec = [bytes[12], bytes[13]];
    let height_spec = [bytes[14], bytes[15]];
    let width = u16::from_le_bytes(width_spec) as usize;
    let height = u16::from_le_bytes(height_spec) as usize;
    //let total_size = width * height;
    //println!("w = {:?}", width);
    //println!("h = {:?}", height);

    // prepare the pixels array

    let mut pixels: PixelArray = Vec::new();
    pixels.resize(height, Vec::new());
    for i in 0..height {
        pixels[i].resize(width, Colour{blue: 0, green: 0, red: 0});
    }

    // read the pixels into the array

    let mut colour_nr = 0;
    for i in 0..height {
        for j in 0..width {
            pixels[height - i - 1][j] = Colour{blue: image[colour_nr], green: image[colour_nr + 1], red: image[colour_nr + 2]};
        colour_nr += 3;
        }
    }

    return Ok(pixels);
}
