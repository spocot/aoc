use std::io;

pub fn part_1() {
    println!("Input:");

    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Err(e) => panic!("{}", e),
        _ => {}
    };

    let num: i32 = input.trim().parse().unwrap();

    let mut dist = 0;

    if num > 1 {

        // Find size (width and height) of the spiral our number is on.
        let mut size = (num as f32).sqrt().ceil() as i32;

        // Adjust in case num was an in between even square.
        if size % 2 == 0 {
            size += 1;
        }

        let m_dist = size / 2;

        // Formula found by hand to mathematically calculate the Manhattan Distance.
        dist = (((num - (size - 2).pow(2) - 1) % (size - 1)) - (m_dist - 1)).abs() + m_dist;
    }

    println!("Distance: {}", dist);
}

pub fn part_2() {
}
