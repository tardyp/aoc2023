use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use num::integer::lcm as getlcm;
#[derive(Debug)]
struct Step {
    left: String,
    right: String,
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let mut steps = lines
        .map(|line| {
            let line = line.unwrap();
            let (key, steps) = line.split(" = ").collect_tuple().unwrap();

            let steps = steps.replace("(", "").replace(")", "");
            let (left, right) = steps.split(", ").collect_tuple().unwrap();
            (
                key.to_string(),
                Step {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            )
        })
        .collect::<HashMap<String, Step>>();
    let mut pos = 0;
    let mut keys = steps
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.to_string())
        .collect::<Vec<_>>();
    let mut lcm:i64 = 1;
    for key in &keys {
        let mut num_steps = 0;
        let mut key = key.clone();
        loop {
            let dir = first_line.chars().nth(pos).unwrap();
            pos = (pos + 1) % first_line.len();
            num_steps += 1;
            let rl = steps.get(&key).unwrap();
            key = match dir {
                'R' => rl.right.clone(),
                'L' => rl.left.clone(),
                _ => panic!("Unknown direction"),
            };
            if key.ends_with("Z") {
                lcm = getlcm(lcm, num_steps);
                println!("{num_steps}, {lcm}");
                break;
            }
        }
    }
}
