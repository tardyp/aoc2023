use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn to_coord(&self) -> (i64, i64) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}
fn print_map(
    map: &[Vec<char>],
    energized: &HashSet<(i64, i64, Direction)>,
    X: i64,
    Y: i64,
    d: Direction,
) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if (x == X as usize) && (y == Y as usize) {
                //print direction with white background
                match d {
                    Direction::Left => print!("\x1b[47m\x1b[30m<\x1b[0m"),
                    Direction::Right => print!("\x1b[47m\x1b[30m>\x1b[0m"),
                    Direction::Up => print!("\x1b[47m\x1b[30m^\x1b[0m"),
                    Direction::Down => print!("\x1b[47m\x1b[30mv\x1b[0m"),
                }
                continue;
            }
            if energized.contains(&(x as i64, y as i64, Direction::Left))
                || energized.contains(&(x as i64, y as i64, Direction::Right))
                || energized.contains(&(x as i64, y as i64, Direction::Up))
                || energized.contains(&(x as i64, y as i64, Direction::Down))
            {
                print!("\x1b[31m{}\x1b[0m", c);
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!("---------------------");
}
fn trace_beam(
    map: &[Vec<char>],
    energized: &mut HashSet<(i64, i64, Direction)>,
    x: i64,
    y: i64,
    dir: Direction,
) {
    let mut x = x;
    let mut y = y;
    let mut dir = dir;
    print_map(map, energized, x, y, dir);
    while x >= 0 && y >= 0 && x < map[0].len() as i64 && y < map.len() as i64 {
        if !energized.insert((x, y, dir)){
            break;
        }
        let c = map[y as usize][x as usize];
        match (c, dir) {
            ('|', Direction::Left) | ('|', Direction::Right) => {
                trace_beam(map, energized, x, y, Direction::Up);
                dir = Direction::Down;
            }
            ('-', Direction::Up) | ('-', Direction::Down) => {
                trace_beam(map, energized, x, y, Direction::Left);
                dir = Direction::Right;
            }
            ('/', Direction::Left) => {
                dir = Direction::Down;
            }
            ('/', Direction::Right) => {
                dir = Direction::Up;
            }
            ('\\', Direction::Left) => {
                dir = Direction::Up;
            }
            ('\\', Direction::Right) => {
                dir = Direction::Down;
            }
            ('\\', Direction::Up) => {
                dir = Direction::Left;
            }
            ('\\', Direction::Down) => {
                dir = Direction::Right;
            }
            ('/', Direction::Up) => {
                dir = Direction::Right;
            }
            ('/', Direction::Down) => {
                dir = Direction::Left;
            }
            _ => {}
        }
        let (dx, dy) = dir.to_coord();
        x += dx;
        y += dy;
        
    }
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r"[-=]").unwrap();
    let lines = BufReader::new(file).lines();
    let map = lines
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut energized: HashSet<(i64, i64, Direction)> = HashSet::new();
    trace_beam(&map, &mut energized, 0, 0, Direction::Right);
    let energized = energized
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<_>>();

    println!("{:?}", energized.iter().count());
}
