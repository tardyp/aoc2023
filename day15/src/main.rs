use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time,
};
fn hash(s: &str) -> u64 {
    let mut h = 0;
    for c in s.chars() {
        h = h + c as u64;
        h *= 17;
        h &= 0xff;
    }
    h
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let data = BufReader::new(file).lines().next().unwrap().unwrap();
    let data = data.split(",").collect_vec();
    println!("{:?}", data);
    let mut sum = 0;
    for d in data{
        let h = hash(d);
        sum += h;
        println!("{} {} {}", d, h, sum);
    }
}
