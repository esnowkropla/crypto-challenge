extern crate crypto;

use crypto::Buff;

fn main() {
    let buf1 = Buff::unhex("1c0111001f010100061a024b53535009181c");
    let buf2 = Buff::unhex("686974207468652062756c6c277320657965");

    println!("{} XOR\n{} =\n{}", buf1.hex(), buf2.hex(), (buf1 ^ buf2).hex());
}
