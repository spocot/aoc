use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn part_1() {
    let f = File::open("input/2-1.txt").unwrap();
    let file = BufReader::new(&f);
    
    let mut spreadsheet: Vec<Vec<u32>> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        let sequence: Vec<u32> = l.split_whitespace().flat_map(|x| x.parse::<u32>()).collect();
        spreadsheet.push(sequence);
    }

    let sum = spreadsheet.iter().fold(0, |acc, x| {
        let max = x.iter().max().unwrap();
        let min = x.iter().min().unwrap();
        acc + max - min
    });

    println!("Answer is: {}", sum);
}

pub fn part_2() {
    let f = File::open("input/2-2.txt").unwrap();
    let file = BufReader::new(&f);
    
    let mut spreadsheet: Vec<Vec<u32>> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        let sequence: Vec<u32> = l.split_whitespace().flat_map(|x| x.parse::<u32>()).collect();
        spreadsheet.push(sequence);
    }

    let sum = spreadsheet.iter().fold(0, |acc, x| {
        let mut quotient = 0;
        for (i, num) in x.iter().enumerate() {
            for cmp in &x[i+1..] {
                if num % cmp == 0 {
                    quotient = num / cmp;
                    println!("{:?} - {} / {}", quotient, num, cmp);
                    break;
                } else if cmp % num == 0 {
                    quotient = cmp / num;
                    println!("{:?} - {} / {}", quotient, cmp, num);
                    break;
                } else {
                    continue;
                }
            }
        }
        acc + quotient
    });

    println!("Answer is: {}", sum);
}
