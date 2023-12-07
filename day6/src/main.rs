use itertools::{Itertools};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range, iter::zip,
};

#[derive(Clone, Debug)]
struct TranslationRange {
    pub translation: i64,
    pub source_range: Range<i64>,
}
#[derive(Clone, Debug)]
struct Map {
    pub name: String,
    pub ranges: Vec<TranslationRange>,
}

fn main() {
    let file = File::open("ex.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let line = lines.next().unwrap().unwrap();
    let t = line.split(":").last().unwrap().trim().replace(" ", "").parse::<i64>().unwrap();
    let line = lines.next().unwrap().unwrap();
    let d = line.split(":").last().unwrap().trim().replace(" ", "").parse::<i64>().unwrap();
    let mut record = 0;
    for a in 0..t {
        let cd = a * (t-a);
        if cd > d {
            record +=1;
        }
        // println!("{a} {t} {cd} {d}");
    }
    println!("{}", record);
}
