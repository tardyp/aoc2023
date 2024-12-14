use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}
// parse input like
// Button A: X+58, Y+37
// Button B: X+22, Y+61
// Prize: X=3046, Y=4855

fn parse_input(input: &str) -> Vec<Machine> {
    let re1 = Regex::new(r"Button A: X\+([-\d]+), Y\+([-\d]+)").unwrap();
    let re2 = Regex::new(r"Button B: X\+([-\d]+), Y\+([-\d]+)").unwrap();
    let re3 = Regex::new(r"Prize: X=([-\d]+), Y=([-\d]+)").unwrap();
    let mut machines = Vec::new();
    let mut a_x = 0;
    let mut a_y = 0;
    let mut b_x = 0;
    let mut b_y = 0;
    let mut prize_x = 0;
    let mut prize_y = 0;

    for line in input.lines() {
        if let Some(caps) = re1.captures(line) {
            a_x = caps[1].parse().unwrap();
            a_y = caps[2].parse().unwrap();
        } else if let Some(caps) = re2.captures(line) {
            b_x = caps[1].parse().unwrap();
            b_y = caps[2].parse().unwrap();
        } else if let Some(caps) = re3.captures(line) {
            prize_x = 10000000000000 + caps[1].parse::<i64>().unwrap();
            prize_y = 10000000000000 + caps[2].parse::<i64>().unwrap();
            machines.push(Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            });
        }
    }
    machines
}

// Function to find the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// Extended Euclidean algorithm to find coefficients for Bézout's identity
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}


fn solve_machine(machine: &Machine) -> Option<i64> {
    let det = machine.a_x * machine.b_y - machine.b_x * machine.a_y;

    if det == 0 {
        return None; // No unique solution or infinite solutions
    }
    
    //solve the system using cramer's rule, 
    // a_x * A + b_x * B = prize_x
    // a_y * A + b_y * B = prize_y

    let a_coeff = (machine.prize_x * machine.b_y - machine.b_x * machine.prize_y) as f64/ det as f64;
    let b_coeff = (machine.a_x * machine.prize_y - machine.prize_x * machine.a_y) as f64/ det as f64;
    
    if a_coeff.fract() == 0.0 && b_coeff.fract() == 0.0 && a_coeff >= 0.0 && b_coeff >= 0.0{

        let a_presses = a_coeff as i64;
        let b_presses = b_coeff as i64;
        
        return Some(a_presses * 3 + b_presses);
    }

    None

}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let machines = parse_input(&input);
    println!("Machines: {:?}", machines);
    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in &machines {
        if let Some(tokens) = solve_machine(machine) {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }
    println!("Total Prizes Won: {}", prizes_won);
    println!("Minimum Tokens to Spend: {}", total_tokens);
}
