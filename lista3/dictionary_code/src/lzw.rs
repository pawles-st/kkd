const NO_BYTES: usize = 256;

type Byte = u8;

#[derive(Debug)]
struct Dictionary {
    value: usize,
    next_entries: [Option<Box<Dictionary>>; NO_BYTES],
}

impl Dictionary {
    fn new(val: usize) -> Self {
        Self {
            value: val,
            next_entries: (0..NO_BYTES)
                .map(|_| None)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Should have turned vec into arr"),
        }
    }

    fn new_root() -> Self {
        Self {
            value: 0,
            next_entries: (0..NO_BYTES)
                .map(|val| Some(Box::new(Self::new(val))))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Should have turned vec into arr"),
        }
    }

    fn add(&mut self, value: usize, pos: usize) {
        self.next_entries[pos] = Some(Box::new(Self::new(value)));
    }
}

pub fn encode(message: &[Byte]) -> Vec<usize> {
    let mut coded: Vec<usize> = Vec::new();

    let mut root = Dictionary::new_root();
    let mut curr_prefix = &mut root;
    let mut next_value = NO_BYTES;

    for byte in message {
        if curr_prefix.next_entries[*byte as usize].is_none() {
            coded.push(curr_prefix.value);
            
            curr_prefix.add(next_value, *byte as usize);
            next_value += 1;
            
            curr_prefix = &mut root;
        }
        curr_prefix = curr_prefix.next_entries[*byte as usize].as_mut().unwrap();
    }
    coded.push(curr_prefix.value); // probably doesn't need 'if'

    return coded;
}

pub fn decode(coded: &[usize]) -> Vec<Byte> {
    let mut decoded: Vec<Byte> = Vec::new();

    let mut dict_bytes: Vec<Vec<Byte>> = (0..NO_BYTES)
        .map(|byte| vec![byte as u8])
        .collect();

    for index in coded {
        if dict_bytes.len() > NO_BYTES {
            let last_symbol = dict_bytes[*index][0].clone();
            dict_bytes.last_mut()
                .unwrap()
                .push(last_symbol);
        }

        decoded.extend(&dict_bytes[*index]);

        dict_bytes.push(dict_bytes[*index].clone());
    }

    return decoded;
}

