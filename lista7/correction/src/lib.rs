use bitvec::prelude::*;
use rand::prelude::*;
use std::iter::zip;

#[derive(Debug)]
pub enum CodingError {
    InvalidNumberOfBits,
    TooManyErrors,
}

pub fn count_deformed_bits(original: &BitVec, deformed: &BitVec) -> Result<usize, CodingError> {
    if original.len() != deformed.len() {
        return Err(CodingError::InvalidNumberOfBits);
    }

    return Ok(zip(original, deformed)
        .fold(0, |acc, (og_bit, df_bit)| {
            if og_bit == df_bit {
                acc
            } else {
                acc + 1
            }
        }));
}

pub fn encode(bits: &BitVec) -> Result<BitVec, CodingError> {
    if bits.len() != 4 {
        return Err(CodingError::InvalidNumberOfBits);
    }

    let mut code = BitVec::new();

    code.push(bits[0]);
    code.push(bits[0] ^ bits[1]);
    code.push(bits[1] ^ bits[2]);
    code.push(bits[0] ^ bits[2] ^ bits[3]);
    code.push(bits[1] ^ bits[3]);
    code.push(bits[2]);
    code.push(bits[3]);
    code.push(bits[0] ^ bits[1] ^ bits[2] ^ bits[3]);

    return Ok(code);
}

pub fn check(code: &BitVec) -> Result<BitVec, CodingError> {
    if code.len() != 8 {
        return Err(CodingError::InvalidNumberOfBits);
    }

    let mut syndrome = BitVec::new();

    syndrome.push(code[2] ^ code[4] ^ code[5] ^ code[6]);
    syndrome.push(code[1] ^ code[3] ^ code[4] ^ code[5]);
    syndrome.push(code[0] ^ code[2] ^ code[3] ^ code[4]);
    syndrome.push(code[0] ^ code[1] ^ code[2] ^ code[3] ^ code[4] ^ code[5] ^ code[6] ^ code[7]);

    return Ok(syndrome);
}

pub fn decode(code: &BitVec, syndrome: &BitVec) -> Result<(BitVec, bool), CodingError> {
    if code.len() != 8 || syndrome.len() != 4 {
        return Err(CodingError::InvalidNumberOfBits);
    }

    let mut corrected;
    let mut two_errors = false;

    if *syndrome == bitvec![0, 0, 0, 0] { // no errors
        corrected = code.clone();
    } else {
        if syndrome[3] == false {
            two_errors = true;
        }
        let bit_flipped = if syndrome[0..3].to_bitvec() == bitvec![0, 0, 1] {
            0
        } else if syndrome[0..3].to_bitvec() == bitvec![0, 1, 0] {
            1
        } else if syndrome[0..3].to_bitvec() == bitvec![1, 0, 1] {
            2
        } else if syndrome[0..3].to_bitvec() == bitvec![0, 1, 1] {
            3
        } else if syndrome[0..3].to_bitvec() == bitvec![1, 1, 1] {
            4
        } else if syndrome[0..3].to_bitvec() == bitvec![1, 1, 0] {
            5
        } else if syndrome[0..3].to_bitvec() == bitvec![1, 0, 0] {
            6
        } else {
            7
        };

        corrected = BitVec::new();
        for i in 0..8 {
            if i == bit_flipped {
                corrected.push(code[i] ^ true);
            } else {
                corrected.push(code[i]);
            }
        }
    }

    //println!("{:?}", corrected);

    let mut data = BitVec::new();
    data.push(corrected[0]);
    data.push(corrected[0] ^ corrected[1]);
    data.push(corrected[5]);
    data.push(corrected[6]);
    return Ok((data, two_errors));
}

pub fn noise(bits: &BitVec, p: f32) -> BitVec {
    let mut result = BitVec::new();

    let mut rng = rand::thread_rng();

    for bit in bits {
        let flip: f32 = rng.gen();
        if flip < p {
            result.push(*bit ^ true);
        } else {
            result.push(*bit);
        }
    }

    return result;
}

pub fn verify(decoded: &BitVec, original: &BitVec) -> bool {
    if decoded == original {
        return true;
    } else {
        return false;
    }
}

