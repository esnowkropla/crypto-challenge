use num::bigint::{ToBigUint, BigUint};
use std::num::{from_str_radix};
use std::fmt::radix;

pub fn unhex(x: &str) -> BigUint {
    match from_str_radix(x, 16) {
        Some(text) => text,
        None => fail!("Couldn't unhex")
    }
}

pub fn hex(big: BigUint) -> String {
    let mut x = big;
    let mut out = String::from_str("");

    while x > 0u.to_biguint().unwrap() {
        let chr : u8 = x.rem(&16u.to_biguint().unwrap()).to_u8().unwrap();
        out.push_str( format!("{}", radix(chr, 16)).as_slice() );
        x = x >> 4;
    }

    return out.as_slice().chars().rev().collect();
}

#[test]
fn test_hex_unhex() {
    let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert!(hex(unhex(x.as_slice())).as_slice() == x);
}
