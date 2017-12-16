use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn part_1() {
    let f = File::open("input/5-1.txt").unwrap();
    let file = BufReader::new(&f);

    let mut steps = 0;

    // Convert file into a Vec of steps.
    let mut offsets: Vec<i32> = file.lines().flat_map(|l| {
        l.unwrap().trim().parse()
    }).collect();
    println!("{:?}", offsets);
    let mut curr_step: i32 = 0;
    loop {
        let next_step = offsets[curr_step as usize] + curr_step;
        offsets[curr_step as usize] += 1;
        curr_step = next_step;
        steps += 1;
        if next_step < 0 || next_step >= offsets.len() as i32 {
            break;
        }
    }
    println!("Number of steps: {}", steps);
}

pub fn part_2() {
    let f = File::open("input/5-2.txt").unwrap();
    let file = BufReader::new(&f);

    let mut steps = 0;

    // Convert file into a Vec of steps.
    let mut offsets: Vec<i32> = file.lines().flat_map(|l| {
        l.unwrap().trim().parse()
    }).collect();
    println!("{:?}", offsets);
    let mut curr_step: i32 = 0;
    loop {
        let offset = offsets[curr_step as usize];
        let next_step = offset + curr_step;
        offsets[curr_step as usize] +=
            if offset >= 3 {
                -1
            } else {
                1
            };
        
        curr_step = next_step;
        steps += 1;
        if next_step < 0 || next_step >= offsets.len() as i32 {
            break;
        }
    }
    println!("Number of steps: {}", steps);
}
