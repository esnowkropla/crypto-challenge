extern crate crypto;

use std::uint::MAX;
use crypto::{Buff, score_fn};

fn main() {
    let buf = Buff::unhex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut leader = MAX;
    let mut candidate = 0u8;
    for key in range(0u8, 255) {
        let key_buf = Buff::from_elem(buf.len(), key);
        let code = key_buf^buf;
        let score = match code.utf8() {
            Some(e) => score_fn(e.as_slice()),
            None => MAX
        };
        if score < leader {
            leader = score;
            candidate = key;
        }
    }
    let decrypt = Buff::from_elem(buf.len(), candidate);
    println!("Best guess:{}\nByte: {}, Score: {}", (decrypt^buf).utf8(), candidate, leader);
}
