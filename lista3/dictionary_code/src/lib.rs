use std::fs::File;

pub mod lzw;
pub mod elias_code;
pub mod fibonacci_code;

pub enum CodeType {
    GAMMA,
    DELTA,
    OMEGA,
}

pub fn compress_str(message: String, code: CodeType) -> Vec<char> {
    let lzw_code = lzw::encode(message.as_bytes());
    return match code {
        CodeType::GAMMA => elias_code::gamma_encode_seq(&lzw_code),
        CodeType::DELTA => elias_code::delta_encode_seq(&lzw_code),
        CodeType::OMEGA => elias_code::omega_encode_seq(&lzw_code),
    }
}

pub fn compress_bytes(message_bytes: &Vec<u8>, code: CodeType) -> Vec<u8> {
    let lzw_code = lzw::encode(message_bytes);
    return match code {
        CodeType::GAMMA => elias_code::gamma_encode_seq(&lzw_code),
        CodeType::DELTA => elias_code::delta_encode_seq(&lzw_code),
        CodeType::OMEGA => elias_code::omega_encode_seq(&lzw_code),
    }
}

pub fn decompress_bytes(coded: &mut Vec<char>, code: CodeType) -> Vec<u8> {
    let lzw_code = match code {
        CodeType::GAMMA => elias_code::gamma_decode_seq(coded),
        CodeType::DELTA => elias_code::delta_decode_seq(coded),
        CodeType::OMEGA => elias_code::omega_decode_seq(coded),
    };
    return lzw::decode(&lzw_code);
}

pub fn compress(input: &mut File, output: &mut File, code: CodeType) {

    // lzw encoding
    
    let encoded = lzw::encode("abababa".as_bytes());
    println!("{:?}", encoded);

    // gamma
    
    println!("--- gamma ---");

    let mut gamma = elias_code::gamma_encode_seq(&encoded);
    println!("{:?}", gamma);
    let goriginal = elias_code::gamma_decode_seq(&mut gamma);
    println!("{:?}", gamma);
    println!("{:?}", goriginal);

    // delta
    
    println!("--- delta ---");

    let mut delta = elias_code::delta_encode_seq(&encoded);
    println!("{:?}", delta);
    let doriginal = elias_code::delta_decode_seq(&mut delta);
    println!("{:?}", delta);
    println!("{:?}", doriginal);

    // omega

    println!("--- omega ---");
    
    let mut omega = elias_code::omega_encode_seq(&encoded);
    println!("{:?}", omega);
    let ooriginal = elias_code::omega_decode_seq(&mut omega);
    println!("{:?}", omega);
    println!("{:?}", ooriginal);

    // lzw decoding
    
    println!("--- original---");
    
    let decoded = lzw::decode(&ooriginal);
    println!("{:?}", decoded);
}
