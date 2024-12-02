use std::{
    cmp::min as smin,
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet, i64::MAX,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}
impl Direction {
    fn to_coord(&self) -> (i64, i64) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::None => (0, 0),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapPoint {
    weight: i64,
    min_left: Option<i64>,
    min_right: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map{
    map: Vec<Vec<MapPoint>>,
    min: i64,
}
impl Map {
    fn x_len(&self) -> usize {
        self.map[0].len()
    }
    fn y_len(&self) -> usize {
        self.map.len()
    }
}
// if compiled in release mode
#[cfg(debug_assertions)]
const FILENAME: &str = "ex.txt";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME).expect("File not found");
    let lines = BufReader::new(file).lines();
    let map = lines
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| MapPoint{weight: x as i64 - '0' as i64, min_left: None, min_right: None})
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = Map{map, min: MAX};
    let init_weight = map.map[0][0].weight;
    let m = min_path(
        &mut map,
        &vec![],
        HashSet::new(),
        0,
        0,
        Direction::None,
        -init_weight
    );
    println!("{:?}", map.min);
}
fn print_map(map: &Map, xx: usize, yy: usize) {
    // #[cfg(not(debug_assertions))]
    // return;
    // print!("\x1b[2J");
    for y in 0..map.y_len() {
        for x in 0..map.x_len() {
            if xx == x && yy == y {
                // print with grey background
                print!("\x1b[48;5;8m");
            }
            print!(
                "{:4}",map.map[y][x].weight 
            );
            if xx == x && yy == y {
                // reset background
                print!("\x1b[0m");
            }
        }
        println!();
        for x in 0..map.x_len() {
            print!(
                "{}",
                match map.map[y][x].min_left {
                    Some(x) => 
                        format!("{:4}", x),
                    None => "    ".to_string(),
                }
            );
        }
        println!();
        for x in 0..map.x_len() {
            print!(
                "{}",
                match map.map[y][x].min_right {
                    Some(x) => 
                        format!("{:4}", x),
                    None => "    ".to_string(),
                }
            );
        }
        println!();
    }
    println!("x: {} y: {} min: {}", xx, yy, map.min);
    // // stop until enter is pressed
    // let mut s = String::new();
    // std::io::stdin().read_line(&mut s).unwrap();

}
// recursive algorithm to find the minimum path
// only works in --release mode or it will exaust the stack
fn min_path(
    map: &mut Map,
    h: &Vec<Direction>,
    ps: HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    d: Direction,
    cur: i64,
) {
    let (dx, dy) = d.to_coord();
    let x = x as i64 + dx;
    let y = y as i64 + dy;
    if x < 0 || y < 0 || x >= map.x_len() as i64 || y >= map.y_len() as i64 {
        return ;
    }
    // if we already went further than the known minimum, we can stop
    if cur >= map.min {
        return;
    }
    let mut h = h.clone();
    // prevent 4 times the same direction
    h.push(d);
    if h.len() > 4 && h[h.len() - 4..] == [d; 4] {
        return;
    }
    let x = x as usize;
    let y = y as usize;

    let mut ps = ps.clone();
    if !ps.insert((x, y)) {
        return;
    }    
    let x_len = map.x_len();
    let y_len = map.y_len();
    let p = &mut map.map[y][x];
    let cur = p.weight + cur;
    
    // we already went there from a shorter path
    if p.min_left.is_some_and(|x|x +1< cur) {
        // print_map(map, x, y);
        return;
    }
    p.min_left = Some(cur);
    // // we already went there and found the shortest path
    // if let Some(min_right) = p.min_right {
    //     // println!("x: {} y: {}", x, y);
    //     assert!(min_right > p.min_left.unwrap());
    //     let full = cur + min_right - p.min_left.unwrap();
    //     println!("{:?} {} {} {} {}", min, full, cur, min_right, p.min_left.unwrap());
    //     p.min_left = Some(cur);
    //     p.min_right = Some(full);
    //     print_map(map, x, y);
    //     return full;
    // }
    // p.min_left = Some(cur);
    if x == x_len - 1 && y == y_len - 1 {
        // run history in reverse
        let mut x:i64 = x as i64;
        let mut y:i64 = y as i64;
        for d in h.iter().rev() {
            let (dx, dy) = d.to_coord();
            x = x - dx;
            y = y - dy;
            // println!("{} {}", x, y);
            let p = &mut map.map[y as usize][x as usize];
            p.min_right = Some(cur);
            
        }
        // println!("{:?}", h);
        // println!("{:?}", ps);
        map.min = smin(map.min, cur);
        // print_map(map, x as usize, y as usize);

        return;
    }
    let mut candidates = [ Direction::Down, Direction::Right, Direction::Up, Direction::Left]
        .iter().map(|d| {
            let (dx, dy) = d.to_coord();
            let x = x as i64 + dx;
            let y = y as i64 + dy;
            if x < 0 || y < 0 || x >= map.x_len() as i64 || y >= map.y_len() as i64 {
                return (i64::MAX, d);
            }
            let x = x as usize;
            let y = y as usize;
            let p = &map.map[y][x];
            (p.weight, d)
        }).collect::<Vec<_>>();
    candidates.sort();
    if h.len() < 4{
    println!("{:?} {:?} {}", h, candidates, map.min);
    }
    for (_, d) in candidates.into_iter() {
        min_path(map, &h, ps.clone(), x, y, *d, cur);
    }
    // if map.min < MAX {
    //     println!("{}", map.min);
    // }
}
