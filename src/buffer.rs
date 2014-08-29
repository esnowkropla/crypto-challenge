use std::fmt;
use num::bigint::{BigUint, ToBigUint};
use std::num::{Zero};
use std::str::from_utf8;
use std::ops::BitXor;
use big_to_base64;

///Struct storing the buffer we're working with in LSB
#[deriving(Clone)]
pub struct Buffer {
    pub contents : Vec<u8>,
}

impl Buffer {
    pub fn new(n: uint) -> Buffer {
        Buffer::from_elem(n, 0u8)
    }

    pub fn from_elem(n: uint, elem: u8) -> Buffer {
        Buffer{contents: Vec::from_elem(n, elem)}
    }
    pub fn unhex(s: &str) -> Buffer {
        let n = if s.len()%2 == 1 {(s.len()+1)/2} else {s.len()/2};//Get right num of bytes
        let mut buf = Buffer::new(n);

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

    pub fn utf8(&self) -> String {
        let mut out = self.contents.clone();
        out.reverse();
        return from_utf8(out.as_slice()).unwrap().to_string();
    }
}

impl fmt::Show for Buffer {
    ///Prints the decimal value of the buffer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_big())
    }
}

impl BitXor<Buffer, Buffer> for Buffer {
    fn bitxor(&self, rhs: &Buffer) -> Buffer {
        assert_eq!(self.contents.len(), rhs.contents.len());
        let mut buf = Buffer::new(self.contents.len());
        for (i, v) in self.contents.iter().enumerate() {
            *buf.contents.get_mut(i) = self.contents[i] ^ rhs.contents[i];
        }
        return buf;
    }
}
