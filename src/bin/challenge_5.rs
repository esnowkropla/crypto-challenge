extern crate crypto;

use crypto::{Buff};

fn main() {
    let plaintext = r"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let plain_buf = Buff::from_slice(plaintext);

    let key = Buff::repeat(plain_buf.len(), "ICE");
    println!("Input: \"{}\"\nKey: \"{}\"\nEnciphered: \"{}\"", plain_buf.utf8().unwrap(), key.utf8().unwrap(), (plain_buf^key).hex());
}

#[test]
fn test_repeat() {
    let target = Buff::unhex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    let plaintext = r"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let plain_buf = Buff::from_slice(plaintext);
    let key = Buff::repeat(plain_buf.len(), "ICE");
    let cipher_text = plain_buf^key;

    assert_eq!(cipher_text, target);
}
