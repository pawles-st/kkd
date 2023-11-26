fn convert_to_bin_rev(n: usize) -> Vec<char> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..=no_bits)
        .map(|bit| {
            let bit_value = ((n >> bit) & 1) as u32;
            char::from_digit(bit_value, 2).unwrap()
        })
        .collect();
}

fn convert_to_bin(n: usize) -> Vec<char> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..=no_bits)
        .rev()
        .map(|bit| {
            let bit_value = ((n >> bit) & 1) as u32;
            char::from_digit(bit_value, 2).unwrap()
        })
        .collect();
}

fn convert_to_bin_no_leading(n: usize) -> Vec<char> {
    let no_bits = f64::log2(n as f64).floor() as usize;
    return (0..no_bits)
        .rev()
        .map(|bit| {
            let bit_value = ((n >> bit) & 1) as u32;
            char::from_digit(bit_value, 2).unwrap()
        })
        .collect();
}

pub fn gamma_encode(value: usize) -> Vec<char> {
    let mut bin_rep = convert_to_bin(value + 1);
    let mut coded = vec!['0'; bin_rep.len() - 1];
    coded.append(&mut bin_rep);
    return coded;
}

pub fn gamma_encode_seq(values: &Vec<usize>) -> Vec<char> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut gamma_encode(val));
        return coded;
    });
}

pub fn gamma_decode(coded: &mut Vec<char>) -> Option<usize> {
    let one_idx = coded
        .iter()
        .position(|&bit| bit == '1');
    if let Some(value_idx) = one_idx {
        //println!("{:?}", Into::<Vec<_>>::into(&coded[value_idx..=(2 * value_idx)]));
        let value = coded
            .drain(..=(2 * value_idx))
            .skip(value_idx)
            .fold(0, |acc, bit| (acc << 1) + bit.to_digit(2).unwrap() as usize);
        //println!("{}", value);
        return Some(value - 1);
    } else {
        return None;
    }
}

pub fn gamma_decode_seq(coded: &mut Vec<char>) -> Vec<usize> {
    let mut values = Vec::new();
    while coded.len() > 0 {
        if let Some(decoded_value) = gamma_decode(coded) {
            values.push(decoded_value);
        } else {
            break;
        }
    }
    return values;
}

pub fn delta_encode(value: usize) -> Vec<char> {
    let mut bin_rep = convert_to_bin_no_leading(value + 1);
    let mut coded = gamma_encode(bin_rep.len());
    coded.append(&mut bin_rep);
    return coded;
}

pub fn delta_encode_seq(values: &Vec<usize>) -> Vec<char> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut delta_encode(val));
        return coded;
    });
}

pub fn delta_decode(coded: &mut Vec<char>) -> Option<usize> {
    let next_len = gamma_decode(coded);
    if let Some(bin_rep_len) = next_len {
        let value = (1 << bin_rep_len) + coded
            .drain(..bin_rep_len)
            .fold(0, |acc, bit| (acc << 1) + bit.to_digit(2).unwrap() as usize);
        return Some(value - 1);
    } else {
        return None;
    }
}

pub fn delta_decode_seq(coded: &mut Vec<char>) -> Vec<usize> {
    let mut values = Vec::new();
    while coded.len() > 0 {
        if let Some(decoded_value) = delta_decode(coded) {
            values.push(decoded_value);
        } else {
            break;
        }
    }
    return values;
}

pub fn omega_encode(mut value: usize) -> Vec<char> {
    value += 1;
    let mut coded = vec!['0'];
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

pub fn omega_encode_seq(values: &Vec<usize>) -> Vec<char> {
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut omega_encode(val));
        return coded;
    })
}

pub fn omega_decode(coded: &mut Vec<char>) -> Option<usize> {
    let mut value = 1;
    while coded[0] != '0' {
        //println!("{:?}", <&[char] as Into<Vec<char>>>::into(&coded[..=value]));
        if coded.len() > value + 1 {
            value = coded
                .drain(..=value)
                .fold(0, |acc, bit| (acc << 1) + bit.to_digit(2).unwrap() as usize);
        } else {
            return None;
        }
    }
    coded.drain(..=0);
    return Some(value - 1);
}

pub fn omega_decode_seq(coded: &mut Vec<char>) -> Vec<usize> {
    let mut values = Vec::new();
    while coded.len() > 0 {
        if let Some(decoded_value) = omega_decode(coded) {
            values.push(decoded_value);
        } else {
            break;
        }
    }
    return values;
}
