use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const BUFFER_SIZE: usize = 256;
const BYTES_RANGE: usize = 256;
const MAX_HIGH: u128 = 0x00000001000000000000000000000000;
const MIN_LOW: u128 = 0x00000000000000000000000000000000;
const INTERVAL_BITS: u32 = 128 - MAX_HIGH.leading_zeros() - 1;

struct Endpoints {
    left: u128,
    right: u128,
}

fn int_to_bin(n: u128) -> Vec<char> {
    let mut bin_rep = Vec::new();
    let mut bin_value = 0;
    let mut two = 2u128.pow(INTERVAL_BITS - 1);
    for _ in 0..INTERVAL_BITS {
        if n >= bin_value + two {
            bin_value += two;
            bin_rep.push('1');
        } else {
            bin_rep.push('0');
        }
        two /= 2;
    }
    return bin_rep;
}

pub fn encode(file: &mut File, out: &mut File) -> Result<(u128, f64, f64, f64), Box<dyn Error>> {

    let mut buf = vec![0u8; BUFFER_SIZE];

    let mut occurences: Vec<u128> = Vec::from([1u128; BYTES_RANGE]);
    let mut cum_occurences: Vec<Endpoints>;
    let mut total_occurences = BYTES_RANGE as u128;

    let mut interval = Endpoints{left: MIN_LOW, right: MAX_HIGH};
    //println!("staring subinterval: [{}, {}]", interval.left, interval.right);

    /* read the file by chunks and apply arithmetic coding to them */

    let mut coded = VecDeque::new();
    let mut coded_len = 0;
    
    let mut counter = 0;

    loop {

        /* compute the cumulative occurences sum for each byte */

        cum_occurences = occurences.iter().scan(0, |cum_occ, &occ| {
            let old_cum_occ = *cum_occ;
            *cum_occ += occ;
            return Some(Endpoints{left: old_cum_occ, right: *cum_occ});
        }).collect();

        //println!("{:?}", cum_occurences);

        /* read the next chunk of bytes */
        
        let bytes_read = file.read(&mut buf)?;

        /* find subintervals for each byte in the chunk */

        for i in 0..bytes_read {

            let byte = &buf[i];

            // compute the subinterval

            let interval_len = interval.right - interval.left;
            (interval.left, interval.right) = (interval.left + interval_len * cum_occurences[*byte as usize].left / total_occurences, interval.left + interval_len * cum_occurences[*byte as usize].right / total_occurences);

            //println!("new subinterval: [{}, {}]", interval.left, interval.right);

            // perform scaling if needed

            loop {
                if interval.right <= MAX_HIGH / 2 {

                    // scale right

                    interval.left = 2 * interval.left;
                    interval.right = 2 * interval.right;
                    //println!("scaling right");
                    //println!("new subinterval: [{}, {}]", interval.left, interval.right);

                    // append 0 and {counter} 1s to the code

                    coded.push_back('0');
                    for _ in 0..counter {
                        coded.push_back('1');
                    }
                    coded_len += 1 + counter;
                    //coded.push_str(&"1".repeat(counter));

                    // reset the counter

                    counter = 0;

                } else if interval.left >= MAX_HIGH / 2 {

                    // scale left

                    interval.left = 2 * interval.left - MAX_HIGH;
                    interval.right = 2 * interval.right - MAX_HIGH;
                    //println!("scaling left");
                    //println!("new subinterval: [{}, {}]", interval.left, interval.right);

                    // append 1 and {counter} 0s to the code

                    coded.push_back('1');
                    for _ in 0..counter {
                        coded.push_back('0');
                    }
                    coded_len += 1 + counter;
                    //coded.push_str(&"0".repeat(counter));

                    // reset the counter

                    counter = 0;

                } else if interval.left >= MAX_HIGH / 4 && interval.right <= MAX_HIGH * 3 / 4 {
                    
                    // scale both ways

                    interval.left = 2 * interval.left - MAX_HIGH / 2;
                    interval.right = 2 * interval.right - MAX_HIGH / 2;
                    //println!("scaling both ways");
                    //println!("new subinterval: [{}, {}]", interval.left, interval.right);

                    // increment the counter

                    counter += 1;

                } else {
                    
                    // can't scale further; quit the loop

                    break;
                }

                while coded.len() >= 8 {
                    let coded_byte_seq = coded.drain(..8);
                    let byte_value: u8 = coded_byte_seq.fold(0, |acc, digit| (acc << 1) + digit.to_digit(2).unwrap() as u8);
                    out.write(&[byte_value]).expect("can't write code to the specified file");
                }
            }
        }

        /* update the occurences of each byte based on the chunk */

        for i in 0..bytes_read {
            let byte = &buf[i];
            occurences[*byte as usize] += 1;
        }
        total_occurences += bytes_read as u128;

        if bytes_read == 0 {

            // finished reading file, so exit

            break;
        }
    }

    let binary_tag = int_to_bin(interval.left + (interval.right - interval.left) / 2);
    for i in 0..binary_tag.len() {
        coded.push_back(binary_tag[i]);
    }

    if coded.len() % 8 != 0 {
        coded_len += 8 - (coded.len() & 8);
        for _ in 0..8 - (coded.len() % 8) {
            coded.push_back('0');
        }
    }

    while coded.len() >= 8 {
        let coded_byte_seq = coded.drain(..8);
        let byte_value: u8 = coded_byte_seq.fold(0, |acc, digit| (acc << 1) + digit.to_digit(2).unwrap() as u8);
        out.write(&[byte_value]).expect("can't write code to the specified file");
    }

    let text_len = total_occurences - BYTES_RANGE as u128;
    let entropy = 1.0;
    let compression_rate = coded_len as f64 / text_len as f64 / 8.0;
    return Ok((text_len, entropy, compression_rate * 8.0, compression_rate));
}
