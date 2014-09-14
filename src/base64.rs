use num::bigint::{ToBigUint, BigUint};
use std::num::Zero;
#[cfg(test)]
use Buff;

pub fn u8_to_base64(num: u8) -> Option<char> {
    match num {
        0 => Some('A'),
        1 => Some('B'),
        2 => Some('C'),
        3 => Some('D'),
        4 => Some('E'),
        5 => Some('F'),
        6 => Some('G'),
        7 => Some('H'),
        8 => Some('I'),
        9 => Some('J'),
        10 => Some('K'),
        11 => Some('L'),
        12 => Some('M'),
        13 => Some('N'),
        14 => Some('O'),
        15 => Some('P'),
        16 => Some('Q'),
        17 => Some('R'),
        18 => Some('S'),
        19 => Some('T'),
        20 => Some('U'),
        21 => Some('V'),
        22 => Some('W'),
        23 => Some('X'),
        24 => Some('Y'),
        25 => Some('Z'),
        26 => Some('a'),
        27 => Some('b'),
        28 => Some('c'),
        29 => Some('d'),
        30 => Some('e'),
        31 => Some('f'),
        32 => Some('g'),
        33 => Some('h'),
        34 => Some('i'),
        35 => Some('j'),
        36 => Some('k'),
        37 => Some('l'),
        38 => Some('m'),
        39 => Some('n'),
        40 => Some('o'),
        41 => Some('p'),
        42 => Some('q'),
        43 => Some('r'),
        44 => Some('s'),
        45 => Some('t'),
        46 => Some('u'),
        47 => Some('v'),
        48 => Some('w'),
        49 => Some('x'),
        50 => Some('y'),
        51 => Some('z'),
        52 => Some('0'),
        53 => Some('1'),
        54 => Some('2'),
        55 => Some('3'),
        56 => Some('4'),
        57 => Some('5'),
        58 => Some('6'),
        59 => Some('7'),
        60 => Some('8'),
        61 => Some('9'),
        62 => Some('+'),
        63 => Some('/'),
        _ => None
    }
}

pub fn base64_to_u8(c: char) -> Option<u8> {
    match c {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(2),
        'D' => Some(3),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(6),
        'H' => Some(7),
        'I' => Some(8),
        'J' => Some(9),
        'K' => Some(10),
        'L' => Some(11),
        'M' => Some(12),
        'N' => Some(13),
        'O' => Some(14),
        'P' => Some(15),
        'Q' => Some(16),
        'R' => Some(17),
        'S' => Some(18),
        'T' => Some(19),
        'U' => Some(20),
        'V' => Some(21),
        'W' => Some(22),
        'X' => Some(23),
        'Y' => Some(24),
        'Z' => Some(25),
        'a' => Some(26),
        'b' => Some(27),
        'c' => Some(28),
        'd' => Some(29),
        'e' => Some(30),
        'f' => Some(31),
        'g' => Some(32),
        'h' => Some(33),
        'i' => Some(34),
        'j' => Some(35),
        'k' => Some(36),
        'l' => Some(37),
        'm' => Some(38),
        'n' => Some(39),
        'o' => Some(40),
        'p' => Some(41),
        'q' => Some(42),
        'r' => Some(43),
        's' => Some(44),
        't' => Some(45),
        'u' => Some(46),
        'v' => Some(47),
        'w' => Some(48),
        'x' => Some(49),
        'y' => Some(50),
        'z' => Some(51),
        '0' => Some(52),
        '1' => Some(53),
        '2' => Some(54),
        '3' => Some(55),
        '4' => Some(56),
        '5' => Some(57),
        '6' => Some(58),
        '7' => Some(59),
        '8' => Some(60),
        '9' => Some(61),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

pub fn big_to_base64(big : BigUint) -> String {
    let mut x = big;
    let mut out = String::from_str("");
    let sixtyfour = 64u.to_biguint().unwrap();

    while x > Zero::zero() {
        let chr : u8 = x.rem(&sixtyfour).to_u8().unwrap();
        let s = match u8_to_base64(chr) {
            Some(e) => e,
            None => fail!("Couldn't convert {} to bases64", chr)
        };
        out.push_char(s);
        x = x >> 6;
    }

    return out.as_slice().chars().rev().collect();
}

pub fn chunk_to_bytes(s: Vec<char>) -> Vec<u8> {
    //Convert base64 chars into a vec of bytes, skipping invalid or padded base64 chars
    let mut nums = Vec::new();
    for i in s.iter().map(|x| base64_to_u8(*x)) {
        match i {
            Some(e) => nums.push(e),
            None => ()
        }
    }

    //Combine the base64 bytes into 8-bit bytes
    let mut out = Vec::new();
    for i in range(0, nums.len() - 1 ) {
        let x = 2*(i+1);
        let y = 2*(2-i);
        let elem = (nums[i] << x ) + (nums[i+1] >> y);
        out.push(elem);
    }
    return out;
}
    
#[test]
fn test_hex_to_base64() {
    let x = Buff::unhex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    assert_eq!(x.base64().as_slice(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

#[test]
fn test_base64_to_hex() {
    let buf = Buff::from_base64("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(buf.hex().as_slice(), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
}
