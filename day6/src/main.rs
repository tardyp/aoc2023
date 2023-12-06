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
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let line = lines.next().unwrap().unwrap();
    let times = line.split(":").last().unwrap().trim();
    let times = sre.split(times).map(|x| x.parse::<i64>().unwrap()).collect_vec();
    let line = lines.next().unwrap().unwrap();
    let distance = line.split(":").last().unwrap().trim();
    let distance = sre.split(distance).map(|x| x.parse::<i64>().unwrap()).collect_vec();
    let mut res = 1;
    for (t, d) in zip(times, distance) {
        let mut record = 0;
        for a in 0..t {
            let cd = a * (t-a);
            if cd > d {
                record +=1;
            }
            println!("{a} {t} {cd} {d}");
        }
        res *= record;
    }
    println!("{}", res);
}
