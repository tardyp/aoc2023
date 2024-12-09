use std::{collections::HashSet, fs::File, io::{BufRead, BufReader, Read}, result};

use itertools::Itertools;

// ex input
// 2333133121414131402

fn main_1() {
    let file = File::open("input.txt").expect("File not found");
    let mut buf = vec![];
    BufReader::new(file).read_to_end(&mut buf).unwrap();
    let mut buf = buf.iter().map(|&x| x - 48).collect::<Vec<u8>>();
    buf.push(0);
    // sum all elements (in u32)
    let sum = buf.iter().map(|&x| x as usize).sum::<usize>();
    let mut map = Vec::<usize>::with_capacity(sum);
    // iterate buf 2 by 2
    for i in (0..buf.len()).step_by(2) {
        let index = i / 2 + 1;
        let num = buf[i] as u32;
        for _ in 0..num {
            map.push(index);
        }
        let num = buf[i+1] as u32;
        for _ in 0..num {
            map.push(0);
        }
    }
    for i in 0..map.len() {
        let i = map.len() - i - 1;
        if map[i] == 0 {
            continue;
        }
        if let Some(zero_idx) = map.iter().position(|&x| x == 0) {
            if zero_idx > i {
                break;
            }
            map.swap(i, zero_idx);
        }
    }
    let mut sum = 0;
    for i in 0..map.len() {
        if map[i] == 0 {
            break;
        }
        sum += (map[i]-1) * i
    }

    println!("Sum: {:?}", sum);
}

fn main() {
    main_1();
}