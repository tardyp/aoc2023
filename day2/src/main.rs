use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        if let Some((id, subsets)) = line.unwrap().split(":").collect_tuple() {
            let id = id.split(" ").collect::<Vec<_>>()[1];
            let mut impossible = false;
            for subset in subsets.split(";") {
                let subset = subset.trim();
                for color in subset.split(",") {
                    let (num, color) = color.trim().split(" ").collect_tuple().unwrap();
                    let num = num.parse::<i32>().unwrap();
                    let max = match color {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => 0,
                    };
                    if num > max {
                        impossible = true;
                        break;
                    }
                }
            }
            if impossible {
                println!("impossible subset: {:?}", subsets);
            } else {
                sum += id.parse::<i32>().unwrap();
            }
        }
    }
    println!("sum: {}", sum);
}

fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        if let Some((id, subsets)) = line.unwrap().split(":").collect_tuple() {
            let id = id.split(" ").collect::<Vec<_>>()[1];
            let mut max = [0;3];
            for subset in subsets.split(";") {
                let subset = subset.trim();
                for color in subset.split(",") {
                    let (num, color) = color.trim().split(" ").collect_tuple().unwrap();
                    let num = num.parse::<i32>().unwrap();
                    let idx = match color {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!("invalid color"),
                    };
                    if num > max[idx] {
                        max[idx] = num;
                    }
                }
            }
            println!("max: {:?}", max);
            let power = max.iter().fold(1, |acc, x| acc * x);
            sum += power;
        }
    }
    println!("sum: {}", sum);
}

fn main() {
    main_2();
}