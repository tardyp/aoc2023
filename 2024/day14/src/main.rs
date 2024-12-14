use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
#[derive(Debug, Clone, Copy,Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_robot(line: &str) -> Option<Robot> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
        return None;
    }

    let pos_part = parts[0];
    let vel_part = parts[1];

    let pos_coords: Vec<&str> = pos_part[2..].split(",").collect();
    let vel_coords: Vec<&str> = vel_part[2..].split(",").collect();

    if pos_coords.len() != 2 || vel_coords.len() != 2 {
        return None;
    }

    let pos_x = pos_coords[0].parse::<i32>().ok()?;
    let pos_y = pos_coords[1].parse::<i32>().ok()?;
    let vel_x = vel_coords[0].parse::<i32>().ok()?;
    let vel_y = vel_coords[1].parse::<i32>().ok()?;


    Some(Robot {
        pos: Point { x: pos_x, y: pos_y },
        vel: Point { x: vel_x, y: vel_y },
    })
}


fn simulate_robots(robots: &Vec<Robot>, width: i32, height: i32, time: i32) -> HashMap<Point, i32> {
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for robot in robots {
        let mut current_pos = robot.pos;
        for _ in 0..time {
            current_pos.x = (current_pos.x + robot.vel.x).rem_euclid(width);
            current_pos.y = (current_pos.y + robot.vel.y).rem_euclid(height);
        }

       *grid.entry(current_pos).or_insert(0) += 1;
    }
    grid
}

fn count_robots_in_quadrants(grid: &HashMap<Point, i32>, width: i32, height: i32) -> Vec<i32> {
    let mut quadrant_counts = vec![0, 0, 0, 0];

    let mid_x = width / 2;
    let mid_y = height / 2;

    for (pos, count) in grid {
        if pos.x == mid_x || pos.y == mid_y {
             continue;
        }

        let quadrant_index = if pos.x < mid_x {
            if pos.y < mid_y {
                0
            } else {
                2
            }
        } else {
            if pos.y < mid_y {
                1
            } else {
                3
            }
        };

        quadrant_counts[quadrant_index] += count;
    }
    quadrant_counts
}
fn main() -> io::Result<()> {
    let mut robots = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                if let Some(robot) = parse_robot(&line){
                    robots.push(robot);
                }
            }
        }
    }

    let width = 101;
    let height = 103;
    let time = 100;

    let final_grid = simulate_robots(&robots, width, height, time);
    let quadrant_counts = count_robots_in_quadrants(&final_grid, width, height);

    let safety_factor: i32 = quadrant_counts.iter().product();
    println!("Safety Factor: {}", safety_factor);
    Ok(())
}
