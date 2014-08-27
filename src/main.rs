extern crate num;
extern crate crypto;

use num::bigint::BigUint;
use std::num::{from_str_radix};

fn unhex(x: &str) -> BigUint {
    match from_str_radix(x, 16) {
        Some(text) => text,
        None => fail!("Couldn't unhex")
    }
}

fn main() {
    let plainhex = "1c0111001f010100061a024b53535009181c";
    let plaintext = unhex(plainhex);

    let keyhex = "686974207468652062756c6c277320657965";
    let key = unhex(keyhex);

    println!("{}\n{}", plaintext, key);
}
