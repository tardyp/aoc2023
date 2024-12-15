use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: Vec<Vec<Cell>>,
    robot_row: usize,
    robot_col: usize,
    rows: usize,
    cols: usize,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut robot_row = 0;
        let mut robot_col = 0;
        let mut rows = 0;

        for (r, line) in input.lines().enumerate() {
            rows += 1;
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '@' => {
                        robot_row = r;
                        robot_col = c;
                        Cell::Robot
                    }
                    _ => panic!("Invalid char in grid: {}", ch),
                };
                row.push(cell);
            }
            grid.push(row);
        }

        let cols = grid[0].len();

        Warehouse {
            grid,
            robot_row,
            robot_col,
            rows,
            cols,
        }
    }

    fn move_robot(&mut self, direction: char) {
        let (dr, dc) = match direction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid direction: {}", direction),
        };

        let new_robot_row = (self.robot_row as isize + dr) as usize;
        let new_robot_col = (self.robot_col as isize + dc) as usize;

        if new_robot_row >= self.rows || new_robot_col >= self.cols {
            return; // Stay still
        }

        match self.grid[new_robot_row][new_robot_col] {
            Cell::Wall => {
                //Stay Still
            }
            Cell::Empty => {
                self.grid[self.robot_row][self.robot_col] = Cell::Empty;
                self.grid[new_robot_row][new_robot_col] = Cell::Robot;
                self.robot_row = new_robot_row;
                self.robot_col = new_robot_col;
            }
            Cell::Box => {
                let mut new_box_row = (new_robot_row as isize + dr) as usize;
                let mut new_box_col = (new_robot_col as isize + dc) as usize;

                if new_box_row >= self.rows || new_box_col >= self.cols {
                    return; // Stay still
                }
                while self.grid[new_box_row][new_box_col] == Cell::Box{
                    new_box_row = (new_box_row as isize + dr) as usize;
                    new_box_col = (new_box_col as isize + dc) as usize;
                    if new_box_row >= self.rows || new_box_col >= self.cols {
                        return; // Stay still
                    }
                }
                match self.grid[new_box_row][new_box_col] {
                    Cell::Wall => {
                        //Stay Still
                    }
                    Cell::Box => {
                        panic!("Unexpected box encountered");
                    }
                    Cell::Empty => {
                        self.grid[self.robot_row][self.robot_col] = Cell::Empty;
                        self.grid[new_robot_row][new_robot_col] = Cell::Robot;
                        self.grid[new_box_row][new_box_col] = Cell::Box;
                        self.robot_row = new_robot_row;
                        self.robot_col = new_robot_col;
                    }
                    Cell::Robot => panic!("Unexpected robot encountered"),
                }
            }
            Cell::Robot => panic!("Unexpected robot encountered"),
        }
    }


    fn calculate_gps_sum(&self) -> usize {
        let mut sum = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == Cell::Box {
                    sum += 100 * r + c;
                }
            }
        }
        sum
    }
    fn print(&self) {
        for row in &self.grid {
            for cell in row {
                let ch = match cell {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Box => 'O',
                    Cell::Robot => '@',
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let mut parts = input.split("\n\n");

    let warehouse_str = parts.next().unwrap();
    let moves_str = parts.next().unwrap();

    let moves = moves_str.chars().filter(|&c| c != '\n').collect::<String>();

    let mut warehouse = Warehouse::new(warehouse_str);

    for move_char in moves.chars() {
        println!("Move {}:", move_char);
        warehouse.move_robot(move_char);
        warehouse.print();
    }
    
    let gps_sum = warehouse.calculate_gps_sum();
    println!("{}", gps_sum);
}