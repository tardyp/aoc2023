use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::thread;
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

fn print_grid(grid: &HashMap<Point, i32>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let count = grid.get(&Point { x, y }).unwrap_or(&0);
            print!("{}", if *count > 0 { "#" } else { " " });
        }
        println!();
    }
    println!("width: {}, height: {}", width, height);
    thread::sleep(std::time::Duration::from_millis(1000));
}
fn grid_to_png(grid: &HashMap<Point, i32>, width: i32, height: i32, filename: &str) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let count = grid.get(&Point { x: x as i32, y: y as i32 }).unwrap_or(&0);
        let pix = if *count > 0 {0} else {255u8};
        *pixel = image::Rgb([pix, pix, pix]);
    }
    imgbuf.save(filename).unwrap();
}
// check that one lines is filled vertically for at least half of the height
fn check_candidate(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
    for x in 0..width {
        let mut count = 0;
        for y in 0..height {
            if *grid.get(&Point { x, y }).unwrap_or(&0) > 0 {
                count += 1;
            }
        }
        if count > height / 4 {
            return true;
        }
    }
    false
}
// 1. Vertical Line Density:
// Checks if there's a column that has a high concentration of points. This would be a good candidate because tree tend to have one.
fn check_vertical_line_density(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
    for x in 0..width {
        let count = (0..height).filter(|y| grid.contains_key(&Point{x,y:*y})).count();
        if count as i32 > height / 3 {  // Threshold for "high density", needs to be tuned
            return true;
        }
    }
    false
}

// 2. Horizontal Line Density:
// Similar to vertical density, but for rows. Good for finding the base of a tree.
fn check_horizontal_line_density(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
    for y in 0..height {
      let count = (0..width).filter(|x| grid.contains_key(&Point{x:*x,y})).count();
        if count as i32 > width / 3 { // Threshold needs tuning
            return true;
        }
    }
    false
}


// 3. Overall Point Density:
// Check if the total number of occupied tile is reasonable for a Christmas tree.
fn check_overall_point_density(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
    let num_points = grid.len();
    let total_cells = width * height;
    let density = num_points as f64 / total_cells as f64;

    // a tree occupies some space, and a density too low or too high is not good
    density > 0.01 && density < 0.05
}


// 4. Center of Mass Check:
// Calculate the center of mass of all points, and see if it's close to the center of the grid.
fn check_center_of_mass(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
        if grid.is_empty() { return false;}
    let mut sum_x = 0;
    let mut sum_y = 0;
    let num_points = grid.len() as i32;

    for point in grid.keys() {
        sum_x += point.x;
        sum_y += point.y;
    }

    let center_x = sum_x as f64 / num_points as f64;
    let center_y = sum_y as f64 / num_points as f64;

    let mid_x = width as f64 / 2.0;
    let mid_y = height as f64 / 2.0;
    let distance = ((center_x - mid_x).powi(2) + (center_y - mid_y).powi(2)).sqrt();
    // a tree is close to the center, but not too much
    distance < (width as f64 / 3.0)
}

//5. Number of Points :
// a tree is supposed to have a certain number of robot positions.
fn check_number_of_points(grid: &HashMap<Point, i32>) -> bool {
    let num_points = grid.len();
    num_points > 15 && num_points < 25

}

//6. Check for Point in the Center
// a tree have point in it's center
fn check_center_point(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
    let mid_x = width / 2;
    let mid_y = height / 2;
    grid.contains_key(&Point{x:mid_x, y:mid_y})
}

//7. Check for a triangle
// a tree has a triangular shape, so the distribution of point should resemble a triangle.
fn check_triangle_points(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
      if grid.is_empty() {
        return false;
    }
    // Find bounding box of occupied points
    let mut min_x = width;
    let mut max_x = -1;
    let mut min_y = height;
    let mut max_y = -1;
    for point in grid.keys() {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    let bounding_box_width = max_x - min_x;
    let bounding_box_height = max_y - min_y;


    // Check if bounding box is more tall than wide and the width is not 0.
    if bounding_box_width == 0 {
        return false
    }
    if bounding_box_height as f32 > bounding_box_width as f32 * 1.5 {
       return true
    }
    false
}

// 8. Check for a bottom Heavy Shape
// A tree has its largest parts at the bottom
fn check_bottom_heavy_shape(grid: &HashMap<Point, i32>,  height: i32) -> bool {

    if grid.is_empty() {
        return false;
    }

    let mut bottom_count = 0;
    let mut top_count = 0;

    for point in grid.keys(){
         if point.y > height / 2{
              bottom_count += 1
         } else {
             top_count += 1
         }
    }
     bottom_count as f64 > (top_count as f64 * 2.0)

}

//9. Check if it is close to an inverted pyramid

fn check_inverted_pyramid_shape(grid: &HashMap<Point, i32>, width: i32, height: i32) -> bool {
  if grid.is_empty() {
        return false;
    }

  let mid_x = width / 2;
    let mid_y = height / 2;
  
    let mut top_count = 0;
    let mut middle_count = 0;
    let mut bottom_count = 0;

    for point in grid.keys() {
        let dist_from_center = ((point.x - mid_x).pow(2) + (point.y- mid_y).pow(2)) as f64;
          
           if dist_from_center < (height as f64 / 4.0).powi(2) {
                  top_count += 1;
            }else if dist_from_center < (height as f64 / 2.0).powi(2) {
              middle_count +=1;
          } else {
              bottom_count += 1
          }
    }
    // check that the number of points increase as we go down.
    if  bottom_count > middle_count && middle_count > top_count{
        return true
    }
   false

}

//10. check for number of single point
// a tree only has points in one position, not multiple robots in the same position

fn check_single_point(grid: &HashMap<Point, i32>) -> bool {
      for count in grid.values() {
         if *count != 1 {
              return false
         }
    }
    true

}
fn simulate_robots(robots: &Vec<Robot>, width: i32, height: i32, time: i32) {

    let mut grid: HashMap<Point, i32> = HashMap::new();
    let mut robots=robots.clone();
    for i in 1..time {
        let mut next_robot = Vec::new();
        for robot in robots {
            let mut current_pos = robot.pos;
            current_pos.x = (current_pos.x + robot.vel.x).rem_euclid(width);
            current_pos.y = (current_pos.y + robot.vel.y).rem_euclid(height);
            next_robot.push(Robot {
                pos: current_pos,
                vel: robot.vel,
            });
            *grid.entry(current_pos).or_insert(0) += 1;
        }
        if check_single_point(&grid) {
            grid_to_png(&grid, width, height, &format!("output_{i}.png"));
            println!("Time: {}", i);
        }
        grid.clear();
        robots = next_robot;
    }
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
    let time = 1000000;

    simulate_robots(&robots, width, height, time);
    Ok(())
}
