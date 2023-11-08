use std::collections::VecDeque;
use std::error::Error;
use std::fs;
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

fn int_to_bin2(n: u8) -> Vec<char> {
    let mut bin_rep = Vec::new();
    let mut bin_value = 0;
    let mut two = 2u8.pow(7);
    for _ in 0..8 {
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

pub fn decode(file: &String, out: &mut File, text_len: u128) -> Result<(), Box<dyn Error>> {

    let code: Vec<u8> = fs::read(file)?;
    //let text_len = total_occurences - BYTES_RANGE as u128;
    let mut exit_code = 1;

    /* create the necessary structures */

    let mut occurences: Vec<u128> = Vec::from([1u128; BYTES_RANGE]);
    let mut cum_occurences: Vec<Endpoints> = occurences.iter().scan(0, |cum_occ, &occ| {
        let old_cum_occ = *cum_occ;
        *cum_occ += occ;
        return Some(Endpoints{left: old_cum_occ, right: *cum_occ});
    }).collect();
    let mut total_occurences = BYTES_RANGE as u128;

    let mut decoded: Vec<u8> = Vec::new();

    let mut interval = Endpoints{left: MIN_LOW, right: MAX_HIGH};
    let mut tag_interval = Endpoints{left: MIN_LOW, right: MAX_HIGH};

    let interval_len = interval.right - interval.left;
    let mut possible_bytes: Vec<(u8, Endpoints)> = cum_occurences.iter().enumerate().map(|(byte, cum_occ)| (byte as u8, Endpoints{left: interval.left + interval_len * cum_occ.left / total_occurences, right: interval.left + interval_len * cum_occ.right / total_occurences})).collect();

    /* read the coded file bit by bit and retrieve the coded bytes */

    'decodeloop: for byte in code {

        let bits = int_to_bin2(byte);

        for bit in bits {

            /* reduce the tag interval */

            if bit == '1' {
                tag_interval.left = tag_interval.left + (tag_interval.right - tag_interval.left) / 2;
            } else if bit == '0' {
                tag_interval.right = tag_interval.left + (tag_interval.right - tag_interval.left) / 2;
            } else {
                eprintln!("invalid bit read while decoding: {}", bit);
                std::process::exit(1);
            }
            //println!("new tag interval: {:?}", tag_interval);

            /* retain only the bytes whose intervals overlap with the tag interval */

            possible_bytes.retain(|(_, byte_interval)| byte_interval.left < tag_interval.right && byte_interval.right > tag_interval.left);
            //println!("{:?}", possible_bytes.iter().map(|(byte, _)| *byte).collect::<Vec<u8>>());

            /* check if a byte can be identified */

            if possible_bytes.len() == 0 {
                eprintln!("no possible bytes to identify");
                std::process::exit(1);
            }

            while possible_bytes.len() == 1 {

                // recover the identified byte and append it to the result

                let (byte, _) = possible_bytes[0];
                decoded.push(byte);
                
                // check if the whole text has been recovered

                if decoded.len() as u128 == text_len {
                    exit_code = 0;
                    break 'decodeloop;
                }

                // restrict the (total) interval to the interval of the identified byte

                let interval_len = interval.right - interval.left;
                (interval.left, interval.right) = (interval.left + interval_len * cum_occurences[byte as usize].left / total_occurences, interval.left + interval_len * cum_occurences[byte as usize].right / total_occurences);
                //println!("new (total) interval: {:?}", interval);

                // scale the interval

                loop {
                    if interval.right <= MAX_HIGH / 2 {

                        // scale to the right

                        interval.left = 2 * interval.left;
                        interval.right = 2 * interval.right;
                        tag_interval.left = 2 * tag_interval.left;
                        tag_interval.right = 2 * tag_interval.right;
                        //println!("left scale: [{}, {}]; [{}, {}]", interval.left, interval.right, tag_interval.left, tag_interval.right);

                    } else if interval.left >= MAX_HIGH / 2 {
                        
                        // scale to the left

                        interval.left = 2 * interval.left - MAX_HIGH;
                        interval.right = 2 * interval.right - MAX_HIGH;
                        tag_interval.left = 2 * tag_interval.left - MAX_HIGH;
                        tag_interval.right = 2 * tag_interval.right - MAX_HIGH;
                        //println!("right scale: [{}, {}]; [{}, {}]", interval.left, interval.right, tag_interval.left, tag_interval.right);

                    } else if interval.left >= MAX_HIGH / 4 && interval.right <= MAX_HIGH * 3 / 4 {
                        
                        // scale both ways

                        interval.left = 2 * interval.left - MAX_HIGH / 2;
                        interval.right = 2 * interval.right - MAX_HIGH / 2;
                        tag_interval.left = 2 * tag_interval.left - MAX_HIGH / 2;
                        tag_interval.right = 2 * tag_interval.right - MAX_HIGH / 2;
                        //println!("scale both ways: [{}, {}]; [{}, {}]", interval.left, interval.right, tag_interval.left, tag_interval.right);

                    } else {

                        // stop scaling; quit the loop

                        break;
                    }
                }

                // check for bytes' probability recalculation

                if decoded.len() % BUFFER_SIZE == 0 {
                    for i in (decoded.len() - BUFFER_SIZE)..decoded.len() {
                        let byte = &decoded[i];
                        occurences[*byte as usize] += 1;
                    }
                    total_occurences += BUFFER_SIZE as u128;

                    cum_occurences = occurences.iter().scan(0, |cum_occ, &occ| {
                        let old_cum_occ = *cum_occ;
                        *cum_occ += occ;
                        return Some(Endpoints{left: old_cum_occ, right: *cum_occ});
                    }).collect();

                    if occurences.iter().sum::<u128>() > total_occurences {
                        eprintln!("invalid number of total occurences");
                    }
                }

                // divide the (total) subinterval into subsubintervals

                let interval_len = interval.right - interval.left;
                possible_bytes = cum_occurences.iter().enumerate().map(|(byte, cum_occ)| (byte as u8, Endpoints{left: interval.left + interval_len * cum_occ.left / total_occurences, right: interval.left + interval_len * cum_occ.right / total_occurences})).collect();

                // retain only the bytes whose intervals overlap with the tag interval

                possible_bytes.retain(|(_, byte_interval)| byte_interval.left < tag_interval.right && byte_interval.right > tag_interval.left);

            }
        }
    }
    //println!("decoded: {:?}", decoded);

    if exit_code == 1 {
        eprintln!("failed to decode the text");
        std::process::exit(1);
    }

    out.write(&decoded).expect("can't write decoded text to the specified file");

    Ok(())
}
