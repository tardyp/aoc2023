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
    v: i64
}
#[derive(Debug)]
struct VerticalLine {
    x: i64,
    y1: i64,
    y2: i64,
    d: Direction
}
#[cfg(debug_assertions)]
const FILENAME: &str = "ex.txt";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME).expect("File not found");
    let lines = BufReader::new(file).lines();
    let moves = lines.map(|x| {
        let line = x.unwrap();
        let mut line = line.split_whitespace();
        let _ = line.next().unwrap();
        let _ = line.next().unwrap();
        let v = line.next().unwrap();
        let d = match(v.chars().nth(7).unwrap()) {
            '3' => Direction::Up,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '0' => Direction::Right,
            e => panic!("bad direction {}", e)
        };
        let v = u64::from_str_radix(&v[2..7], 16).unwrap();
        Move {
            d,
            v:v as i64
        }
    }).collect_vec();
    println!("moves: {:?}", moves);
    let (min_x, min_y, max_x, max_y) = find_limits(&moves);
    let mut up_lines = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut ys = HashSet::new();
    ys.insert(y);
    for m in moves.iter() {
        let (dx, dy) = m.d.to_coord();
        let newx = x + dx * m.v;
        let newy = y + dy * m.v;
        match m.d{
            Direction::Up|
            Direction::Down => {
                up_lines.push(VerticalLine{x, y1:std::cmp::min(y, newy), y2:std::cmp::max(y, newy), d:m.d});
            },
            _ => {}
        }
        x = newx;
        y = newy;
        ys.insert(y-1);
        ys.insert(y);
        ys.insert(y+1);
    }
    up_lines.sort_by_key(|x| x.x);
    println!("up_lines: {:?}", up_lines);
    let mut ys = ys.into_iter().collect_vec();
    ys.sort();
    let mut sumsum = 0;
    println!("ys: {:?} {} {}", ys, min_y, max_y);
    for y in min_y..=max_y {
    // for y in ys.into_iter() {
        let up_lines = up_lines.iter().filter(|x| x.y1 <= y && y <= x.y2).collect_vec();
        // println!("y: {}, up_lines: {:?}", y, up_lines);
        let mut x = 0;
        let mut sum = 0;
        let mut last_d = Direction::Left;
        for l in up_lines.iter() {
            match l.d {
                Direction::Up => {
                    if l.y2 == y {
                        sum += l.x - x +1;
                    }

                },
                Direction::Down => {
                    sum += l.x - x +1;
                    if last_d == Direction::Down {
                        sum -= 2;
                    }
            },
                _ => panic!("bad direction")
            }
            x = l.x;
            last_d = l.d;
        }
        // println!("sum: {}", sum);
        sumsum += sum;
    }
    println!("sum: {} {}", sumsum, sumsum - 952408144115);
    let mut f = File::create("output.svg").unwrap();
    output_svg(&mut f, &moves);
}

fn output_svg(f: &mut File, moves: &Vec<Move>) {
    let (min_x, min_y, max_x, max_y) = find_limits(moves);

    println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut svg = format!(
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        0,
        0,
        width/1000,
        height/1000
    );
    let mut x = 0;
    let mut y = 0;
    // display moves as a svg path
    svg += "<path d=\"M ";
    for m in moves {
        let (dx, dy) = m.d.to_coord();
        x += dx * m.v;
        y += dy * m.v;
        svg += &format!("{} {} \n", (-min_x+x)/1000, (-min_y+y)/1000);
    }
    svg += "\" stroke=\"black\" stroke-width=\"1\" fill=\"black\" />";
    // display each move end as a circle
    for m in moves {
        let (dx, dy) = m.d.to_coord();
        x += dx * m.v;
        y += dy * m.v;
        svg += &format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"1\" fill=\"white\" />",
            (-min_x+x)/1000,
            (-min_y+y)/1000,
            1
        );
    }
    svg += "</svg>";
    f.write_all(svg.as_bytes()).unwrap();
}

fn find_limits(moves: &Vec<Move>) -> (i64, i64, i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for m in moves {
        let (dx, dy) = m.d.to_coord();
        x += dx * m.v;
        y += dy * m.v;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    (min_x, min_y, max_x, max_y)
}