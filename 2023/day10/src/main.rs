use itertools::Itertools;
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    next: [(i32, i32); 2],
    c: char,
}
impl Node {
    fn next(&self, from: (i32, i32)) -> Option<(i32, i32)> {
        if self.next[0] == from {
            Some(self.next[1])
        } else if self.next[1] == from {
            Some(self.next[0])
        } else {
            None
        }
    }
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut nodes: HashMap<(i32, i32), Node> = HashMap::new();
    let mut y: i32 = 0;
    let mut start = (0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        let mut x: i32 = 0;
        let line = line.unwrap();
        for c in line.chars() {
            match c {
                '|' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x, y - 1), (x, y + 1)],
                            c,
                        },
                    );
                }
                '-' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x + 1, y)],
                            c,
                        },
                    );
                }
                'L' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x + 1, y), (x, y - 1)],
                            c,
                        },
                    );
                }
                'J' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x, y - 1)],
                            c,
                        },
                    );
                }
                '7' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x, y + 1)],
                            c,
                        },
                    );
                }
                'F' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x + 1, y), (x, y + 1)],
                            c,
                        },
                    );
                }
                '.' => {}
                'S' => {
                    start = (x, y);
                }
                _ => {
                    println!("Unknown char {}", c);
                }
            }
            x += 1;
        }
        max_x = x;
        y += 1;
    }
    max_y = y;
    // find the initial direction
    let around_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut all = vec![];
    let mut found = false;
    for d in around_dirs.iter() {
        let next = (start.0 + d.0, start.1 + d.1);
        if let Some(next_node) = nodes.get(&next) {
            println!("found next: {:?} {:?}", next, d);
            let mut sum = 0;
            let mut prev = start;
            let mut cur_node = next_node;
            let mut cur = next;
            all.clear();
            all.push(start.clone());
            loop {
                if let Some(next) = cur_node.next(prev) {
                    sum += 1;
                    all.push(cur.clone());
                    if next == start {
                        println!(
                            "Found a loop of length {} longest dist: {}",
                            sum,
                            (sum + 1) / 2
                        );
                        found = true;
                        break;
                    }
                    cur_node = nodes.get(&next).unwrap();
                    prev = cur;
                    cur = next;
                } else {
                    break;
                }
            }
        }
        if found {
            break;
        }
    }
    println!("all: {:?}", all);
    let allh = all.iter().collect::<HashSet<_>>();
    let mut tab = vec![];
    for y in 0..max_y {
        let mut line = vec![];
        for x in 0..max_x {
            if allh.contains(&(x, y)) {
                if let Some(node) = nodes.get(&(x, y)) {
                    line.push(match(node.c) {
                        'L' => '╚',
                        'J' => '╝',
                        '7' => '╗',
                        'F' => '╔',
                        '|' => '║',
                        '-' => '═',
                        _ => panic!("Unknown char {}", node.c),
                    });
                }
                else {
                    line.push('S');
                }
            } else {
                if (x,y) == start {
                    line.push('S');
                } else {
                    line.push('.');
                }
            }
        }
        tab.push(line);
    }
    for line in tab.iter() {
        println!("{}", line.iter().join(""));
    }
    for i in 0..all.len()-1 {
        let cur = all[i];
        let next = all[i+1];
        let diff = (next.0 - cur.0, next.1 - cur.1);
        let c:char = 'I';//((i %10) as u8 + ('0' as u8)) as char;
        let colored = match diff {
            (0, -1) => {
                if tab[cur.1 as usize][cur.0 as usize] == '╚' {
                    color(&mut tab, cur.0-1, cur.1+1, 'I');
                    color(&mut tab, cur.0, cur.1+1, 'I');
                }
                color(&mut tab, cur.0-1, cur.1, c)
            },
            (0, 1) => {
                if tab[cur.1 as usize][cur.0 as usize] == '╗' {
                    color(&mut tab, cur.0+1, cur.1-1, 'I');
                    color(&mut tab, cur.0, cur.1-1, 'I');
                }
                color(&mut tab, cur.0+1, cur.1, c)
            },
            (-1, 0) => {
                if tab[cur.1 as usize][cur.0 as usize] == '╝' {
                    color(&mut tab, cur.0+1, cur.1+1, 'I');
                    color(&mut tab, cur.0+1, cur.1, 'I');
                }
                color(&mut tab, cur.0, cur.1+1, c)
            },
            (1, 0) => {
                if tab[cur.1 as usize][cur.0 as usize] == '╔' {
                    color(&mut tab, cur.0-1, cur.1-1, 'I');
                    color(&mut tab, cur.0-1, cur.1, 'I');
                }
                color(&mut tab, cur.0, cur.1-1, c)
            },            _ => {panic!("Unknown diff {:?} {cur:?} {next:?} {i}", diff, );}
        };
        if colored {
            // tab[cur.1 as usize][cur.0 as usize] = c;
        }
    }
    let mut sum = 0;
    for line in tab.iter() {
        println!("{}", line.iter().join(""));
        sum += line.iter().filter(|c| **c == 'I').count();
    }
    println!("sum: {}", sum);
}

fn color(tab: &mut [Vec<char>], x: i32, y: i32, arg: char) -> bool{
    if x < 0 || y < 0 {
        return false;
    }
    if y as usize >= tab.len() || x as usize >= tab[y as usize].len() {
        return false;
    }
    if tab[y as usize][x as usize] == '.' {
        tab[y as usize][x as usize] = arg;
        color(tab, x+1, y, arg);
        color(tab, x-1, y, arg);
        color(tab, x, y+1, arg);
        color(tab, x, y-1, arg);
        return true;
    }
    false
}
