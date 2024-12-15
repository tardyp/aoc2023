
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
                    '#' => (Cell::Wall, Cell::Wall),
                    '.' => (Cell::Empty, Cell::Empty),
                    'O' => (Cell::BoxLeft, Cell::BoxRight),
                    '@' => {
                        robot_row = r;
                        robot_col = c*2;
                        (Cell::Robot, Cell::Empty)
                    }
                    _ => panic!("Invalid char in grid: {}", ch),
                };
                row.push(cell.0);
                row.push(cell.1);

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
    fn try_move_box_horizontal(&mut self, r: isize, c: isize, dc: isize) -> bool {
        // this one, we just need to shift the box to the right or left
        // its just that there are two kind of boxes, left and right
        // but they will shift together
        let new_box_row = r as usize;
        let mut new_box_col = (c + dc) as usize;

        loop{
            match self.grid[new_box_row][new_box_col]{
                Cell::Wall => return false,
                Cell::Empty => {
                    // shift all boxes to the right or left
                    while new_box_col != c as usize{
                        let prev_box_col = (new_box_col as isize - dc) as usize;
                        self.grid[new_box_row][new_box_col] = self.grid[new_box_row][prev_box_col];
                        new_box_col = prev_box_col;
                    }
                    return true;
                }
                Cell::BoxLeft | Cell::BoxRight => {
                    new_box_col = (new_box_col as isize + dc) as usize;
                }
                Cell::Robot => panic!("Unexpected robot encountered"),
            }
        }
    }
    fn try_move_box_vertical(&mut self, r: isize, c: isize, dr: isize, doit: bool) -> bool {
        // this one is more tricky as each pair of boxes will move together
        // so we will help need to recursively move the boxes one by one, handling all the cases
        let nr = (r+dr) as usize;
        let c = if self.grid[r as usize][c as usize] == Cell::BoxLeft {c} else {c-1};
        let nc = c as usize;
        // check if the box can move
        let shall_move = match (self.grid[nr][nc], self.grid[nr][nc+1]) {
            (Cell::Empty, Cell::Empty) => true,
            (Cell::BoxLeft, Cell::BoxRight) => {
                self.try_move_box_vertical(nr as isize, c, dr, doit)
            }
            (Cell::BoxRight, Cell::BoxLeft) => {
                self.try_move_box_vertical(nr as isize, c, dr, doit) && 
                self.try_move_box_vertical(nr as isize, c+1, dr, doit)
            }
            (Cell::BoxRight, Cell::Empty) => {
                self.try_move_box_vertical(nr as isize, c, dr, doit)
            }
            (Cell::Empty, Cell::BoxLeft) => {
                self.try_move_box_vertical(nr as isize, c+1, dr, doit)
            }

            _ => false,
        };
        if !shall_move {
            return false;
        }
        if doit {
            self.grid[nr][nc] = self.grid[r as usize][nc];
            self.grid[nr][nc+1] = self.grid[r as usize][nc+1];
            self.grid[r as usize][c as usize] = Cell::Empty;
            self.grid[r as usize][(c+1) as usize] = Cell::Empty;
        }
        return true;

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
            Cell::BoxLeft | Cell::BoxRight => {
                if direction == '>' || direction == '<' {
                    if self.try_move_box_horizontal(new_robot_row as isize, new_robot_col as isize, dc) {
                        self.grid[self.robot_row][self.robot_col] = Cell::Empty;
                        self.grid[new_robot_row][new_robot_col] = Cell::Robot;
                        self.robot_row = new_robot_row;
                        self.robot_col = new_robot_col;
                    }
                } else {
                    if self.try_move_box_vertical(new_robot_row as isize, new_robot_col as isize, dr, false) {
                        // redo the recursion, but this time we will actually move the boxes
                        self.try_move_box_vertical(new_robot_row as isize, new_robot_col as isize, dr, true);
                        self.grid[self.robot_row][self.robot_col] = Cell::Empty;
                        self.grid[new_robot_row][new_robot_col] = Cell::Robot;
                        self.robot_row = new_robot_row;
                        self.robot_col = new_robot_col;
                    }
                }
            }
            Cell::Robot => panic!("Unexpected robot encountered"),
        }
    }


    fn calculate_gps_sum(&self) -> usize {
        let mut sum = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid[r][c] == Cell::BoxLeft {
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
                    Cell::BoxLeft => '[',
                    Cell::BoxRight => ']',
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