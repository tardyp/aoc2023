use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time,
};
fn debug(s: &[bool]) -> String {
    s.iter()
        .map(|x| if *x { '#' } else { '.' })
        .collect::<String>()
}
// rotate the table 90 degree clockwise
fn rotate(s: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = vec![];
    for i in 0..s[0].len() {
        let mut v = vec![];
        for j in (0..s.len()).rev() {
            v.push(s[j][i]);
        }
        res.push(v);
    }
    // print_table(&res);
    res
}
fn print_table(t: &[Vec<char>]) {
    for l in t {
        println!("{}", l.iter().collect::<String>());
    }
    println!();
}
fn detect_loop(s: &[usize]) -> Option<usize> {
    if s.len() < 50 {
        return None;
    }
    let mut i = s.len() - 10;
    let mut ln = 10;
    while i > s.len() / 2 {
        if s[i] == s[s.len() - 1] {
            let right = &s[i + 1..];
            let left = &s[(i - ln + 2)..i + 1];
            // println!("{:?}\n{:?} {ln} {i}",right, left);
            if left == right {
                return Some(i+1);
            }
        }
        i -= 1;
        ln += 1;
    }
    None
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut table = vec![];
    let tot_cycles = 1000000000;
    for l in lines {
        let l = l.unwrap();
        table.push(l.chars().collect_vec());
    }
    //calculate time of loop
    let t1 = time::Instant::now();
    let mut sums = vec![];
    let mut i = 0;
    while i < tot_cycles {
        for rotates in 0..4 {
            for _ in 0..rotates {
                table = rotate(&table);
            }
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
            }
            for _ in rotates..4 {
                table = rotate(&table);
            }
        }
        let mut sum = 0;
        // print_table(&table);
        for (i, l) in table.iter().enumerate() {
            let v = table.len() - i;
            let s = l.iter().filter(|x| **x == 'O').count();
            sum += v * s;
        }
        sums.push(sum);
        if let Some(lp) = detect_loop(&sums) {
            let ln = sums.len() - lp;
            println!(
                "loop after {} iterations {lp} {} {:?}",
                i,
                ln,
                sums
            );
            while i + ln < tot_cycles {
                i += ln;
            }
            let idx = tot_cycles - i-1;
            println!("{} {} {idx}",sums[lp-1],sums[lp]);
            println!("{}", sums[lp+idx-1]);
            break;
        }
        i += 1;
    }
//    println!("{:?}", sums);
}
