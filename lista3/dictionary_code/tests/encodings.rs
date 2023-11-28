use dictionary_code::lzw;
use dictionary_code::elias_code;
use dictionary_code::fibonacci_code;

type BIT = u8;

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

#[test]
fn lzw_test() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    assert_eq!(encoded, vec![97, 98, 256, 258]);
}

/*
#[test]
fn gamma_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let gamma_encoded = elias_code::gamma_encode(&encoded);
    assert_ne!(gamma_encoded.len() % 8, 0);
    assert_eq!(gamma_encoded, vec!['0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let mut code_bytes = bits_to_bytes(&gamma_encoded);
    let gamma_decoded = elias_code::gamma_decode(&code_bytes);
    assert_eq!(gamma_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&gamma_decoded)).unwrap(), test_message);
}
*/

#[test]
fn gamma_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut gamma_encoded = elias_code::gamma_encode(&encoded);
    pad_zeros(&mut gamma_encoded);
    assert_eq!(gamma_encoded, vec!['0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let code_bytes = bits_to_bytes(&gamma_encoded);
    let gamma_decoded = elias_code::gamma_decode(&code_bytes);
    assert_eq!(gamma_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&gamma_decoded)).unwrap(), test_message);
}

/*
#[test]
fn delta_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let delta_encoded = elias_code::delta_encode(&encoded);
    assert_ne!(delta_encoded.len() % 8, 0);
    assert_eq!(delta_encoded, vec!['0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let mut code_bytes = bits_to_bytes(&delta_encoded);
    let delta_decoded = elias_code::delta_decode(&mut code_bytes);
    assert_eq!(delta_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&delta_decoded)).unwrap(), test_message);
}
*/

#[test]
fn delta_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut delta_encoded = elias_code::delta_encode(&encoded);
    pad_zeros(&mut delta_encoded);
    assert_eq!(delta_encoded, vec!['0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let mut code_bytes = bits_to_bytes(&delta_encoded);
    let delta_decoded = elias_code::delta_decode(&mut code_bytes);
    assert_eq!(delta_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&delta_decoded)).unwrap(), test_message);
}

/*
#[test]
fn omega_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut omega_encoded = elias_code::omega_encode(&encoded);
    assert_ne!(omega_encoded.len() % 8, 0);
    assert_eq!(omega_encoded, vec!['1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let mut code_bytes = bits_to_bytes(&omega_encoded);
    let omega_decoded = elias_code::omega_decode(&mut code_bytes);
    assert_eq!(omega_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&omega_decoded)).unwrap(), test_message);
}
*/

#[test]
fn omega_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut omega_encoded = elias_code::omega_encode(&encoded);
    pad_ones(&mut omega_encoded);
    assert_eq!(omega_encoded, vec!['1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '1', '1', '1', '1', '1', '1'].iter().map(|&b| b.to_digit(2).unwrap() as u8).collect::<Vec<u8>>());

    let mut code_bytes = bits_to_bytes(&omega_encoded);
    let omega_decoded = elias_code::omega_decode(&mut code_bytes);
    assert_eq!(omega_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&omega_decoded)).unwrap(), test_message);
}

/*
#[test]
fn fibonacci_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    println!("{:?}", encoded);
    
    let fib_encoded = fibonacci_code::fib_encode(&encoded);
    println!("{:?}", fib_encoded);
}
*/

#[test]
fn fibonacci_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    println!("{:?}", encoded);
    
    let mut fib_encoded = fibonacci_code::fib_encode(&encoded);
    pad_zeros(&mut fib_encoded);
    println!("{:?}", fib_encoded);

    let code_bytes = bits_to_bytes(&fib_encoded);
    let fib_decoded = fibonacci_code::fib_decode(&code_bytes);
    assert_eq!(fib_decoded, encoded);
    assert_eq!(String::from_utf8(lzw::decode(&fib_decoded)).unwrap(), test_message);
}
