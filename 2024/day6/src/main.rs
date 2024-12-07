use std::{fs::File, io::{BufReader, BufRead}};

// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...
// map example
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }
    fn to_bit(&self) -> u8 {
        match self {
            Direction::Up => 0b0001,
            Direction::Down => 0b0010,
            Direction::Left => 0b0100,
            Direction::Right => 0b1000
        }
    }
    fn to_dx_dy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>'
        }
    }
}
fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut lines = BufReader::new(file).lines();
    let mut map = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        map.push(line.chars().collect::<Vec<char>>());
    }
    let mut x = 0;
    let mut y = 0;
    // find the guardian (the ^)
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                x = j as i32;
                y = i as i32;
            }
        }
    }
    let mut direction = Direction::Up;
    let mut visited = vec![vec![' '; map[0].len()]; map.len()];
    visited[y as usize][x as usize] = direction.to_char();
    loop {
        let (dx, dy) = direction.to_dx_dy();

        let next_x = x + dx;
        let next_y = y + dy;
        if next_x < 0 || next_x >= map[0].len() as i32 || next_y < 0 || next_y >= map.len() as i32 {
            break;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            direction = direction.turn_right();
            
        } else {
            x = next_x;
            y = next_y;
            visited[y as usize][x as usize] = direction.to_char();
            //clear terminal
            // std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    print_visited_map(&map, &visited);
    // count visited
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited[i][j] != ' ' {
                sum += 1;
            }
        }
    }
    println!("Sum: {}", sum);
}

fn is_loop(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let (mut x, mut y) = (x, y);
    let mut direction = Direction::Up;
    let mut visited = vec![vec![0u8; map[0].len()]; map.len()];
    visited[y as usize][x as usize] = direction.to_bit();
    loop {
        let (dx, dy) = direction.to_dx_dy();

        let next_x = x + dx;
        let next_y = y + dy;
        if next_x < 0 || next_x >= map[0].len() as i32 || next_y < 0 || next_y >= map.len() as i32 {
            return false;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            direction = direction.turn_right();
            
        } else {
            x = next_x;
            y = next_y;
            let bit = direction.to_bit();
            if visited[y as usize][x as usize] & bit != 0 {
                return true;
            }
            visited[y as usize][x as usize] |= bit;
        }
    }
}
fn main_2() {
    let file = File::open("input.txt").expect("File not found");
    let mut sum = 0;
    let mut lines = BufReader::new(file).lines();
    let mut map = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        map.push(line.chars().collect::<Vec<char>>());
    }
    let mut x = 0;
    let mut y = 0;
    // find the guardian (the ^)
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                x = j as i32;
                y = i as i32;
            }
        }
    }
    for i in 0..map.len() {
        for j in 0..map[i].len() {

            if map[i][j] != '#' {
                map[i][j] = '#';
                if is_loop(&map, x as i32, y as i32) {
                    sum += 1;
                }
                map[i][j] = '.';
            } 
        }
    }
    println!("Sum: {}", sum);
}
fn print_visited_map(map: &Vec<Vec<char>>, visited: &Vec<Vec<char>>) {
    print!("\x1B[2J");
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                print!("#");
                assert_eq!(visited[i][j] , ' ');
            } else {
            print!("{}", visited[i][j]);
            }
        }
        println!();
    }
    }


fn main() {
    main_2();
}