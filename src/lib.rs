extern crate num;

pub use base64::{u8_to_base64, base64_to_u8, big_to_base64, chunk_to_bytes};
pub use buffer::{Buff};
pub use english::{english_dict, lowercase, levenshtein, score_fn, count};
pub use xor_cipher::{single_xor, hamming};

use std::io::{File, BufferedReader, IoError};

pub mod base64;
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
