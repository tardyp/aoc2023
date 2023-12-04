use core::num;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Number {
    pub value: i32,
    pub x: i32,
    pub x2: i32,
    pub y: i32,
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum: i32 = 0;
    let mut map = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        numbers.append(
            re.find_iter(&line)
                .map(|mat| Number {
                    value: mat.as_str().parse::<i32>().unwrap(),
                    x: mat.start() as i32,
                    x2: mat.end() as i32,
                    y: y as i32,
                })
                .collect::<Vec<_>>()
                .as_mut(),
        );

        let digits = line.chars().collect::<Vec<_>>();
        map.push(digits.clone());
    }
    let mx = map[0].len() as i32;
    let my = map.len() as i32;
    for number in numbers.iter_mut() {
        let mut is_adjacent = false;
        for x in number.x - 1..number.x2 + 1 {
            for y in number.y - 1..number.y + 2 {
                if x < 0 || y < 0 || x >= mx || y >= my {
                    continue;
                }
                let c = map[y as usize][x as usize];
                if c != '.' && !c.is_digit(10) {
                    is_adjacent = true;                    
                }
            }
        }
        if is_adjacent {
            sum += number.value;
        } else {
            println!("{:?}", number);
        }
    }
    println!("Sum: {}", sum);
}
