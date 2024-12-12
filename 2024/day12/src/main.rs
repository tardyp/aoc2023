use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("ex.txt")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();

    let height = lines.len();
    let width = lines[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for r in 0..height {
        for c in 0..width {
            if visited.contains(&(r, c)) {
                continue;
            }
            let plant = lines[r].chars().nth(c).unwrap();
            let (area, perimeter) = calculate_region(r, c, &lines, &mut visited, plant);
            total_price += area * perimeter;
        }
    }

    println!("Total price: {}", total_price);
    Ok(())
}

fn calculate_region(
    start_r: usize,
    start_c: usize,
    lines: &Vec<String>,
    visited: &mut HashSet<(usize, usize)>,
    plant: char,
) -> (usize, usize) {
    let height = lines.len();
    let width = lines[0].len();
    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back((start_r, start_c));
    visited.insert((start_r, start_c));
    area += 1;

    while let Some((r, c)) = queue.pop_front() {
       
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nr >= height as isize || nc < 0 || nc >= width as isize {
                perimeter += 1; // Edge of map = part of the perimeter
                continue;
            }
            
             let nr = nr as usize;
             let nc = nc as usize;
            if lines[nr].chars().nth(nc).unwrap() != plant {
                perimeter += 1; // Adjacent cell is different plant = part of perimeter
                continue;
            }
             if !visited.contains(&(nr, nc)){
                 queue.push_back((nr, nc));
                 visited.insert((nr,nc));
                 area += 1;
             }
        }
    }

    (area, perimeter)
}