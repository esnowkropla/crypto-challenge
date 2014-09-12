extern crate num;

pub use challenge_1::{num_to_base64, big_to_base64};
pub use buffer::Buff;
pub use english::{english_dict, lowercase, levenshtein, score_fn, count};
pub use xor_cipher::{single_xor};

use std::io::{File, BufferedReader, IoError};
use std::num::Zero;
use num::bigint::{ToBigUint};

pub mod challenge_1;
pub mod buffer;
pub mod english;
pub mod xor_cipher;

fn unwrap_and_noline(x: Result<String, IoError>) -> String {
    let mut unwrapped = x.unwrap();
    let n = unwrapped.len();
    unwrapped.truncate(n-1);
    unwrapped
}

pub fn read_file(input: &str) -> Vec<String> {
    let path = Path::new(input);
    let file  = match File::open(&path) {
        Ok(t) => t,
        Err(e) => fail!("Couldn't open {}, got error {}", input, e)
    };
    let mut reader = BufferedReader::new(file);
    reader.lines().map(unwrap_and_noline).collect()
}

pub fn hamming(a: &Buff, b: &Buff) -> uint {
    let mut acc = 0;
    let mut c = (*a^*b).to_big();
    let two = 2u.to_biguint().unwrap();
    while c > Zero::zero() {
        acc += (c % two).to_uint().unwrap();
        c = c >> 1;
    }
    return acc;
}

#[test]
fn test_hamming() {
    assert_eq!(37, hamming(&Buff::from_slice("this is a test"), &Buff::from_slice("wokka wokka!!!")));
}
