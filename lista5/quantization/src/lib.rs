use std::fs;
use pic_entropy::colour::{Colour, colour_to_bytes};
use crate::colour_dict::ColourDict;

pub mod colour_dict;

type PixelArray = Vec<Vec<Colour>>;
type PixelVec = Vec<Colour>;

pub fn quantize(pixels: &PixelVec, dictionary: &ColourDict) -> PixelVec {
    return pixels
        .iter()
        .map(|&col| {
            let bval = dictionary
                .blue_values
                .iter()
                .min_by_key(|&v| col.blue.abs_diff(*v))
                .unwrap();
            let gval = dictionary
                .green_values
                .iter()
                .min_by_key(|&v| col.green.abs_diff(*v))
                .unwrap();
            let rval = dictionary
                .red_values
                .iter()
                .min_by_key(|&v| col.red.abs_diff(*v))
                .unwrap();
            return Colour{blue: *bval, green: *gval, red: *rval};
        })
        .collect();
}

pub fn write_tga(out: &str, header: &[u8], pixels: &PixelVec, footer: &[u8]) {
    let mut image_bytes = header.to_vec();
    image_bytes.extend(colour_to_bytes(&pixels));
    image_bytes.extend(footer);
    fs::write(out, image_bytes);
}
