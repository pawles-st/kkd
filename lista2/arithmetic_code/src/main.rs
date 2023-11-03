use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)] 
#[derive(Clone)] 
struct Endpoints {
    left: f64,
    right: f64,
}

fn float_to_binary(z: f64) -> String {
    let mut bin = String::new();
    let mut bin_value = 0.0;
    let mut two = 0.5;
    for _ in 1..=52 {
        if z >= bin_value + two {
            bin_value += two;
            bin.push('1');
        } else {
            bin.push('0');
        }
        two /= 2.0;
    }
    return bin;
}

fn main() {

    // read command line arguments

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("invalid number of arguments");
        std::process::exit(1);
    }


    /* read the input file */

    //let text = "abc";
    let text = fs::read_to_string(&args[1]).unwrap();

    let mut occurences: BTreeMap<char, u64> = BTreeMap::new();
    let mut total_occurences = 0;

    for c in text.chars() {

        // increment the total number of read characters
        total_occurences += 1;

        // increment the occurences of the character that was just read
        match occurences.get_mut(&c) {
            Some(occ) => *occ += 1,
            None => { occurences.insert(c, 1); }
        }
    }

    //println!("{:?}", occurences);
    //println!("total occ = {}", total_occurences);

    /* compute the probability of each character */

    let probabilities: BTreeMap<char, f64> = occurences.iter().map(|(c, occ)| (c.clone(), (*occ as f64) / (total_occurences as f64))).collect();
    //let probabilities: BTreeMap<char, f64> = BTreeMap::from([('a', 0.7), ('b', 0.1), ('c', 0.2)]);

    /* compute the interval endpoints for each character */

    let mut total_probability = 0.0;
    let intervals: BTreeMap<char, Endpoints> = probabilities.iter().map(|(c, pbb)| {
        let prev_total_probability = total_probability;
        total_probability += pbb;
        return (c.clone(), Endpoints{left: prev_total_probability, right: total_probability});
    }).collect();

    //println!("character probabilities: {:?}", probabilities);
    //println!("total pbb = {}", total_probability);
    //println!("character intervals: {:?}", intervals);

    /* perform arithmetic coding on the input sequence */

    println!("--- coding ---");

    //println!("starting interval: [0, 1]");

    //let mut coded = String::new()i;
    let mut coded = String::new();

    {
    let mut left = 0.0;
    let mut right = 1.0;
    let mut counter = 0;
    for c in text.chars() {

        //println!("character read: {}", c);

        // compute the new subinterval

        let interval_len = right - left;
        (left, right) = (left + interval_len * intervals[&c].left, left + interval_len * intervals[&c].right);
        //println!("new interval: [{}, {}]", left, right);

        // scale the interval

        loop {
            if right <= 0.5 {

                // scale to the right

                left = 2.0 * left;
                right = 2.0 * right;
                //println!("right scale: [{}, {}]", left, right);

                // append 0 and {counter} 1s to the code

                //println!("appending 0{} to the code", "1".repeat(counter));
                coded.push('0');
                coded.push_str(&"1".repeat(counter));

                // reset the counter

                counter = 0;

            } else if left >= 0.5 {

                // scale to the left

                left = 2.0 * left - 1.0;
                right = 2.0 * right - 1.0;
                //println!("left scale: [{}, {}]", left, right);

                // append 1 and {counter} 0s to the code

                //println!("appending 1{} to the code", "0".repeat(counter));
                coded.push('1');
                coded.push_str(&"0".repeat(counter));

                // reset the counter
                
                counter = 0;

            } else if left >= 0.25 && right <= 0.75 {

                // scale both ways

                left = 2.0 * left - 0.5;
                right = 2.0 * right - 0.5;
                //println!("both scale: [{}, {}]", left, right);

                // increment the repetition counter

                counter += 1;

            } else {

                // stop scaling: quit the loop

                break;

            }
        }
    }
    let z = left + (right - left) / 2.0;
    coded.push_str(&float_to_binary(z));
    println!("coded: {}", coded);
    //println!("interval: [{}, {}]", left, right);
    }
    
    /* decoding */
    
    println!("--- decoding ---");

    //println!("starting interval: [0, 1]");

    let mut decoded = String::new();
    let mut exit_code = 1;

    {
    let mut possible_chars = intervals.clone();
    let mut total_left = 0.0;
    let mut total_right = 1.0;
    let mut tag_left = 0.0;
    let mut tag_right = 1.0;
    'readloop: for bit in coded.chars() {

        // reduce the tag interval

        if bit == '1' {
            tag_left = tag_left + (tag_right - tag_left) / 2.0;
        } else if bit == '0' {
            tag_right = tag_left + (tag_right - tag_left) / 2.0;
        } else {
            //eprintln!("invalid bit read");
            std::process::exit(1);
        }
        //println!("new interval: [{}, {}]", tag_left, tag_right);

        // retain only the characters still possible

        possible_chars.retain(|_, range| range.left < tag_right && range.right > tag_left);
        //println!("possible characters: {:?}", possible_chars.keys());

        // check if a character can be identified

        while possible_chars.len() == 1 {

            // recover the identified character and append it to the result

            let (c, _) = possible_chars.pop_first().unwrap();
            decoded.push(c);
            //println!("character identified: {}", c);

            if decoded.len() == text.len() {
                exit_code = 0;
                break 'readloop;
            }

            // reduce the total interval based on the decoded character

            let interval_len = total_right - total_left;
            (total_left, total_right) = (total_left + interval_len * intervals[&c].left, total_left + interval_len * intervals[&c].right);
            //println!("total interval: [{}, {}]", total_left, total_right);

            // scale the interval

            loop {
                if total_right <= 0.5 {

                    // scale to the right

                    total_left = 2.0 * total_left;
                    total_right = 2.0 * total_right;
                    tag_left = 2.0 * tag_left;
                    tag_right = 2.0 * tag_right;
                    //println!("right scale: [{}, {}]; [{}, {}]", total_left, total_right, tag_left, tag_right);

                } else if total_left >= 0.5 {

                    // scale to the left

                    total_left = 2.0 * total_left - 1.0;
                    total_right = 2.0 * total_right - 1.0;
                    tag_left = 2.0 * tag_left - 1.0;
                    tag_right = 2.0 * tag_right - 1.0;
                    //println!("left scale: [{}, {}]; [{}, {}]", total_left, total_right, tag_left, tag_right);

                } else if total_left >= 0.25 && total_right <= 0.75 {

                    // scale both ways

                    total_left = 2.0 * total_left - 0.5;
                    total_right = 2.0 * total_right - 0.5;
                    tag_left = 2.0 * tag_left - 0.5;
                    tag_right = 2.0 * tag_right - 0.5;
                    //println!("both scale: [{}, {}]; [{}, {}]", total_left, total_right, tag_left, tag_right);

                } else {

                    // stop scaling: quit the loop

                    break;

                }
            }

            // divide the resulting total interval into subintervals

            possible_chars = intervals.clone().iter().map(|(c, p)| (c.clone(), Endpoints{left: total_left + (total_right - total_left) * p.left, right: total_left + (total_right - total_left) * p.right})).collect();
            //println!("new character intervals: {:?}", possible_chars);
            
            // retain only the characters still possible

            possible_chars.retain(|_, range| range.left < tag_right && range.right > tag_left);
            //println!("possible characters: {:?}", possible_chars.keys());

        }
    }
    println!("decoded: {}", decoded);
    }

    if exit_code == 1 {
        eprintln!("couldn't successfully decode the text");
        std::process::exit(1);
    }
}
