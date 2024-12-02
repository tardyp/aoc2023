use std::{fs::File, io::{BufReader, BufRead}};

fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        list1.push(digits[0]);
        list2.push(digits[1]);
    }
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }
    println!("Sum: {}", sum);
}

fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        list1.push(digits[0]);
        list2.push(digits[1]);
    }
    let mut sum = 0;
    for i in 0..list1.len() {
        let num = list1[i];
        let count = list2.iter().filter(|&x| *x == num).count() as i32;
        sum += num * count;
    }
    println!("Sum: {}", sum);
}

fn main() {
    main_2();
}