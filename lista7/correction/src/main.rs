use bitvec::prelude::*;
use std::fs;
use std::env;

use correction::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("please specify a file");
        std::process::exit(1);
    }
    let bytes = fs::read(&args[1]).unwrap();
    let bits = BitVec::<_, Lsb0>::from_vec(bytes);
    //let message = bitvec![1, 0, 1, 1];

    let mut mismatched_blocks = 0;
    let mut two_errors = 0;
    for block in bits.chunks(4) {
        let mut data = BitVec::new();
        for i in 0..4 {
            data.push(block[i]);
        }

        let code = encode(&data).unwrap();
        let distorted = noise(&code, 0.1);
        let syndrome = check(&distorted).unwrap();
        match decode(&distorted, &syndrome) {
            Ok(original) => {
                if data != original {
                    mismatched_blocks += 1;
                }
            }
            Err(_) => {
                mismatched_blocks += 1;
                two_errors += 1;
            }
        }
    }
    println!("mismatched blocks: {}", mismatched_blocks);
    println!("number of blocks with two errors: {}", two_errors);
    /*
    let code = encode(&message).unwrap();
    println!("{:?}", code);
    let distorted = noise(&code, 0.1);
    println!("{:?}", distorted);
    let syndrome = check(&distorted).unwrap();
    println!("{:?}", syndrome);
    let original = decode(&distorted, &syndrome).unwrap();
    println!("{:?}", original);
    */
}
