use dictionary_code::lzw;
use dictionary_code::elias_code;
use dictionary_code::fibonacci_code;

#[test]
fn lzw_test() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    assert_eq!(encoded, vec![97, 98, 256, 258]);
}

#[test]
fn gamma_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut gamma_encoded = elias_code::gamma_encode_seq(&encoded);
    assert_ne!(gamma_encoded.len() % 8, 0);
    assert_eq!(gamma_encoded, vec!['0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1']);

    let gamma_decoded = elias_code::gamma_decode_seq(&mut gamma_encoded);
    assert_eq!(gamma_encoded.len(), 0);
    assert_eq!(gamma_decoded, encoded);
}

#[test]
fn gamma_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut gamma_encoded = elias_code::gamma_encode_seq(&encoded);
    for _ in 0..(8 - gamma_encoded.len() % 8) {
        gamma_encoded.push('0');
    }
    assert_eq!(gamma_encoded, vec!['0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0']);

    let gamma_decoded = elias_code::gamma_decode_seq(&mut gamma_encoded);
    assert_eq!(gamma_decoded, encoded);
}

#[test]
fn delta_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut delta_encoded = elias_code::delta_encode_seq(&encoded);
    assert_ne!(delta_encoded.len() % 8, 0);
    assert_eq!(delta_encoded, vec!['0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1']);

    let delta_decoded = elias_code::delta_decode_seq(&mut delta_encoded);
    assert_eq!(delta_encoded.len(), 0);
    assert_eq!(delta_decoded, encoded);
}

#[test]
fn delta_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut delta_encoded = elias_code::delta_encode_seq(&encoded);
    for _ in 0..(8 - delta_encoded.len() % 8) {
        delta_encoded.push('0');
    }
    assert_eq!(delta_encoded, vec!['0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '0', '0']);

    let delta_decoded = elias_code::delta_decode_seq(&mut delta_encoded);
    assert_eq!(delta_decoded, encoded);
}

#[test]
fn omega_test_no_pad() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut omega_encoded = elias_code::omega_encode_seq(&encoded);
    assert_ne!(omega_encoded.len() % 8, 0);
    assert_eq!(omega_encoded, vec!['1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0']);

    let omega_decoded = elias_code::omega_decode_seq(&mut omega_encoded);
    assert_eq!(omega_encoded.len(), 0);
    assert_eq!(omega_decoded, encoded);
}

#[test]
fn omega_test_padding() {
    let test_message = "abababa";
    let encoded = lzw::encode(test_message.as_bytes());
    
    let mut omega_encoded = elias_code::omega_encode_seq(&encoded);
    for _ in 0..(8 - omega_encoded.len() % 8) {
        omega_encoded.push('1');
    }
    assert_eq!(omega_encoded, vec!['1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '0', '0', '1', '0', '1', '1', '0', '1', '1', '0', '0', '0', '1', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '1', '0', '1', '1', '1', '1', '1', '1']);

    let omega_decoded = elias_code::omega_decode_seq(&mut omega_encoded);
    assert_eq!(omega_decoded, encoded);
}

#[test]
fn fibonacci_test_no_pad() {
    panic!("not implemented");
}

#[test]
fn fibonacci_test_padding() {
    panic!("not implemented");
}
