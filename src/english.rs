use std::io::{File, BufferedReader};
use std::collections::hashmap::HashSet;

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
