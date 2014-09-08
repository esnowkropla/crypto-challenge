extern crate crypto;
extern crate num;

use crypto::{Buff, levenshtein};
use num::bigint::{ToBigUint};
use std::num::Zero;

fn hamming(a: &Buff, b: &Buff) -> uint {
    let mut acc = 0;
    let mut c = (*a^*b).to_big();
    let two = 2u.to_biguint().unwrap();
    while c > Zero::zero() {
        acc += (c % two).to_uint().unwrap();
        c = c >> 1;
    }
    return acc;
}

fn main() {
    let a = Buff::unhex("1");
    let b = Buff::unhex("2");
    println!("{}\n{:t}\n{}", a.to_big().to_u8().unwrap(), b.to_big().to_u8().unwrap(), hamming(&a, &b));
    println!("{}", levenshtein("etaoinshrdlcumwfgypbvkjxqz", ""));
    println!("Hamming: {}", hamming(&Buff::from_slice("this is a test"), &Buff::from_slice("wokka wokka!!!")));
}
