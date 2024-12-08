use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, result};

use itertools::Itertools;

// ............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............
fn add_antinode(map: &mut Vec<Vec<char>>, x: i32, y: i32) -> Option<(i32, i32)> {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        return None;
    }
    if map[y as usize][x as usize] == '.' {
        map[y as usize][x as usize] = '#';
    }
    return Some((x, y));
}
trait MaybeAdd {
    fn maybe_add(&mut self, maybe: Option<(i32, i32)>);
}
impl MaybeAdd for HashSet<(i32, i32)> {
    fn maybe_add(&mut self, maybe: Option<(i32, i32)>) {
        if let Some((x, y)) = maybe {
            self.insert((x, y));
        }
    }
}
fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut found = HashSet::new();
    let mut lines = BufReader::new(file).lines();
    let mut map = vec![];
    while let Some(Ok(line)) = lines.next() {
        map.push(line.chars().collect::<Vec<char>>());
    }
    // walk the map to find nodes != '.'
    for (y, x) in (0..map.len()).cartesian_product(0..map[0].len()) {
        if map[y][x] == '.' || map[y][x] == '#' {
            continue;
        }
        for (y2, x2) in (0..map.len()).cartesian_product(0..map[0].len()) {
            if y == y2 && x == x2 {
                continue;
            }
            if map[y2][x2] != map[y][x] {
                continue;
            }
            let dx = x2 as i32 - x as i32;
            let dy = y2 as i32 - y as i32;
            for i in -100..100 {
                if let Some((x, y)) = add_antinode(&mut map, x as i32 + dx * i, y as i32 + dy * i) {
                    found.insert((x, y));
                } else {
                    continue;
                }
            }
        }
    }
    print_map(&map);
    println!("Sum: {}", found.len());
}

fn main() {
    main_1();
}