use std::uint::MAX;
use {Buff, score_fn};

pub fn single_xor(hex: &str) -> (u8, Buff, uint) {
    let buf = Buff::unhex(hex);
    let mut leader = MAX;
    let mut candidate = 0u8;
    for key in range(0u8, 255) {
        let key_buf = Buff::from_elem(buf.len(), key);
        let code = key_buf^buf;
        let score = match code.utf8() {
            Some(e) => score_fn(e.as_slice()),
            None => MAX
        };
        if score < leader {
            leader = score;
            candidate = key;
        }
    }
    let decrypt = Buff::from_elem(buf.len(), candidate) ^ buf;
    return (candidate, decrypt, leader);
}
