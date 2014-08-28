extern crate num;
extern crate crypto;

use crypto::{Buffer};

fn main() {
    let cipherhex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let buf = Buffer::unhex(cipherhex);
    println!("{}\n{}\n{}", buf ,buf.hex(), buf.base64());
    println!("{}", buf.utf8());
}
