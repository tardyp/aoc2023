use itertools::Itertools;
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();
        let mut parts = sre.split(&line);
        let mut data = parts.next().unwrap().chars().collect_vec();
        let mut neighbors = parts.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
        let mut comb = vec![data.clone()];
        println!("{:?}", data);
        while comb[0].contains(&'?') {
            let pos = comb[0].iter().position(|&x| x == '?').unwrap();
            comb = comb.into_iter().flat_map(|x| {
                let mut x = x;
                x[pos] = '.';
                let mut y = x.clone();
                y[pos] = '#';
                vec![x, y]
            }).collect_vec();
        }
        sum += comb.into_iter().filter(|x| {
            let mut count = 0;
            let mut n = neighbors.clone();
            n.reverse();
            for (c,i) in x.iter().enumerate() {
                match *i {
                    '.' => {
                        if count == 0 {
                            continue;
                        }
                        let pop = n.pop().unwrap();
                        if pop != count {
                            return false;
                        }
                        count = 0;
                    }
                    '#' => {
                        if n.is_empty() {
                            return false;
                        }
                        count += 1;
                    }
                    _ => {}
                }
            }
            if n == [count] || n.is_empty() && count == 0 {
                return true;
            }
            false
        }).count();
        println!("\n\nsum: {}\n\n", sum);
    }
    println!("{}", sum);        
}