use std::{fs::File, io::{BufReader, BufRead}};

use rand::seq::SliceRandom;

fn check_ordering_rule(digits: &Vec<i32>, ordering_rule: &Vec<Vec<i32>>) -> bool {
    for rule in ordering_rule {
        let mut found_second = false;
        let first = rule[0];
        let second = rule[1];
        for digit in digits.iter() {
            if *digit == first {
                if found_second {
                    return false;
                }
                break;
            } else if *digit == second {
                found_second = true;
            }
        }
    }
    true
}

fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut ordering_rule: Vec<_> = Vec::new();
    let mut lines = BufReader::new(file).lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let digits = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        ordering_rule.push(digits);
    }
    while let Some(Ok(line)) = lines.next() {
        let digits = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if check_ordering_rule(&digits, &ordering_rule) {
            // find number in the middle of the list
            let middle = digits[digits.len() / 2];
            sum += middle;
        }
    }
    println!("Sum: {}", sum);
}

fn check_ordering_rule_idx(digits: &Vec<i32>, ordering_rule: &Vec<Vec<i32>>) -> Option<usize> {
    // suffle rules with random crate
    let mut ordering_rule = ordering_rule.clone();
    ordering_rule.shuffle(&mut rand::thread_rng());

    for rule in ordering_rule {
        let mut found_second = false;
        let first = rule[0];
        let second = rule[1];
        for (idx, digit) in digits.iter().enumerate() {
            if *digit == first {
                if found_second {
                    return Some(idx);
                }
                break;
            } else if *digit == second {
                found_second = true;
            }
        }
    }
    None
}
fn sort_according_to_rule(digits: &Vec<i32>, ordering_rule: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sorted_digits = digits.clone();
    // implement a bubble sort
    while let Some(idx) = check_ordering_rule_idx(&sorted_digits, &ordering_rule) {
        sorted_digits.swap(idx-1, idx);
        println!("{:?} {}", sorted_digits, idx);
    }
    sorted_digits
}
fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut ordering_rule: Vec<_> = Vec::new();
    let mut lines = BufReader::new(file).lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let digits = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        ordering_rule.push(digits);
    }
    while let Some(Ok(line)) = lines.next() {
        let digits = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if !check_ordering_rule(&digits, &ordering_rule) {
            let digits = sort_according_to_rule(&digits, &ordering_rule);
            // find number in the middle of the list
            let middle = digits[digits.len() / 2];
            sum += middle;
        }
    }
    println!("Sum: {}", sum);
}

fn main() {
    main_2();
}