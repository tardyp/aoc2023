use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_input_file("input.txt").expect("Failed to read input file");
    let total_rating = calculate_total_trailhead_rating(&map);
    println!("The sum of the ratings of all trailheads is: {}", total_rating);
}

fn read_input_file(filename: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        map.push(row);
    }
    Ok(map)
}

fn calculate_total_trailhead_rating(map: &Vec<Vec<u8>>) -> u32 {
    let mut total_rating = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                total_rating += calculate_trailhead_rating(map, r, c);
            }
        }
    }
    total_rating
}

fn calculate_trailhead_rating(map: &Vec<Vec<u8>>, start_r: usize, start_c: usize) -> u32 {
    let num_rows = map.len();
    let num_cols = map[0].len();
    let mut path_count = 0;
    
    let mut stack = vec![(start_r, start_c, 0)];

    while let Some((r, c, current_height)) = stack.pop() {
        
        if map[r][c] == 9 {
            path_count += 1;
            continue;
        }

        let neighbors = get_valid_neighbors(map, r, c);
        for (nr, nc) in neighbors {
            if map[nr][nc] == current_height + 1 {
                stack.push((nr, nc, current_height+1));
            }
        }
    }
    path_count
}

fn get_valid_neighbors(map: &Vec<Vec<u8>>, r: usize, c: usize) -> Vec<(usize, usize)> {
    let num_rows = map.len();
    let num_cols = map[0].len();
    let mut neighbors = Vec::new();

    // Check up
    if r > 0 {
        neighbors.push((r - 1, c));
    }
    // Check down
    if r < num_rows - 1 {
        neighbors.push((r + 1, c));
    }
    // Check left
    if c > 0 {
        neighbors.push((r, c - 1));
    }
    // Check right
    if c < num_cols - 1 {
        neighbors.push((r, c + 1));
    }
    neighbors
}
