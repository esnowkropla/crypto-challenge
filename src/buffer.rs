use std::fmt;
use num::bigint::{BigUint, ToBigUint};
use std::num::{Zero, ToStrRadix};
use std::str::from_utf8;
use std::ops::BitXor;
use big_to_base64;
use chunk_to_bytes;

///Struct storing the buffer we're working with in LSB
#[deriving(Clone, Eq)]
pub struct Buff {
    pub contents : Vec<u8>,
}

impl<'a> Buff {
    pub fn new(n: uint) -> Buff {
        Buff::from_elem(n, 0u8)
    }

    pub fn from_elem(n: uint, elem: u8) -> Buff {
        Buff{contents: Vec::from_elem(n, elem)}
    }

    pub fn from_fn(n: uint, op: |uint| -> u8) -> Buff {
        Buff{contents: Vec::from_fn(n, op)}
    }

    pub fn from_slice(s: &str) -> Buff {
        let n = s.len();
        let mut vec = Vec::from_elem(n, 0u8);
        for (i, v) in s.chars().enumerate() {
            *vec.get_mut(n-i-1) = v as u8;
        }
        Buff{contents: vec.clone()}
    }

    pub fn from_base64(s: &str) -> Buff {
        //Length has to be a multiple of 4 or input was not padded properly
        assert_eq!(s.len() % 4, 0);
        let mut out = Vec::new();

        let mut it = s.chars().peekable();
        while it.peek().is_some() {
            let chunk : Vec<char> = it.by_ref().take(4).collect();
            let bytes = chunk_to_bytes(chunk);
            out.push_all_move(bytes);
        }
        out.reverse();
        return Buff{contents: out};
    }

    pub fn repeat(n: uint, s: &str) -> Buff {
        let mut buf = Buff::new(n);
        for (i, v) in s.chars().cycle().take(n).enumerate() {
            *buf.contents.get_mut(n - i -1) = v as u8;
        }
        return buf;
    }

    pub fn len(&self) -> uint {
        self.contents.len()
    }

    pub fn unhex(s: &str) -> Buff {
        let n = if s.len()%2 == 1 {(s.len()+1)/2} else {s.len()/2};//Get right num of bytes
        let mut buf = Buff::new(n);

        for (i, chr) in s.chars().rev().enumerate() {
            let num : u8 = chr.to_digit(16).unwrap() as u8;
            *buf.contents.get_mut(i/2) += num << (i%2)*4;
        }
        buf
    }
    
    pub fn hex(&self) -> String {
        let mut out = String::from_str("");
        for byte in self.contents.iter().rev() {
            out.push_str( format!("{}", fmt::radix(*byte, 16)).as_slice() );
        }
        return out;
    }

    pub fn bin(&self) -> String {
        self.to_big().to_str_radix(2)
    }

    pub fn base64(&self) -> String {
        return big_to_base64(self.to_big());
    }

    pub fn to_big(&self) -> BigUint {
        let mut acc : BigUint = Zero::zero();
        for (i, v) in self.contents.iter().enumerate() {
            acc = acc + (v.to_biguint().unwrap() << 8*i);
        }
        return acc;
    }

    pub fn utf8(&self) -> Option<String> {
        let mut out = self.contents.clone();
        out.reverse();
        let temp = from_utf8(out.as_slice());
        return match temp {
            Some(e) => Some(e.to_string()),
            None => None
        };
    }

    pub fn sub_buf(&'a self, a: uint, b: uint) ->  &'a [u8] {
        self.contents.slice(a,b)
    }
}

impl fmt::Show for Buff {
    ///Prints the decimal value of the buffer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_big())
    }
}

impl BitXor<Buff, Buff> for Buff {
    fn bitxor(&self, rhs: &Buff) -> Buff {
        assert_eq!(self.contents.len(), rhs.contents.len());
        let mut buf = Buff::new(self.contents.len());
        for (i, v) in self.contents.iter().enumerate() {
            *buf.contents.get_mut(i) = self.contents[i] ^ rhs.contents[i];
        }
        return buf;
    }
}

impl PartialEq for Buff {
    fn eq(&self, rhs: &Buff) -> bool {
        self.to_big() == rhs.to_big()
    }
}
