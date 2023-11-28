type BIT = u8;

const BYTE_SIZE: u8 = 8;

fn convert_to_bin_rev(n: usize) -> Vec<BIT> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..=no_bits)
        .map(|bit| ((n >> bit) & 1) as BIT)
        .collect();
}

fn convert_to_bin(n: usize) -> Vec<BIT> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..=no_bits)
        .rev()
        .map(|bit| ((n >> bit) & 1) as BIT)
        .collect();
}

fn convert_to_bin_no_leading(n: usize) -> Vec<u8> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..no_bits)
        .rev()
        .map(|bit| ((n >> bit) & 1) as BIT)
        .collect();
}

fn bytes_to_bits(v: &[u8]) -> Vec<BIT> {
    let b = v
        .iter()
        .fold(Vec::new(), |mut bits, byte| {
            bits.push((0..BYTE_SIZE)
                .rev()
                .fold(0, |acc, pos| ((byte >> pos) & 1)));
            return bits;
        });
    println!("{:?}", b);
    return b;
}



pub fn gamma_encode_one(value: usize) -> Vec<BIT> {
    //let no_bits = 
    let mut bin_rep = convert_to_bin(value + 1);
    let mut coded = vec![0; bin_rep.len() - 1];
    coded.append(&mut bin_rep);
    return coded;
}

pub fn gamma_encode(values: &[usize]) -> Vec<BIT> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut gamma_encode_one(val));
        return coded;
    });
}

pub fn gamma_decode_one(coded: &[BIT]) -> Option<(usize, &[BIT])> {
    let one_idx = coded
        .iter()
        .position(|&bit| bit == 1);
    if let Some(value_idx) = one_idx {
        //println!("{:?}", Into::<Vec<_>>::into(&coded[value_idx..=(2 * value_idx)]));
        let value = coded[value_idx..=(2 * value_idx)]
            .iter()
            .fold(0, |acc, &bit| (acc << 1) + bit as usize);
        //println!("{}", value);
        return Some((value - 1, &coded[(2 * value_idx + 1)..]));
    } else {
        return None;
    }
}

pub fn gamma_decode(coded: &[u8]) -> Vec<usize> {
    let mut coded_left = &bytes_to_bits(&coded)[..];
    let mut values = Vec::new();
    while coded_left.len() > 0 {
        if let Some((decoded_value, left)) = gamma_decode_one(&coded_left) {
            values.push(decoded_value);
            coded_left = left;
        } else {
            break;
        }
    }
    return values;
}



pub fn delta_encode_one(value: usize) -> Vec<BIT> {
    let mut bin_rep = convert_to_bin_no_leading(value + 1);
    let mut coded = gamma_encode_one(bin_rep.len());
    coded.append(&mut bin_rep);
    return coded;
}

pub fn delta_encode(values: &[usize]) -> Vec<BIT> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut delta_encode_one(val));
        return coded;
    });
}

pub fn delta_decode_one(coded: &[BIT]) -> Option<(usize, &[BIT])> {
    let code_split = gamma_decode_one(coded);
    if let Some((bin_rep_len, coded_left)) = code_split {
        let value = (1 << bin_rep_len) + coded_left[..bin_rep_len]
            .iter()
            .fold(0, |acc, &bit| (acc << 1) + bit as usize);
        return Some((value - 1, &coded_left[bin_rep_len..]));
    } else {
        return None;
    }
}

pub fn delta_decode(coded: &[u8]) -> Vec<usize> {
    let mut coded_left = &bytes_to_bits(&coded)[..];
    let mut values = Vec::new();
    while coded_left.len() > 0 {
        if let Some((decoded_value, left)) = delta_decode_one(&coded_left) {
            values.push(decoded_value);
            coded_left = left;
        } else {
            break;
        }
    }
    return values;
}



pub fn omega_encode_one(mut value: usize) -> Vec<BIT> {
    value += 1;
    let mut coded = vec![0];
    while value > 1 {
        let mut bin_rep = convert_to_bin_rev(value);
        //println!("{:?}", bin_rep);
        value = bin_rep.len() - 1;
        coded.append(&mut bin_rep);
    }
    return coded
        .iter()
        .rev()
        .map(|&b| b)
        .collect();
}

pub fn omega_encode(values: &[usize]) -> Vec<BIT> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut omega_encode_one(val));
        return coded;
    })
}

pub fn omega_decode_one(coded: &[BIT]) -> Option<(usize, &[BIT])> {
    let mut coded_left = coded;
    let mut value = 1;
    while coded_left[0] != 0 {
        //println!("{:?}", <&[char] as Into<Vec<char>>>::into(&coded[..=value]));
        if coded_left.len() > value + 1 {
            let cutoff = value + 1;
            value = coded_left[..cutoff]
                .iter()
                .fold(0, |acc, &bit| (acc << 1) + bit as usize);
            coded_left = &coded_left[cutoff..];
        } else {
            return None;
        }
    }
    return Some((value - 1, &coded_left[1..]));
}

pub fn omega_decode(coded: &[u8]) -> Vec<usize> {
    let mut coded_left = &bytes_to_bits(&coded)[..];
    let mut values = Vec::new();
    while coded_left.len() > 0 {
        if let Some((decoded_value, left)) = omega_decode_one(&coded_left) {
            values.push(decoded_value);
            coded_left = left;
        } else {
            break;
        }
    }
    return values;
}
