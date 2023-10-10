use std::fs::File;
use std::io::Read;

type ConditionalOccurenceTable = [[u64; ALPHABET_SIZE]; ALPHABET_SIZE];
type TotalOccurenceTable = [u64; ALPHABET_SIZE];

const ALPHABET_SIZE: usize = 256;
const BUFFER_SIZE: usize = 32;

// compute conditional occurences for each byte
fn count_cond_occurences(file: &mut File) -> ConditionalOccurenceTable {
    let mut cond_occ = [[0; ALPHABET_SIZE]; ALPHABET_SIZE];
    let mut buffer: [u8; BUFFER_SIZE] = Default::default();

    let mut prev: usize = 0;
    loop {
        let n = file.read(&mut buffer).expect("invalid file read");

        for curr in buffer {
            cond_occ[curr as usize][prev] += 1;
            prev = curr as usize;
        }

        if n < BUFFER_SIZE {
            break;
        }
    }

    return cond_occ;
}

// compute total occurences from the conditional occurences table
fn count_total_occurences(cond_occ: ConditionalOccurenceTable) -> TotalOccurenceTable {
    let mut total_occ = [0; ALPHABET_SIZE];

    for letter in 0..ALPHABET_SIZE {
        total_occ[letter] = cond_occ[letter].iter().sum();
    }

    return total_occ;
}

fn main() {
    //let mut conditional_occurences = [[0; ALPHABET_SIZE]; ALPHABET_SIZE];
    //let mut occurences
    //let mut word_size = 0;

    //let mut file = File::open("/home/pawles/.cargo/bin/rustc").expect("can't read the file");
    let mut file = File::open("../files/Rabi-Ribi Original Soundtrack - 43 M.R..flac").expect("can't read the file");
    let cond_occurences = count_cond_occurences(&mut file);

    let total_occurences = count_total_occurences(cond_occurences);

    let total_length: u64 = total_occurences.iter().sum();

    println!("conditional occurences: {:?}", cond_occurences);
    println!("total occurences: {:?}", total_occurences);
    println!("total bytes: {}", total_length);

    /*
    let mut conditional_entropy = 0.0;
    for letter in 0..ALPHABET_SIZE {
        for previous_letter in 0..ALPHABET_SIZE {

        }
        let occurence_probability: f64 = occurences[i] as f64 / word_size as f64;
        entropy += -occurence_probability * f64::log2(occurence_probability);
    }
    println!("entropy = {}", entropy);
    */
}
