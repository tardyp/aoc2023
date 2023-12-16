use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
fn display_map(map: &Vec<Point>) {
    let mut min_x = map[0].x;
    let mut max_x = map[0].x;
    let mut min_y = map[0].y;
    let mut max_y = map[0].y;
    for p in map.iter() {
        if p.x < min_x {
            min_x = p.x
        }
        if p.x > max_x {
            max_x = p.x
        }
        if p.y < min_y {
            min_y = p.y
        }
        if p.y > max_y {
            max_y = p.y
        }
    }
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            for (i, p) in map.iter().enumerate() {
                if p.x == x && p.y == y {
                    print!("{}", i + 1);
                }
            }
            if !map.contains(&Point { x, y }) {
                print!(".");
            }
        }
        println!();
    }
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();
    let mut map = Vec::new();
    let mut cols = HashSet::new();
    let mut y = 0;
    for line in lines {
        let line = line.unwrap();
        let mut found = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                found = true;
                map.push(Point { x: x as i64, y: y as i64 });
                cols.insert(x);
            }
        }
        if !found {
            y += 1000000;
        } else {
            y += 1;
        }
    }
    let mut expand_xs = Vec::new();
    let mut expand_x = 0;
    for x in 0..*cols.iter().max().unwrap() + 1 {
        expand_xs.push(expand_x);
        if !cols.contains(&x) {
            expand_x += 1000000;
        }else {
            expand_x += 1
        }
    }
    for p in map.iter_mut() {
        p.x = expand_xs[p.x as usize];
    }
    let mut sum = 0;
    for i in 0..map.len() {
        let p1 = &map[i];
        let i = i+1;
        for j in i..map.len() {
            let p2 = &map[j];
            let j = j+1;
            let d1 = (p2.x - p1.x).abs() ;
            let d2= (p2.y - p1.y).abs();
            match (i, j) {
                (1, 7) |
                (3, 6) |
                (8, 9) => {
                println!("{:?} {:?} {i} {j} {d1} {d2} {}", p1, p2, d1+d2);
                }
                _=>()
            }
            sum += d1 + d2;
        }
    }
    println!("{:#?}", sum);
}
