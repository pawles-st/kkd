use std::fs::File;
use std::io::Read;

const ALPHABET_SIZE: usize = 256;

fn main() {
    let mut occurences = [0; ALPHABET_SIZE];
    let mut word_size = 0;

    //let mut file = File::open("/home/pawles/.cargo/bin/rustc").expect("can't read the file");
    let mut file = File::open("../files/Rabi-Ribi Original Soundtrack - 43 M.R..flac").expect("can't read the file");
    let mut buffer: [u8; 16] = Default::default();

    loop {
        let n = file.read(&mut buffer).expect("invalid read");

        word_size += n;
        for byte in buffer {
            occurences[byte as usize] += 1;
        }
        
        if n < 16 {
            break;
        }

        //println!("buffer = {:?}", buffer);
        //println!("occurences: {:?}", occurences);
    }
    println!("total bytes: {}", word_size);
    println!("occurences: {:?}", occurences);
    
    let mut entropy = 0.0;
    for i in 0..ALPHABET_SIZE {
        let occurence_probability: f64 = occurences[i] as f64 / word_size as f64;
        entropy += -occurence_probability * f64::log2(occurence_probability);
    }
    println!("entropy = {}", entropy);
}
