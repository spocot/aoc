use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn part_1() {
    let f = File::open("input/4-1.txt").unwrap();
    let file = BufReader::new(&f);

    let mut num_valid = 0;

    for line in file.lines() {
        let l = line.unwrap();

        let mut i = 0;

        let words: Vec<_> = l.split_whitespace().collect();
        if words.iter().all(|w| {
            i += 1;
            for word in &words[i..] {
                if w == word {
                    return false;
                }
            }
            true
        }){ num_valid += 1; }
    }

    println!("Num valid: {}", num_valid);
}

pub fn part_2() {
    let f = File::open("input/4-2.txt").unwrap();
    let file = BufReader::new(&f);

    let mut num_valid = 0;

    for line in file.lines() {
        let l = line.unwrap();

        let mut i = 0;

        let words: Vec<_> = l.split_whitespace().collect();
        if words.iter().all(|&w| {
            i += 1;
            let mut c1: Vec<_> = w.chars().collect();
            c1.sort();
            for &word in &words[i..] {
                let mut c2: Vec<_> = word.chars().collect();
                c2.sort();
                if c2 == c1 {
                    return false;
                }
            }
            true
        }){ num_valid += 1; }
    }

    println!("Num valid: {}", num_valid);
}
