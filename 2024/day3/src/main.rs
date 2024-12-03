use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    // regex that match like 'mul(2,4)'
    let mut sum = 0;
    let mut do_count = true;
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        println!("{}", line);
        for cap in re.captures_iter(&line) {
            if cap[0] == *"don't()" {
                println!("Don't");
                do_count = false;
                continue;
            } else if cap[0] == *"do()" {
                println!("Don");
                do_count = true;
                continue;
            }
            println!("{:?}", cap);
            if do_count == false {
                continue;
            }
            let a = cap[2].parse::<i32>().unwrap();
            let b = cap[3].parse::<i32>().unwrap();
            sum += a * b;
        }
    }
    println!("Sum: {}", sum);
}

fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    // println!("Sum: {}", sum);
}

fn main() {
    main_1();
}