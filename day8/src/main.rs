use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};
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
    let mut steps = lines.map(|line|{
        let line = line.unwrap();
        let (key, steps) = line.split(" = ").collect_tuple().unwrap();
        
        
        let steps = steps.replace("(", "").replace(")", "");
        let (left, right) = steps.split(", ").collect_tuple().unwrap();
        (key.to_string(), Step{left: left.to_string(), right: right.to_string()})
    }).collect::<HashMap<String, Step>>();
    let mut pos = 0;
    let mut key = "AAA".to_string();
    let mut num_steps = 0;
    loop {
        let dir = first_line.chars().nth(pos).unwrap();
        pos = (pos + 1) % first_line.len();
        let rl = steps.get(&key).unwrap();
        key = match dir {
            'R'=> rl.right.clone(),
            'L'=> rl.left.clone(),
            _ => panic!("Unknown direction"),
        };
        println!("{}: {}", num_steps, key);
        num_steps += 1;
        if key == "ZZZ" {
            break;
        }
    }
    println!("{:#?}", num_steps);
}
