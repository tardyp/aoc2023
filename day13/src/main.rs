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
    let mut patterns = vec![vec![]];
    for l in lines {
        let l = l.unwrap();
        if l.is_empty() {
            patterns.push(vec![]);
        } else {
            patterns.last_mut().unwrap().push(l.chars().collect_vec());
        }
    }
    let h = find_horizontals(&patterns);
    let v = find_verticals(&patterns);
    println!("h: {h} v: {v} -> {}", v*100+h);
}

fn find_horizontal(pattern: &[Vec<char>]) -> usize {
    let ln = pattern[0].len();
    for c in 1..ln {
        let mut found = true;
        for l in pattern {
            let (left, right) = 
            if c <= ln/2 {
                (&l[..c],
                &l[c..(c+c)])
            } else {
                (&l[(c+c-ln)..c],
                &l[c..])
            };
            assert_eq!(left.len(), right.len(), "c:{c} left: {:?}, right: {:?}", left, right);
            let mut right = right.to_owned();
            right.reverse();
            println!("left: {:?}, right: {:?}, {c}", left, right);
            if left != right {
                found = false;
                break;
            } else {

            }
        }
        if found {
            return c;
        }
    }
    0
}
fn find_horizontals(patterns: &[Vec<Vec<char>>]) -> usize {
    let mut sum = 0;
    for p in patterns {
        sum += find_horizontal(p);
    }
    sum
}
fn find_verticals(patterns: &[Vec<Vec<char>>]) -> usize {
    let mut sum = 0;
    for p in patterns {
        // transpose p
        let mut t = vec![];
        for i in 0..p[0].len() {
            let mut l = vec![];
            for j in 0..p.len() {
                l.push(p[j][i]);
            }
            t.push(l);
        }
        for l in &t {
            println!("{:?}", l);
        }
        println!();
        sum += find_horizontal(&t);
    }
    sum
}
