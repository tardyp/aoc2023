use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn look_for_xmas(data: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> i32 {
    let mut x = x as i32;
    let mut y = y as i32;
    let searched = "MAS".chars().collect::<Vec<_>>();
    let mut searched_slice = &searched[..];
    loop {
        x += dx;
        y += dy;
        if x < 0 || y < 0 || x >= data[0].len() as i32 || y >= data.len() as i32 {
            break;
        }
        if data[y as usize][x as usize] == searched_slice[0] {
            searched_slice = &searched_slice[1..];
            if searched_slice.len() == 0 {
                return 1;
            }
        } else {
            break;
        }
    }
    return 0;
}
fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut data = Vec::new();
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().chars().collect::<Vec<_>>();
        data.push(digits);
    }
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 'X' {
                sum += look_for_xmas(&data, x, y, 0, 1);
                sum += look_for_xmas(&data, x, y, 0, -1);
                sum += look_for_xmas(&data, x, y, 1, 0);
                sum += look_for_xmas(&data, x, y, 1, 1);
                sum += look_for_xmas(&data, x, y, 1, -1);
                sum += look_for_xmas(&data, x, y, -1, -1);
                sum += look_for_xmas(&data, x, y, -1, 1);
                sum += look_for_xmas(&data, x, y, -1, 0);

            }
        }
    }
    println!("Sum: {}", sum);
}

fn look_for(data: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32, c: char) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;
    x += dx;
    y += dy;
    if x < 0 || y < 0 || x >= data[0].len() as i32 || y >= data.len() as i32 {
        return false;
    }
    if data[y as usize][x as usize] == c {
        println!("{} at {}, {}", c, x, y);

        return true;
    }
    println!("no {} at {}, {}", c, x, y);
    return false;
}
fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut data = Vec::new();
    for line in BufReader::new(file).lines() {
        let digits = line.unwrap().chars().collect::<Vec<_>>();
        data.push(digits);
    }
    for y in 1..data.len()-1 {
        for x in 1..data[y].len()-1 {
            println!("{} {} {}", x, y, data[y][x]);
            if data[y][x] == 'A' {
                println!("found A at {}, {}", x, y);
                if ((look_for(&data, x, y, -1, -1, 'M') && look_for(&data, x, y, 1, 1, 'S')) || (look_for(&data, x, y, -1, -1, 'S') && look_for(&data, x, y, 1, 1, 'M')))&&
                ((look_for(&data, x, y, -1, 1, 'M') && look_for(&data, x, y, 1, -1, 'S')) || (look_for(&data, x, y, -1, 1, 'S') && look_for(&data, x, y, 1, -1, 'M'))) {
                    println!("found at {}, {}", x, y);
                    sum += 1;
                }

            }
        }
    }
    println!("Sum: {}", sum);
}

fn main() {
    main_2();
}