extern crate num;
extern crate crypto;

use crypto::{Buff};

fn main() {
    let buf1 = Buff::unhex("1c0111001f010100061a024b53535009181c");
    let buf2 = Buff::unhex("686974207468652062756c6c277320657965");

    println!("{}\n{}", (buf1 ^ buf2).utf8(), buf2.utf8());
}
