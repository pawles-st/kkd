pub mod lzw;
pub mod elias_code;
pub mod fibonacci_code;

type BIT = u8;

pub enum CodeType {
    GAMMA,
    DELTA,
    OMEGA,
    FIB,
}

fn pad_zeros(v: &mut Vec<BIT>) {
    if v.len() % 8 != 0 {
        for _ in 0..(8 - v.len() % 8) {
            v.push(0);
        }
    }
}

fn pad_ones(v: &mut Vec<BIT>) {
    if v.len() % 8 != 0 {
        for _ in 0..(8 - v.len() % 8) {
            v.push(1);
        }
    }
}

fn bits_to_bytes(bits: &[BIT]) -> Vec<u8> {
    return bits
        .chunks(8)
        .fold(Vec::<u8>::new(), |mut bytes, chunk| {
            bytes.push(
                chunk
                .iter()
                .fold(0, |acc, &bit| (acc << 1) + bit as u8)
            );
            bytes
        });
}

pub fn compress_str(message: String, code: &CodeType) -> Vec<u8> {
    return compress_bytes(message.as_bytes(), code);
}

pub fn compress_bytes(message_bytes: &[u8], code: &CodeType) -> Vec<u8> {
    let lzw_code = lzw::encode(message_bytes);
    return match code {
        CodeType::GAMMA => {
            let mut code_bits = elias_code::gamma_encode(&lzw_code);
            pad_zeros(&mut code_bits);
            bits_to_bytes(&code_bits)
        },
        CodeType::DELTA => {
            let mut code_bits = elias_code::delta_encode(&lzw_code);
            pad_zeros(&mut code_bits);
            bits_to_bytes(&code_bits)
        },
        CodeType::OMEGA => {
            let mut code_bits = elias_code::omega_encode(&lzw_code);
            pad_ones(&mut code_bits);
            bits_to_bytes(&code_bits)
        },
        CodeType::FIB => {
            let mut code_bits = fibonacci_code::fib_encode(&lzw_code);
            pad_ones(&mut code_bits);
            bits_to_bytes(&code_bits)
        },
    };
}

pub fn decompress_bytes(coded: &[u8], code: &CodeType) -> Vec<u8> {
    let lzw_code = match code {
        CodeType::GAMMA => elias_code::gamma_decode(&coded),
        CodeType::DELTA => elias_code::delta_decode(&coded),
        CodeType::OMEGA => elias_code::omega_decode(&coded),
        CodeType::FIB => fibonacci_code::fib_decode(&coded),
    };
    return lzw::decode(&lzw_code);
}
