use itertools::Itertools;
use std::{
    collections::HashMap,
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
    for line in lines {
        let mut x: i32 = 0;
        let line = line.unwrap();
        for c in line.chars() {
            match c {
                '|' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x, y - 1), (x, y + 1)],c
                        },
                    );
                }
                '-' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x + 1, y)],c
                        },
                    );
                }
                'L' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x + 1, y), (x, y - 1)],c
                        },
                    );
                }
                'J' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x, y - 1)],c
                        },
                    );
                }
                '7' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x - 1, y), (x, y + 1)],c
                        },
                    );
                }
                'F' => {
                    nodes.insert(
                        (x, y),
                        Node {
                            next: [(x + 1, y), (x, y + 1)],c
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
        y += 1;
    }
    // find the initial direction
    let around_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for d in around_dirs.iter() {
        let next = (start.0 + d.0, start.1 + d.1);
        if let Some(next_node) = nodes.get(&next) {
            println!("found next: {:?} {:?}", next, d);
            let mut sum = 0;
            let mut prev = start;
            let mut cur_node = next_node;
            let mut cur =  next;
            loop {
                if let Some(next) = cur_node.next(prev) {
                    sum += 1;
                    if next == start {
                        println!("Found a loop of length {} longest dist: {}", sum, (sum+1)/2);
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
    }
}
