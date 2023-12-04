use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
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
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for number in numbers.iter_mut() {
        for x in number.x - 1..number.x2 + 1 {
            for y in number.y - 1..number.y + 2 {
                if x < 0 || y < 0 || x >= mx || y >= my {
                    continue;
                }
                let c = map[y as usize][x as usize];
                if c == '*' {
                    gears.entry((x, y)).or_insert(vec![]).push(number.value);
                }
            }
        }
    }
    for gear in gears.values() {
        if gear.len() == 2 {
            sum += gear[0] * gear[1];
        }
    }
    println!("Sum: {}", sum);
}
