use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
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
             let (area, sides) = calculate_region(r, c, &lines, &mut visited, plant);
            total_price += area * sides;
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
    let mut region_cells = HashSet::new();
    let mut queue = VecDeque::new();
     queue.push_back((start_r, start_c));
    visited.insert((start_r, start_c));
    area += 1;
    region_cells.insert((start_r,start_c));

     while let Some((r, c)) = queue.pop_front() {
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
             for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

              if nr >= 0 && nr < height as isize && nc >= 0 && nc < width as isize {
                 let nr = nr as usize;
                 let nc = nc as usize;
                if lines[nr].chars().nth(nc).unwrap() == plant {
                    if !visited.contains(&(nr,nc)){
                            queue.push_back((nr,nc));
                            visited.insert((nr,nc));
                            area += 1;
                             region_cells.insert((nr,nc));
                       }
                    }
               }
            }
    }

    let mut edges = HashSet::new();
      for &(r, c) in &region_cells {
           let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (dr, dc) in directions {
               let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= height as isize || nc < 0 || nc >= width as isize {
                    edges.insert((r,c,(dr,dc)));
                    
                }else {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if lines[nr].chars().nth(nc).unwrap() != plant{
                       edges.insert((r,c,(dr,dc)));

                    }
                }
            }
        }


    let mut sides = 0;
    let mut visited_edges = HashSet::new();

    for &(r, c, (dr, dc)) in &edges {
        if visited_edges.contains(&(r, c, (dr, dc))) {
            continue;
        }
         sides+=1;
           let mut current_r = r as isize;
            let mut current_c = c as isize;
            let mut current_direction = (dr, dc);

          while edges.contains(&(current_r as usize, current_c as usize, current_direction)){
           visited_edges.insert((current_r as usize, current_c as usize, current_direction));
              current_r = current_r as isize + current_direction.0;
              current_c = current_c as isize + current_direction.1;

             let next_directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
               let mut next_direction_found = false;
               for (ndr, ndc) in next_directions {
                  if edges.contains(&(current_r as usize,current_c as usize,(ndr,ndc))) && (ndr != current_direction.0 || ndc != current_direction.1) && (ndr != -current_direction.0 || ndc != -current_direction.1){
                      current_direction = (ndr,ndc);
                      next_direction_found = true;
                      break;
                  }
                }

                if !next_direction_found {
                  break;
                }
        }
    }
     

    (area, sides)
}