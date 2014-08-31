extern crate crypto;

use std::collections::hashmap::HashSet;
use crypto::{Buffer, english_dict, lowercase};

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

fn main() {
    let buf = Buffer::unhex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let dict = english_dict();

    let mut leader = 0u;
    let mut candidate = 0u8;
    for key in range(0u8, 128) {
        let key_buf = Buffer::from_elem(buf.len(), key);
        let code = key_buf^buf;
        let score = score_fn(code.utf8().as_slice(), &dict);
        if score > leader {
            leader = score;
            candidate = key;
        }
    }
    let decrypt = Buffer::from_elem(buf.len(), candidate);
    println!("Best guess:{}\nByte: {}, Score: {}", (decrypt^buf).utf8(), candidate, leader);
}
