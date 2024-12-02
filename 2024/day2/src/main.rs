use std::{fs::File, io::{BufReader, BufRead}};


fn is_safe(v: &Vec<i32>) -> bool {
    let diffs: Vec<_> = v.windows(2).map(|w| w[0] - w[1]).collect();
    if diffs.iter().all(|&x| x >0 && x <= 3) || diffs.iter().all(|&x| x < 0 && x >= -3) {
        return true;
    }
    return false;
}
fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        sum += if is_safe(&digits) {1} else {0};
    }
    println!("Sum: {}", sum);
}

fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        if !is_safe(&digits){
            for i in 0..digits.len() {
                let mut new_digits = digits.clone();
                new_digits.remove(i);
                if is_safe(&new_digits) {
                    sum += 1;
                    break;
                }
            }
        } else {
            sum += 1;
        }
    }
    println!("Sum: {}", sum);
    // println!("Sum: {}", sum);
}

fn main() {
    main_2();
}