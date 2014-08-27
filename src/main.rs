extern crate num;
extern crate crypto;

use num::bigint::BigUint;
use crypto::{hex, unhex};

fn main() {
    let plainhex = "1c0111001f010100061a024b53535009181c";
    let plaintext = unhex(plainhex);

    let keyhex = "686974207468652062756c6c277320657965";
    let key = unhex(keyhex);

    let cipher : BigUint = plaintext ^ key;

    println!("{}\n{}\n{}", plaintext, key, hex(cipher));

    let x : u8 = 255;
    let y : u8 = 4;

    println!("{:08t}\n{:08t}\n{:08t}",x, y,  x ^ y);
}
