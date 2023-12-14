use core::{panic};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader}, time,
};
fn debug(s: &[bool]) -> String {
    s.iter()
        .map(|x| if *x { '#' } else { '.' })
        .collect::<String>()
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut table = vec![];
    for l in lines {
        let l = l.unwrap();
        table.push(l.chars().collect_vec());
    }
    //calculate time of loop
    let t1  = time::Instant::now();
    for i in 0..1000{
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            let mut y = y;
            if table[y][x] != 'O' {
                continue;
            }
            loop {
                if y == 0 {
                    break;
                }
                if table[y - 1][x] == '.' {
                    table[y - 1][x] = 'O';
                    table[y][x] = '.';
                    y -= 1;
                } else {
                    break;
                }
            }
        }
    }}
    let t2 = time::Instant::now();
    println!("{}s", (t2-t1).as_millis()*1000/3600);
    let mut sum = 0;
    for (i, l) in table.iter().enumerate() {
        let v = table.len()-i;
        let s =  l.iter().filter(|x| **x=='O').count();
        sum += v*s;
        println!("{} {s} {v}", l.iter().collect::<String>());
    }
    println!("{}", sum);
}
