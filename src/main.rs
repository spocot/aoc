use std::env;

extern crate aoc;

use aoc::{aoc1,aoc2,aoc3,aoc4};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        match args[1].parse().unwrap() {
            1 => match args[2].parse().unwrap() {
                    1 => aoc1::part_1(),
                    2 => aoc1::part_2(),
                    _ => println!("Not a valid part!")},
            2 => match args[2].parse().unwrap() {
                    1 => aoc2::part_1(),
                    2 => aoc2::part_2(),
                    _ => println!("Not a valid part!")},
            3 => match args[2].parse().unwrap() {
                    1 => aoc3::part_1(),
                    2 => aoc3::part_2(),
                    _ => println!("Not a valid part!")},
            4 => match args[2].parse().unwrap() {
                    1 => aoc4::part_1(),
                    2 => aoc4::part_2(),
                    _ => println!("Not a valid part!")},
            _ => println!("Not a valid day!")
        };
    } else {
        println!("Must specify which day and part to test!");
    }
}