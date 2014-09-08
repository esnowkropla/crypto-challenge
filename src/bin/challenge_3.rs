extern crate crypto;

use crypto::{single_xor};

fn main() {
    let (key, decrypt, score) = single_xor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Best guess: {}\nByte: {}, Score: {}", decrypt.utf8(), key, score);
}
