use std::fs;
use std::collections::HashMap;

fn num_digits(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn apply_rules_count(counts: &mut HashMap<u64, u64>) {
    let mut next_counts: HashMap<u64, u64> = HashMap::new();
    let mut max_value = 0;
    for (&value, &count) in counts.iter() {
            if count == 0 {
                continue;
            }
           if value == 0 {
                 *next_counts.entry(1).or_insert(0) += count;
                 continue;
           }

            let num_digs = num_digits(value);

           if num_digs % 2 == 0{
                 let mut pow10 = 1;
                for _ in 0..(num_digs/2){
                    pow10 *= 10;
                }

               *next_counts.entry(value / pow10).or_insert(0) += count;
               *next_counts.entry(value % pow10).or_insert(0) += count;
            } else{
                *next_counts.entry(value * 2024).or_insert(0) += count;
            }
             if value > max_value{
                max_value = value;
            }

    }
    *counts = next_counts;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut counts: HashMap<u64, u64> = HashMap::new();
        for num in stones{
            *counts.entry(num).or_insert(0) += 1;
        }
    for _ in 0..750 {
         apply_rules_count(&mut counts);
    }
        let mut total_count: u64 = 0;
    for (_key,value) in counts.iter(){
          total_count += value;
        }
    println!("{}", total_count);
}