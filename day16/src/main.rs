use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
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
    posx: i64,
    posy: i64,
    d: Direction,
) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if (x == posx as usize) && (y == posy as usize) {
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
    // print_map(map, energized, x, y, dir);
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
fn trace_beam_and_count(
    map: &[Vec<char>],
    x: i64,
    y: i64,
    dir: Direction,
) -> usize{
    let mut energized: HashSet<(i64, i64, Direction)> = HashSet::new();
    trace_beam(&map, &mut energized, x, y, dir);
    energized
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<_>>().iter().count()

}
fn maxab(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        println!("{} {}", a, b);
        b
    }
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();
    let map = lines
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // for all sides of the map, count the number of energized points
    let mut max = 0;
    for x in 0..map[0].len() {
        max = maxab(max,trace_beam_and_count(&map, x as i64, 0, Direction::Down));
        max = maxab(max,trace_beam_and_count(&map, x as i64, map.len() as i64 -1, Direction::Up));
    }
    for y in 0..map.len() {
        max = maxab(max,trace_beam_and_count(&map, 0, y as i64, Direction::Right));
        max = maxab(max,trace_beam_and_count(&map, map[0].len() as i64 -1, y as i64, Direction::Left));
    }
    println!("{:?}", max);
}
