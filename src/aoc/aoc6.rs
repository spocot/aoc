use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Eq, PartialEq)]
struct Configuration {
    blocks: String
}

impl Configuration {
    pub fn new(blocks: &[u32]) -> Self {

        let mut block_str: String = "".to_string();
        for b in blocks {
            block_str.push_str(&b.to_string());
        }

        Configuration {
            blocks: block_str
        }
    }
}

pub fn part_1() {
    let f = File::open("input/6-1.txt").unwrap();
    let file = BufReader::new(&f);

    // Convert file into a Vec of blocks.
    let mut blocks: Vec<u32> = file.lines().flat_map(|l| {
        l.unwrap().trim().parse()
    }).collect();

    // This will store all the block configurations we've seen before.
    let mut configs: Vec<Configuration> = Vec::new();

    // Keep track of the number of times we've looped.
    let mut count = 0;

    loop {
        println!("{:?}", blocks);
        // Current config
        let c = Configuration::new(blocks.as_slice());

        // Check if we've seen it before.
        if configs.contains(&c) {
            break;
        }

        // Add current config to our list.
        configs.push(c);

        // Find first occurence of max value.
        let mut max = 0;
        let mut index = 0;
        for (i,&b) in blocks.iter().enumerate() {
            if b > max {
                max = b;
                index = i;
            }
        }
        blocks[index] = 0;
        // Redistribute blocks.
        while max > 0 {
            index += 1;
            if index >= blocks.len() {
                index = 0;
            }
            blocks[index] += 1;
            max -= 1;
        }

        count += 1;
    }

    println!("Iterations: {}", count);
}
    

pub fn part_2() {
    let f = File::open("input/6-2.txt").unwrap();
    let file = BufReader::new(&f);

    // Convert file into a Vec of blocks.
    let mut blocks: Vec<u32> = file.lines().flat_map(|l| {
        l.unwrap().trim().parse()
    }).collect();

    // This will store all the block configurations we've seen before.
    let mut configs: Vec<Configuration> = Vec::new();

    // Keep track of the number of times we've looped.
    let mut count = 0;

    loop {
        println!("{:?}", blocks);
        // Current config
        let c = Configuration::new(blocks.as_slice());

        // Check if we've seen it before.
        if configs.contains(&c) {
            // If we have print the number of times we've looped since we last saw it.
            let pos = configs.iter().position(|x| *x == c).unwrap();
            println!("Iteration Diff: {:?}", count - pos);
            break;
        }

        // Add current config to our list.
        configs.push(c);

        // Find first occurence of max value.
        let mut max = 0;
        let mut index = 0;
        for (i,&b) in blocks.iter().enumerate() {
            if b > max {
                max = b;
                index = i;
            }
        }
        blocks[index] = 0;
        // Redistribute blocks.
        while max > 0 {
            index += 1;
            if index >= blocks.len() {
                index = 0;
            }
            blocks[index] += 1;
            max -= 1;
        }

        count += 1;
    }
}
