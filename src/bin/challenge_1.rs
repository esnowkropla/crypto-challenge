extern crate crypto;

use crypto::Buff;

fn main() {
    let buf = Buff::unhex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("The hex input {} converts to the base64 output {}", buf.hex(), buf.base64());
}
