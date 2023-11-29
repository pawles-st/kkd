type BIT = u8;

const BYTE_SIZE: u8 = 8;

fn bytes_to_bits(v: &[u8]) -> Vec<BIT> {
    let b = v
        .iter()
        .fold(Vec::new(), |mut bits, byte| {
            for pos in (0..BYTE_SIZE).rev() {
                bits.push((byte >> pos) & 1);
            }
            return bits;
        });
    return b;
}



pub fn fib_encode_one(mut value: usize, fib_table: &mut Vec<usize>) -> Vec<BIT> {
    //let mut coded = vec![1];

    value += 1;
    let max_fib = fib_table
        .iter()
        .position(|&fib_value| fib_value > value);

    let curr_fib_idx = if max_fib.is_some() {
        max_fib.unwrap() - 1
    } else {
        let mut next_fib = fib_table[fib_table.len() - 1] + fib_table[fib_table.len() - 2];
        while next_fib <= value {
            fib_table.push(next_fib);
            next_fib = fib_table[fib_table.len() - 1] + fib_table[fib_table.len() - 2];
        }
        fib_table.push(next_fib);
        fib_table.len() - 2
    };
    
    return fib_table[..=curr_fib_idx]
        .iter()
        .rev()
        .fold(vec![1], |mut coded, &fib_value| {
            if fib_value <= value {
                value -= fib_value;
                coded.push(1);
            } else {
                coded.push(0);
            }
            return coded;
        })
        .iter()
        .rev()
        .map(|&v| v)
        .collect();
    /*
    loop {
        coded.push(1);
        value -= fib_table[curr_fib_idx];

        if value == 0 {
            if curr_fib_idx > 0 {
                for _ in 0..curr_fib_idx {
                    coded.push(0);
                }
            }
            break;
        } else {
            curr_fib_idx -= 1;
            let fib_idx_decrease = fib_table[..curr_fib_idx]
                .iter()
                .rev()
                .position(|&fib_value| fib_value <= value)
                .unwrap();
            for _ in 1..fib_idx_decrease {
                coded.push(0);
            }
            curr_fib_idx -= fib_idx_decrease;
        }
    }
    return coded;
    */
}

pub fn fib_encode(values: &[usize]) -> Vec<BIT> {
    let mut fib_table = vec![1, 2];
    return values.iter().fold(Vec::new(), |mut coded, &val| {
        coded.append(&mut fib_encode_one(val, &mut fib_table));
        return coded;
    });
}

pub fn fib_decode_one<'a, 'b>(coded: &'a[BIT], fib_table: &'b mut Vec<usize>) -> Option<(usize, &'a[BIT])> {
    let double_ones = (1..coded.len())
        .position(|idx| coded[idx - 1] == 1 && coded[idx] == 1);
    if let Some(code_end) = double_ones {
        if fib_table.len() <= code_end {
            for _ in 0..(code_end + 1 - fib_table.len()) {
                let next_fib = fib_table[fib_table.len() - 1] + fib_table[fib_table.len() - 2];
                fib_table.push(next_fib);
            }
        }
        let value = (0..=code_end)
            .fold(0, |acc, idx| {
                if coded[idx] == 1 {
                    acc + fib_table[idx]
                } else {
                    acc
                }
            });
        return Some((value - 1, &coded[(code_end + 2)..]));
    } else {
        return None;
    }
}

pub fn fib_decode(coded: &[u8]) -> Vec<usize> {
    let mut fib_table = vec![1, 2];
    let mut coded_left = &bytes_to_bits(&coded)[..];
    let mut values = Vec::new();
    while coded_left.len() > 0 {
        if let Some((decoded_value, left)) = fib_decode_one(&coded_left, &mut fib_table) {
            values.push(decoded_value);
            coded_left = left;
        } else {
            break;
        }
    }
    return values;
}
