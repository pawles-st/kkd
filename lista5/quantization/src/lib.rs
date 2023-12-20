use std::fs;
use std::cmp::Ordering;
use rand::distributions::{Distribution, Uniform};
use pic_entropy::colour::*;
use crate::colour_dict::ColourDict;

pub mod colour_dict;

type PixelVec = Vec<Colour>;

#[derive(Clone)]
#[derive(Debug)]
pub struct Centroid {
    blue: f64,
    green: f64,
    red: f64,
}

type CentroidVec = Vec<Centroid>;

fn lbg_move_centroids(centroids: &mut CentroidVec, clusters: &Vec<PixelVec>) {
    for i in 0..centroids.len() {
        if clusters[i].len() > 0 {
            centroids[i].blue = extract_colour(&clusters[i], &Hue::BLUE).iter().map(|&v| v as usize).sum::<usize>() as f64 / clusters[i].len() as f64;
            centroids[i].green = extract_colour(&clusters[i], &Hue::GREEN).iter().map(|&v| v as usize).sum::<usize>() as f64 / clusters[i].len() as f64;
            centroids[i].red = extract_colour(&clusters[i], &Hue::RED).iter().map(|&v| v as usize).sum::<usize>() as f64 / clusters[i].len() as f64;
            //println!("moved: {:?}", centroids[i]);
        } else {
            centroids[i].blue = 1000.0;
            centroids[i].green = 1000.0;
            centroids[i].red = 1000.0;
        }
    }
}

fn lbg_cluster(centroids: &CentroidVec, pixels: &PixelVec) -> Vec<PixelVec> {
    let grouped_pixels = pixels
        .iter()
        .map(|p| centroids
            .iter()
            .map(|c| (p.blue as f64 - c.blue).abs() + (p.green as f64 - c.green).abs() + (p.red as f64 - c.red).abs())
            .enumerate()
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap()
        ).collect::<Vec<usize>>();

    let mut clusters = Vec::new();
    clusters.resize(centroids.len(), Vec::new());
    for i in 0..grouped_pixels.len() {
        clusters[grouped_pixels[i]].push(pixels[i]);
    }
    return clusters;
}

pub fn create_lbg_dictionary(pixels: &PixelVec, no_bits: u8, no_repeats: usize, error: f64) -> PixelVec {
    let mut rng = rand::thread_rng();
    let noise = Uniform::new_inclusive(-1, 1);
    let no_colours = 2_i32.pow(no_bits as u32) as usize;
    let no_pixels = pixels.len();

    let avg_blue = extract_colour(&pixels, &Hue::BLUE).iter().map(|&v| v as usize).sum::<usize>() as f64 / no_pixels as f64;
    let avg_green = extract_colour(&pixels, &Hue::GREEN).iter().map(|&v| v as usize).sum::<usize>() as f64 / no_pixels as f64;
    let avg_red = extract_colour(&pixels, &Hue::RED).iter().map(|&v| v as usize).sum::<usize>() as f64 / no_pixels as f64;
    let avg = Centroid{blue: avg_blue, green: avg_green, red: avg_red};

    let mut mse = f64::MAX;
    let mut centroids = vec![avg];
    let mut no_centroids = 1;
    let mut repeat = 0;
    while repeat < no_repeats {
        //println!("{:?}", centroids);
        if no_centroids < no_colours {
            for j in 0..no_centroids {
                let new_centroid = centroids[j].clone();
                centroids.push(new_centroid);
            }
            no_centroids *= 2;
        }
        for j in 0..no_centroids {
            centroids[j].blue += noise.sample(&mut rng) as f64;
            centroids[j].green += noise.sample(&mut rng) as f64;
            centroids[j].red += noise.sample(&mut rng) as f64;
        }
        let clusters = lbg_cluster(&centroids, &pixels);
        lbg_move_centroids(&mut centroids, &clusters);
        let dictionary = centroids
            .iter()
            .map(|c| Colour{blue: c.blue.round() as u8, green: c.green.round() as u8, red: c.red.round() as u8})
            .collect();
        let quantized_pixels = vector_quantize(&pixels, &dictionary);
        let new_mse = calculate_mse(&colour_to_bytes(&pixels), &colour_to_bytes(&quantized_pixels));
        if (mse - new_mse).abs() < error {
            println!("quitting lbg due to error change reaching the set threshold");
            break;
        } else {
            mse = new_mse;
            repeat += 1;
        }
    }
    if repeat == no_repeats {
        println!("quitting lbg due to max repeats reached");
    }
    return centroids
        .iter()
        .map(|c| Colour{blue: c.blue.round() as u8, green: c.green.round() as u8, red: c.red.round() as u8})
        .collect();
}

pub fn vector_quantize(pixels: &PixelVec, dictionary: &PixelVec) -> PixelVec {
    return pixels
        .iter()
        .map(|p| dictionary
            .iter()
            .min_by_key(|c| p.blue.abs_diff(c.blue) as usize + p.green.abs_diff(c.green) as usize + p.red.abs_diff(c.red) as usize)
            .cloned()
            .unwrap()
        ).collect::<PixelVec>();
    
}

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

pub fn calculate_mse(original: &Vec<u8>, quantized: &Vec<u8>) -> f64 {
    let no_bytes = original.len();
    return 1.0 / no_bytes as f64 * (0..no_bytes)
        .map(|i| original[i].abs_diff(quantized[i]) as f64)
        .fold(0f64, |acc, diff| acc + diff.powf(2.0));
}

pub fn calculate_snr(original: &Vec<u8>, mse: f64) -> f64 {
    let no_bytes = original.len();
    return 1.0 / no_bytes as f64 / mse * (0..no_bytes)
        .fold(0f64, |acc, i| acc + (original[i] as f64).powf(2.0))
}

/*
pub fn calculate_mse(original: &PixelVec, quantized: &PixelVec) -> f64 {
    let no_pixels = original.len();
    return 1.0 / no_pixels as f64 * (0..no_pixels)
        .map(|i| (original[i].blue.abs_diff(quantized[i].blue) as f64, original[i].green.abs_diff(quantized[i].green) as f64, original[i].red.abs_diff(quantized[i].red) as f64))
        .fold(0f64, |acc, diff| acc + diff.0.powf(2.0) + diff.1.powf(2.0) + diff.2.powf(2.0))
}

pub fn calculate_snr(original: &PixelVec, mse: f64) -> f64 {
    let no_pixels = original.len();
    return 1.0 / no_pixels as f64 / mse * (0..no_pixels)
        .fold(0f64, |acc, i| acc + (original[i].blue as f64).powf(2.0) + (original[i].green as f64).powf(2.0) + (original[i].red as f64).powf(2.0));
}
*/

pub fn write_tga(out: &str, header: &[u8], pixels: &PixelVec, footer: &[u8]) {
    let mut image_bytes = header.to_vec();
    image_bytes.extend(colour_to_bytes(&pixels));
    image_bytes.extend(footer);
    fs::write(out, image_bytes).expect("can't write to file");
}
