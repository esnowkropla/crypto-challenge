use std::uint::MAX;
use {Buff, score_fn};

pub fn single_xor(buf: &Buff) -> (u8, Buff, uint) {
    let mut current_key = 0u8;
    let mut current_score = MAX;

    for i in range(0u, 256) {
        let test = Buff::from_elem(buf.len(), i as u8);
        let code = test ^ *buf;
        let score = match code.utf8() {
            Some(e) => score_fn(e.as_slice()),
            None => MAX
        };
        if score < current_score {
            current_score = score;
            current_key = i as u8;
        }
    }
    let decrypt = Buff::from_elem(buf.len(), current_key) ^ *buf;
    return (current_key, decrypt, current_score);
}

fn single_byte_hamming(a: u8, b: u8) -> uint {
    let mut c = a^b;
    let mut out : uint = 0;
    while c > 0 {
        out += (c%2) as uint;
        c = c >> 1;
    }
    return out;
}

pub fn hamming(a: &[u8], b: &[u8]) -> uint {
    let mut out = 0;
    for i in range(0, a.len()) {
        out += single_byte_hamming(a[i], b[i]);
    }
    return out;
}

#[test]
fn test_hamming() {
    assert_eq!(37, hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()));
}

#[test]
fn test_single_hamming() {
    assert_eq!(single_byte_hamming(255, 0), 8);
    assert_eq!(single_byte_hamming(1, 0) , 1);
    assert_eq!(single_byte_hamming(3, 0), 2);
}
