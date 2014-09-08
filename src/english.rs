use std::io::{File, BufferedReader};
use std::collections::hashmap::HashSet;
use std::cmp::min;

pub fn english_dict() -> HashSet<String> {
    let path = Path::new("/usr/share/dict/british-english");
    let mut file = BufferedReader::new(File::open(&path));
    let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    let mut dict = HashSet::new();

    for line in lines.iter() {
        if line.len() > 2 {
            let mut temp = line.clone();
            let n = temp.len();//Stripping the newline off:
            temp.truncate(n - 1);//rust ain't got a lot of string manip yet
            dict.insert(temp);
        }
    }
    return dict;
}

fn lower_letter(letter: char) -> char {
    match letter {
        'A' => 'a',
        'B' => 'b',
        'C' => 'c',
        'D' => 'd',
        'E' => 'e',
        'F' => 'f',
        'G' => 'g',
        'H' => 'h',
        'I' => 'i',
        'J' => 'j',
        'K' => 'k',
        'L' => 'l',
        'M' => 'm',
        'N' => 'n',
        'O' => 'o',
        'P' => 'p',
        'Q' => 'q',
        'R' => 'r',
        'S' => 's',
        'T' => 't',
        'U' => 'u',
        'V' => 'v',
        'W' => 'w',
        'X' => 'x',
        'Y' => 'y',
        'Z' => 'z',
        _ => letter
    }
}

pub fn lowercase(string: &str) -> String {
    string.chars().map(lower_letter).collect()
}

#[test]
fn test_lower() {
    assert_eq!(lowercase("TESTING!"), "testing!".to_string());
}

pub fn levenshtein(s: &str, t: &str) -> uint {
    let mut v0 : Vec<uint> = Vec::from_fn(t.len()+1, |x| x);
    let mut v1 : Vec<uint> = Vec::from_elem(t.len()+1, 0);

    for (i, v) in s.chars().enumerate() {
        *v1.get_mut(0) = i + 1;

        for (j, w) in t.chars().enumerate() {
            let cost = if v == w {0u} else {1};
            *v1.get_mut(j + 1) = min(v1[j] + 1, min(v0[j + 1] + 1, v0[j] + cost));
        }

        v0 = v1.clone();
    }
    return v1[t.len()];
}

#[test]
fn test_levenshtein() {
    assert_eq!(levenshtein("etaoinshrdlcumwfgypbvkjxqz", "etaoinshrdlcumwfgypbvkjxqz"), 0); 
    assert_eq!(levenshtein("etaoinshrdlcumwfgypbvkjxqz", "etaoinshrdlcumwfgypbvkjxq"), 1); 
    assert_eq!(levenshtein("etaoinshrdlcumwfgypbvkjxqz", "abcdefghijklmnopqrstuvwxyz"), 22); 
    assert_eq!(levenshtein("etaoinshrdlcumwfgypbvkjxqz", ""), 26); 
}

pub fn count(slice: &str, chr: char) -> uint {
    slice.chars().filter(|x| *x == chr).count()
}

#[test]
fn test_count() {
    assert_eq!(count("a testing string", 'i'), 2);
}

pub fn score_fn(to_score: &str) -> uint {
    let etaoin = " etaoinshrdlcumwfgypbvkjxqz";
    let mut alpha : Vec<char> = "abcdefghijklmnopqrstuvwxyz ".chars().collect();
    let subject = lowercase(to_score);
    
    //This section adds non-alphabetic characters from subject to alpha, to penalize strings 
    //with lots of gibberish
    for letter in subject.as_slice().chars() {
        if !alpha.contains(&letter) {
            alpha.push(letter);
        }
    }

    alpha.sort_by(|a, b| count(subject.as_slice(), *b).cmp(&count(subject.as_slice(), *a)));
    let alpha_str = String::from_chars(alpha.as_slice());
    return levenshtein(alpha_str.as_slice(), etaoin);
}

#[test]
fn test_score() {
    assert_eq!(score_fn("Cooking MC's like a pound of bacon"), 23);
}
