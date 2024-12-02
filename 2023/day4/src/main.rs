use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::{HashMap, HashSet, VecDeque},
};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum: i32 = 0;
    let sre = regex::Regex::new(r" +").unwrap();
    let mut winning_counts = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let (_, numbers) = line.split(":").collect_tuple().unwrap();
        let numbers = numbers.trim();
        let (winning, numbers) = numbers.split("|").collect_tuple().unwrap();
        let winning:HashSet<i32> = sre.split(winning.trim()).map(|x|x.parse::<i32>().unwrap()).collect();
        let numbers:HashSet<i32> = sre.split(numbers.trim()).map(|x|x.parse::<i32>().unwrap()).collect();
        winning_counts.push(numbers.intersection(&winning).count());
    }
    let mut cards: VecDeque<usize> = (0..winning_counts.len()).collect();
    while !cards.is_empty() {
        let card = cards.pop_front().unwrap();
        sum += 1;
        for i in card + 1..card + winning_counts[card]+1 {
            cards.push_back(i);
        }
    }
    println!("Sum: {}", sum);
}
