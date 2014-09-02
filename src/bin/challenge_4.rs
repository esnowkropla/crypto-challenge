extern crate crypto;

use std::collections::hashmap::HashSet;
use crypto::{Buff, english_dict, lowercase, read_file};

fn score_fn(haystack: &str, dict: &HashSet<String>) -> uint {
    let mut acc = 0;
    for word in haystack.split(' ') {
        if dict.contains(&lowercase(word)) {
            acc += word.len();
            println!("Found \"{}\" in \"{}\"", word.as_slice(), haystack);
        }
    }
    return acc;
}

fn find_best(buf: &Buff, dict: &HashSet<String>) -> (uint, u8) {
    let mut score = 0u;
    let mut key = 0u8;
    for chr in range(0u8, 255) {
        let key_buf = Buff::from_elem(buf.len(), chr);
        let code = key_buf^*buf;
        let key_score = match code.utf8() {
            Some(stuff) => score_fn(stuff.as_slice(), dict),
            None => 0
        };
        if key_score > score {
            score = key_score;
            key = chr;
        }
    }
    return (score, key);
}

fn main () {
    let lines = read_file("/home/esk/src/rust/crypto-challenge/src/assets/4.txt");
    let dict = english_dict();

    let mut best_score = 0u;
    let mut best_buf = Buff::new(30);
    let mut best_key = 0u8;
    let mut i = 0;

    for (index, line) in lines.iter().enumerate() {
        let buf = Buff::unhex(line.as_slice());
        let (score, key) = find_best(&buf, &dict);
        if score > best_score {
            best_score = score;
            best_buf = buf.clone();
            best_key = key;
            i = index;
        }
    }
    let cipher = Buff::from_elem(30, best_key);
    println!("Best hex is \"{}\"\nDecrypt is \"{}\" at {}, coded with {}", best_buf.hex(), (best_buf^cipher).utf8().unwrap(), i, best_key);
}
