use num::bigint::{ToBigUint, BigUint};
#[cfg(test)]
use unhex;

pub fn num_to_base64(num: u8) -> String {
    match num {
        0 => "A".to_string(),
        1 => "B".to_string(),
        2 => "C".to_string(),
        3 => "D".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "G".to_string(),
        7 => "H".to_string(),
        8 => "I".to_string(),
        9 => "J".to_string(),
        10 => "K".to_string(),
        11 => "L".to_string(),
        12 => "M".to_string(),
        13 => "N".to_string(),
        14 => "O".to_string(),
        15 => "P".to_string(),
        16 => "Q".to_string(),
        17 => "R".to_string(),
        18 => "S".to_string(),
        19 => "T".to_string(),
        20 => "U".to_string(),
        21 => "V".to_string(),
        22 => "W".to_string(),
        23 => "X".to_string(),
        24 => "Y".to_string(),
        25 => "Z".to_string(),
        26 => "a".to_string(),
        27 => "b".to_string(),
        28 => "c".to_string(),
        29 => "d".to_string(),
        30 => "e".to_string(),
        31 => "f".to_string(),
        32 => "g".to_string(),
        33 => "h".to_string(),
        34 => "i".to_string(),
        35 => "j".to_string(),
        36 => "k".to_string(),
        37 => "l".to_string(),
        38 => "m".to_string(),
        39 => "n".to_string(),
        40 => "o".to_string(),
        41 => "p".to_string(),
        42 => "q".to_string(),
        43 => "r".to_string(),
        44 => "s".to_string(),
        45 => "t".to_string(),
        46 => "u".to_string(),
        47 => "v".to_string(),
        48 => "w".to_string(),
        49 => "x".to_string(),
        50 => "y".to_string(),
        51 => "z".to_string(),
        52 => "0".to_string(),
        53 => "1".to_string(),
        54 => "2".to_string(),
        55 => "3".to_string(),
        56 => "4".to_string(),
        57 => "5".to_string(),
        58 => "6".to_string(),
        59 => "7".to_string(),
        60 => "8".to_string(),
        61 => "9".to_string(),
        62 => "+".to_string(),
        63 => "/".to_string(),
        _ => "Ruh roh shaggy".to_string()
    }
}

pub fn big_to_base64(big : BigUint) -> String {
    let mut x = big;
    let mut out = String::from_str("");

    while x > 0u.to_biguint().unwrap() {
        let chr : u8 = x.rem(&64u.to_biguint().unwrap()).to_u8().unwrap();
        out.push_str( num_to_base64(chr).as_slice() );
        x = x >> 6;
    }

    return out.as_slice().chars().rev().collect();
}

#[test]
fn test_hex_to_base64() {
    let x = unhex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    assert!(big_to_base64(x) == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
}
