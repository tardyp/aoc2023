use std::{fs::File, io::{BufRead, BufReader}, result};

use itertools::Itertools;

// 190: 10 19
// 3267: 81 40 27
// 83: 17 5
// 156: 15 6
// 7290: 6 8 6 15
// 161011: 16 10 13
// 192: 17 8 14
// 21037: 9 7 18 13
// 292: 11 6 16 20
enum Operator {
    Add,
    Multiply,
}
fn search_result(result: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == result;
    }
    for i in 0..3 {
        let cur =
        match i {
            0 => {
                 numbers[0] + numbers[1]
            }
            1 => {
                numbers[0] * numbers[1]
            }
            2 => {
                format!("{}{}", numbers[0], numbers[1]).parse::<i64>().unwrap()
            }
            _ => {
                panic!("Invalid operator");
            }
        };
        if cur > result {
            continue;
        }
        let mut new_numbers = vec![cur];
        new_numbers.extend_from_slice(&numbers[2..]);
        if search_result(result, &new_numbers) {
            return true;
        }
    }
    return false;
}
fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut lines = BufReader::new(file).lines();
    while let Some(Ok(line)) = lines.next() {
        let (result, numbers) = line.split(": ").collect_tuple().unwrap();
        let result = result.parse::<i64>().unwrap();
        let numbers = numbers.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        println!("{} {:?}", result, numbers);
        if search_result(result, &numbers) {
            println!("Found: {} {:?}", result, numbers);
            sum += result;
        }
        // sum += result;
    }
    println!("Sum: {}", sum);
}

fn main() {
    main_1();
}