use bitvec::prelude::*;
use std::fs;
use std::env;

use correction::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("please specify a file and probability");
        std::process::exit(1);
    }

    // read file and probability

    let bytes = fs::read(&args[1]).unwrap();
    let bits = BitVec::<_, Lsb0>::from_vec(bytes);
    let p = (&args[2]).parse().expect("should be a float number");

    let no_blocks = bits.len() / 4;
    let mut mismatched_blocks = 0;
    let mut blocks_with_deformations = 0;
    let mut blocks_with_two_deformations = 0;

    // split data into chunks

    for block in bits.chunks(4) {
        let mut data = BitVec::new();
        for i in 0..4 {
            data.push(block[i]);
        }

        // code the chunk

        let code = encode(&data).unwrap();

        // apply noise

        let distorted = noise(&code, p);

        // count the number of deformed bits

        let deformed_bits = count_deformed_bits(&code, &distorted).unwrap();

        // gather statistics

        if deformed_bits > 0 {
            blocks_with_deformations += 1;
        }

        // compute syndrome

        let syndrome = check(&distorted).unwrap();

        // apply correction

        match decode(&distorted, &syndrome) {
            Ok((original, two_errors)) => {
                if data != original {
                    mismatched_blocks += 1;
                }
                if two_errors {
                    blocks_with_two_deformations += 1;
                }
            }
            Err(_) => {
                mismatched_blocks += 1;
            }
        }
    }

    // print statistics

    println!("{} {} {} {}", blocks_with_deformations, blocks_with_two_deformations, mismatched_blocks, no_blocks);
}
