use core::panic;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    x: usize,
    y: usize,
    direction: usize,
    score: usize,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn print_map(maze: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (start_x, start_y) = find_start(&maze);
    let (end_x, end_y) = find_end(&maze);
    let mut all_pathes = HashSet::new();
    loop {
        let path = solve(&maze, start_x, start_y, end_x, end_y);
        for p in path.iter() {
            all_pathes.insert(*p);
        }
        println!("{}", all_pathes.len());
    }

}
fn solve(maze: &Vec<Vec<char>>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Vec<(usize, usize)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // East, South, West, North
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(State {
        x: start_x,
        y: start_y,
        direction: 0,
        score: 0,
        path: vec![(start_x, start_y)],
    });

    while let Some(State {
        x,
        y,
        direction,
        score,
        path,
    }) = heap.pop()
    {
        if (x, y) == (end_x, end_y) {
            return path;
        }

        match visited.entry((x, y, direction)) {
            Occupied(_) => {
                continue;
            }
            Vacant(entry) => {
                entry.insert(score);
            }
        }
        // println!("score: {} direction: {}", score, direction);
        // print_map(&maze, &path.iter().cloned().collect());

        // Move forward
        let (dx, dy) = directions[direction];
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        if maze[ny][nx] != '#' {
            let mut path = path.clone();
            path.push((nx, ny));
            heap.push(State {
                x: nx,
                y: ny,
                direction,
                score: score + 1,
                path,
            });
        }
        // the first time, turning is free \o/
        let next_score = if score == 0 { 0 } else { score + 1000 };

        // choose the next direction randomly XD
        if rand::random::<bool>() {
            // Turn clockwise
            let new_direction = (direction + 1) % 4;
            heap.push(State {
                x,
                y,
                direction: new_direction,
                score: next_score,
                path: path.clone(),
            });

            // Turn counterclockwise
            let new_direction = (direction + 3) % 4;
            heap.push(State {
                x,
                y,
                direction: new_direction,
                score: next_score,
                path,
            });
        } else {
            // Turn counterclockwise
            let new_direction = (direction + 3) % 4;
            heap.push(State {
                x,
                y,
                direction: new_direction,
                score: next_score,
                path: path.clone(),
            });
            // Turn clockwise
            let new_direction = (direction + 1) % 4;
            heap.push(State {
                x,
                y,
                direction: new_direction,
                score: next_score,
                path,
            });
            
        } 
    }
    panic!("no path found");
}

fn find_start(maze: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                return (x, y);
            }
        }
    }
    panic!("Start position not found");
}

fn find_end(maze: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == 'E' {
                return (x, y);
            }
        }
    }
    panic!("End position not found");
}
