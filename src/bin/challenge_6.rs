extern crate crypto;

use crypto::{Buff, hamming, read_file, single_xor};

fn transpose(buf: &Buff, size: uint) -> Vec<Buff> {
    let mut vec : Vec<Buff> = Vec::from_elem(size, Buff{contents: Vec::new()});
    for (i,v) in buf.contents.iter().rev().enumerate() {
        vec.get_mut(i%size).contents.push(*v);
    }
    return vec;
}

fn find_keysize(buf: &Buff) -> (uint, f64) {
    let mut prob_size : uint = 0;
    let mut prob_norm : f64 = 99999f64;    
    for keysize in range(1,41) {
        let mut norms = Vec::new();

        for i in range(0,20) {
            for j in range(i+1, i+20) {
                let sub_a = buf.sub_buf(i*keysize, (i+1)*keysize);
                let sub_b = buf.sub_buf(j*keysize, (j+1)*keysize);
                let norm = hamming(sub_a, sub_b) as f64 / keysize as f64;
                norms.push(norm);
            }
        }
        let norm = norms.iter().fold(0f64, |a, &b| a+b )/(norms.len() as f64);
        if norm < prob_norm {
            prob_norm = norm;
            prob_size = keysize;
        }
    }
    return (prob_size, prob_norm);
}

fn main() {
    let file = read_file("src/assets/6.txt");
    let mut cipher = String::from_str("");
    for line in file.iter() {
        cipher.push_str(line.as_slice());
    }

    let buf = Buff::from_base64(cipher.as_slice());
    let (keysize, _) = find_keysize(&buf);

    let mut keys = Vec::new();
    let blocks = transpose(&buf, keysize);
    for block in blocks.iter() {
        let (key, _, _) = single_xor(block);
        keys.push(key);
    }

    let key = match String::from_utf8(keys) {
        Ok(t) => t,
        Err(e) => fail!("Couldn't make key string out of {}", e)
    };
    println!("{}", key);
    let key_buf = Buff::repeat(buf.len(), key.as_slice());
    let plaintext = key_buf ^ buf;
    match plaintext.utf8() {
        Some(e) => println!("{}", e),
        None => println!("Couldn't decode")
    }
}
