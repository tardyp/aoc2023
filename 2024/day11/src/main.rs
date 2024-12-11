use std::fs;

fn apply_rules(stones: &Vec<u64>) -> Vec<u64> {
    let mut next_stones = Vec::new();
    for &stone in stones {
        if stone == 0 {
            next_stones.push(1);
        } else {
             let num_str = stone.to_string();
            if num_str.len() % 2 == 0 {
                let mid = num_str.len() / 2;
                 let left = num_str[0..mid].parse::<u64>().unwrap();
                 let right = num_str[mid..].parse::<u64>().unwrap();
                next_stones.push(left);
                next_stones.push(right);
            } else {
               next_stones.push(stone * 2024);
            }
        }
    }
    next_stones
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = std::time::Instant::now();
    let initial_stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut current_stones = initial_stones;
    for i in 0..25 {
        current_stones = apply_rules(&current_stones);
        println!("{} {} {:?}",start.elapsed().as_secs_f32(), i, current_stones);

    }

    println!("{}", current_stones.len());
}