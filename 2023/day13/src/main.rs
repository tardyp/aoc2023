use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
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
    let mut patterns = vec![vec![]];
    for l in lines {
        let l = l.unwrap();
        if l.is_empty() {
            patterns.push(vec![]);
        } else {
            patterns
                .last_mut()
                .unwrap()
                .push(l.chars().map(|x| x == '#').collect_vec());
        }
    }
    let h = find_horizontals(&patterns);
    let v = find_verticals(&patterns);
    println!("h: {h} v: {v} -> {}", v * 100 + h);
}
fn find_horizontal(pattern: &[Vec<bool>]) -> usize {
    for y in 0..pattern.len() {
        for x in 0..pattern[0].len() {
            let mut p = pattern.to_owned();
            p[y][x] = !p[y][x];
            println!("trying -> {x} {y}");
            let h = find_horizontal_(&p, x);
            // [.x...c...]
            // [...c...x]
            if h > 0 {
                println!("found smudge at {},{}, {h}, {}", x, y, pattern[0].len());
                return h;
            }
        }
    }
    0
}
fn find_horizontal_(pattern: &[Vec<bool>], must_include: usize) -> usize {
    let ln = pattern[0].len();
    let mut min = 1;
    let mut max = ln - 1;
    max = (ln + must_include) / 2;
    min = 1+(must_include) / 2;
    for c in min..max+1 {
        let mut found = true;
        for l in pattern {
            let (left, right) = if c <= ln / 2 {
                (&l[..c], &l[c..(c + c)])
            } else {
                (&l[(c + c - ln)..c], &l[c..])
            };
            assert_eq!(
                left.len(),
                right.len(),
                "c:{c} left: {:?}, right: {:?}",
                debug(left),
                debug(right)
            );
            let mut right = right.to_owned();
            right.reverse();
            println!("{} | {}, {c}", debug(left), debug(&right));
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
fn find_horizontals(patterns: &[Vec<Vec<bool>>]) -> usize {
    let mut sum = 0;
    for p in patterns {
        sum += find_horizontal(p);
    }
    sum
}
fn find_verticals(patterns: &[Vec<Vec<bool>>]) -> usize {
    let mut sum = 0;
    println!("verticals");
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
            println!("{}", debug(l));
        }
        println!();
        sum += find_horizontal(&t);
    }
    sum
}
