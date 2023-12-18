use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use itertools::Itertools;
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
#[derive(Debug)]
struct Move {
    d: Direction,
    v: i64,
    color: [u8; 3],
}

// if compiled in release mode
// #[cfg(debug_assertions)]
// const FILENAME: &str = "ex.txt";
// #[cfg(not(debug_assertions))]
const FILENAME: &str = "input.txt";
// #[cfg(debug_assertions)]
// const SIZE : usize = 10;
// #[cfg(not(debug_assertions))]
const SIZE : usize = 2000;

fn main() {
    let file = File::open(FILENAME).expect("File not found");
    let lines = BufReader::new(file).lines();
    let moves = lines.map(|x| {
        let line = x.unwrap();
        // line can be "U 2 (#caa173)" turn into Map
        let mut line = line.split_whitespace();
        let d = match line.next().unwrap().chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("bad direction")
        };
        let v = line.next().unwrap();
        let v = v.parse::<i64>().unwrap();
        let color = line.next().unwrap();
        let color = color[2..color.len()-1].chars().chunks(2);
        let color = color.into_iter()
        .map(|x| 
            u8::from_str_radix(&x.into_iter().collect::<String>(), 16).unwrap() as u8
        ).collect::<Vec<u8>>();
        Move {
            d,
            v,
            color: [color[0], color[1], color[2]]
        }
    }).collect_vec();
    let mut map = [[0 as u8; SIZE]; SIZE];
    let mut x = (SIZE/2) as i64;
    let mut y = (SIZE/2) as i64;
    for m in moves.iter() {
        let (dx, dy) = m.d.to_coord();
        for _ in 0..m.v {
            x += dx;
            y += dy;
            map[y as usize][x as usize] = 1;
        }
    }
    // print_map(&map);
    color_map(&mut map, (SIZE/2 +1) as i64, (SIZE/2 +1) as i64);
    // print_map(&map);
    // print sum of map
    let mut sum:i64 = 0;
    let mut min_x = SIZE;
    let mut min_y = SIZE;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            sum += map[y][x] as i64;
            if map[y][x] == 1 {
                if x < min_x {
                    min_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    println!("{}", sum);
    println!("{} {} {} {} {} {}", min_x, min_y, max_x, max_y, max_x - min_x, max_y - min_y);
    // output the map into a ppm file
    let mut file = File::create("output.ppm").unwrap();
    let header = format!("P6\n{} {}\n255\n", max_x - min_x, max_y - min_y);
    file.write_all(header.as_bytes()).unwrap();
    for y in min_y..max_y {
        for x in min_x..max_x {
            if map[y][x] == 1 {
                file.write_all(&[0, 0, 0]).unwrap();
            } else {
                file.write_all(&[255, 255, 255]).unwrap();
            }
        }
    }
    
}

fn print_map(map: &[[u8; SIZE]; SIZE])  {    
    for y in 0..SIZE {
        for x in 0..SIZE {
            if map[y][x] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn color_map(map: &mut [[u8;SIZE]; SIZE], x: i64, y: i64) {
    if map[y as usize][x as usize] == 1 {
        return;
    }
    map[y as usize][x as usize] = 1;
    color_map(map, x-1, y);
    color_map(map, x+1, y);
    color_map(map, x, y-1);
    color_map(map, x, y+1);

}