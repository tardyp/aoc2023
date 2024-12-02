use std::{fs::File, io::{BufReader, BufRead}};

fn main_1() {
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

fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum:u32 = 0;
    let searches = vec![("one", 1), ("1", 1), ("two", 2), ("2", 2), ("three", 3), ("3", 3), ("four", 4), ("4", 4), ("five", 5), ("5", 5), ("six", 6), ("6", 6), ("seven", 7), ("7", 7), ("eight", 8), ("8", 8), ("nine", 9), ("9", 9), ("zero", 0), ("0", 0)];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut matches:Vec<(usize, usize)> = Vec::new();
        for s in searches.iter(){
            if let Some(idx) = line.find(s.0) {
                matches.push((idx, s.1));
            }
        }
        matches.sort();
        let mut lmatches:Vec<(i32, usize)> = Vec::new();
        for s in searches.iter(){
            if let Some(idx) = line.rfind(s.0) {
                lmatches.push((-(idx as i32), s.1));
            }
        }
        lmatches.sort();
        let digit = matches[0].1.to_string() + &lmatches[0].1.to_string();
        println!("{:?}", digit);
        sum += digit.parse::<u32>().unwrap();
    }
    println!("Sum: {}", sum);
}

fn main() {
    main_2();
}