use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum:u32 = 0;
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().chars().filter(
            |c| c.is_digit(10))
        .collect::<Vec<_>>();
        let digit = [digits[0],digits[digits.len() - 1]];
        sum += digit.iter().collect::<String>().parse::<u32>().unwrap();
    }
    println!("Sum: {}", sum);
}
