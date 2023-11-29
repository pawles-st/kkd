use std::env;
use std::fs::File;
use entropy::*;

fn main() {
    let args: Vec<String>  = env::args().collect();
    let mut file = File::open(&args[1]).expect("Can't read the file");

    let entropy = calculate_entropy(&mut file);
    println!("entropy = {}", entropy);
}
