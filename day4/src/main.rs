use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::{HashMap, HashSet},
};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum: i32 = 0;
    let sre = regex::Regex::new(r" +").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, numbers) = line.split(":").collect_tuple().unwrap();
        let numbers = numbers.trim();
        let (winning, numbers) = numbers.split("|").collect_tuple().unwrap();
        let winning:HashSet<i32> = sre.split(winning.trim()).map(|x|x.parse::<i32>().unwrap()).collect();
        let numbers:HashSet<i32> = sre.split(numbers.trim()).map(|x|x.parse::<i32>().unwrap()).collect();
        let winning_count = numbers.intersection(&winning).count();
        println!("Winning count: {}", winning_count);
        if winning_count == 0 {
            continue;
        }
        sum += 1<<(winning_count-1);
    }
    println!("Sum: {}", sum);
}
