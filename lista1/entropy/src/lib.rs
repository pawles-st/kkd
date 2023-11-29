use std::fs::File;
use std::io::Read;

//type ConditionalOccurenceTable = [[usize; ALPHABET_SIZE]; ALPHABET_SIZE];
type TotalOccurenceTable = [usize; ALPHABET_SIZE];

const ALPHABET_SIZE: usize = 256;
const BUFFER_SIZE: usize = 32;

/*
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
*/

// compute total occurences from the conditional occurences table
fn count_total_occurences(file: &mut File) -> TotalOccurenceTable {
    let mut total_occ = [0; ALPHABET_SIZE];
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    loop {
        let n = file.read(&mut buf).expect("Should have read the file");

        for c in buf {
            total_occ[c as usize] += 1;
        }

        if n < BUFFER_SIZE {
            break;
        }
    }

    return total_occ;
}

fn count_total_occurences_from_bytes(bytes: &[u8]) -> TotalOccurenceTable {
    let mut total_occ = [0; ALPHABET_SIZE];

    for c in bytes {
        total_occ[*c as usize] += 1;
    }

    return total_occ;
}

/*
fn count_total_prev_occurences(cond_occ: ConditionalOccurenceTable) -> TotalOccurenceTable {
    let mut total_prev_occ = [0; ALPHABET_SIZE];

    for prev in 0..ALPHABET_SIZE {
        for letter in 0..ALPHABET_SIZE {
            total_prev_occ[prev] += cond_occ[letter][prev];
        }
    }
    
    return total_prev_occ;
}
*/

pub fn calculate_entropy(file: &mut File) -> f64 {
    let mut entropy = 0.0;

    let total_occ = count_total_occurences(file);
    let total_length: usize = total_occ.iter().map(|&v| v).sum();

    for letter in 0..ALPHABET_SIZE {
        let occurence_pbb: f64 = total_occ[letter] as f64 / total_length as f64;
        if occurence_pbb > 0.0 {
            entropy += -occurence_pbb * f64::log2(occurence_pbb);
        }
    }

    return entropy;
}

pub fn calculate_entropy_from_bytes(bytes: &[u8]) -> f64 {
    let mut entropy = 0.0;

    let total_occ = count_total_occurences_from_bytes(bytes);
    let total_length: usize = total_occ.iter().map(|&v| v).sum();
    
    for letter in 0..ALPHABET_SIZE {
        let occurence_pbb: f64 = total_occ[letter] as f64 / total_length as f64;
        if occurence_pbb > 0.0 {
            entropy += -occurence_pbb * f64::log2(occurence_pbb);
        }
    }

    return entropy;
}

/*
pub fn calculate_conditional_entropy(file: &mut File) -> f64 {
    let mut cond_entropy = 0.0;

    let cond_occ = count_cond_occurences(file);
    let total_occ = (0..ALPHABET_SIZE)
        .map(|prev| cond_occ
            .iter()
            .map(|occ| occ[prev])
            .sum()
        )
    let total_length = total_occ.iter().sum();

    for prev in 0..ALPHABET_SIZE {
        let prev_total_occurence_pbb: f64 = total_occ[prev] as f64 / total_length as f64
    }
}
*/

