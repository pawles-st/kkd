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

fn count_total_prev_occurences(cond_occ: ConditionalOccurenceTable) -> TotalOccurenceTable {
    let mut total_prev_occ = [0; ALPHABET_SIZE];

    for prev in 0..ALPHABET_SIZE {
        for letter in 0..ALPHABET_SIZE {
            total_prev_occ[prev] += cond_occ[letter][prev];
        }
    }
    
    return total_prev_occ;
}

fn main() {
    //let mut file = File::open("/home/pawles/.cargo/bin/rustc").expect("can't read the file");
    //let mut file = File::open("../files/Rabi-Ribi Original Soundtrack - 43 M.R..flac").expect("can't read the file");
    //let mut file = File::open("../files/pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt").expect("can't read the file");
    let mut file = File::open("../files/test1.bin").expect("can't read the file");
    let cond_occurences = count_cond_occurences(&mut file);

    let total_occurences = count_total_occurences(cond_occurences);

    let total_length: u64 = total_occurences.iter().sum();

    let total_prev_occurences = count_total_prev_occurences(cond_occurences);

    println!("conditional occurences: {:?}", cond_occurences);
    println!("total occurences: {:?}", total_occurences);
    println!("total bytes: {}", total_length);

    // compute the entropy

    let mut entropy = 0.0;
    for letter in 0..ALPHABET_SIZE {
        let occurence_pbb: f64 = total_occurences[letter] as f64 / total_length as f64;
        if occurence_pbb > 0.0 {
            entropy += -occurence_pbb * f64::log2(occurence_pbb);
        }
    }
    println!("entropy = {}", entropy);

    // compute the conditional entropy

    let mut cond_entropy = 0.0;
    for prev in 0..ALPHABET_SIZE {
        let total_occurence_pbb: f64 = total_occurences[prev] as f64 / total_length as f64;
        let mut letter_cond_entropy = 0.0;
        for letter in 0..ALPHABET_SIZE {
            let cond_occurence_pbb: f64 = cond_occurences[letter][prev] as f64 / total_prev_occurences[prev] as f64;
            if cond_occurence_pbb > 0.0 {
                letter_cond_entropy += -cond_occurence_pbb * f64::log2(cond_occurence_pbb);
            }
        }
        if total_occurence_pbb > 0.0 {
            cond_entropy += total_occurence_pbb * letter_cond_entropy;
        }
    }
    println!("conditional entropy = {}", cond_entropy);
}
