use std::io;

pub fn part_1() {
    println!("Captcha to solve: ");

    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Err(e) => panic!("{}", e),
        _ => {}
    };

    let mut sequence: Vec<u32> = input.chars().flat_map(|x| x.to_digit(10)).collect();

    // Append first number to end so we can check wraparound.
    let first = sequence[0];
    sequence.push(first);

    // Fold over each number, comparing to previous and adding it if its the same.
    let sum = sequence.iter().fold((0,0), |acc, &x|{
        let mut sum = acc.1;
        if x == acc.0 {
            sum += x;
        }
        (x, sum)
    }).1;

    println!("Answer is: {}", sum);
}

pub fn part_2() {
    println!("Captcha to solve: ");

    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Err(e) => panic!("{}", e),
        _ => {}
    };

    let sequence: Vec<u32> = input.chars().flat_map(|x| x.to_digit(10)).collect();

    let half_dist: usize = sequence.len() / 2;

    let mut sum = 0;

    for (i,&c) in sequence.iter().enumerate() {
        let mut check_index = i + half_dist;
        if check_index >= sequence.len() {
            check_index -= sequence.len();
        }
        if c == sequence[check_index] {
            sum += c;
        }
    }

    println!("Answer is: {}", sum);
}
