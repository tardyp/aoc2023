use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct Range {
    pub destination_range_start: i64,
    pub source_range_start: i64,
    pub range_length: i64,
    }
#[derive(Clone, Debug)]
struct Map {
    pub name: String,
    pub ranges: Vec<Range>,
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let first_line = lines.next().unwrap().unwrap();
    let (_, seeds) = first_line.split(": ").collect_tuple().unwrap();
    let seeds = sre
        .split(seeds)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("Seeds: {:?}", seeds);
    lines.next().unwrap().unwrap();
    let mut cur_name = "".to_string();
    let mut cur_ranges = vec![];
    let mut maps = vec![];
    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line.ends_with(":") {
            if !cur_name.is_empty() {
                let map = Map {
                    name: cur_name.clone(),
                    ranges: cur_ranges.clone(),
                };
                maps.push(map);
                // empty cur_ranges
                cur_ranges = vec![];
            }
            let (name, _) = line.split(" ").collect_tuple().unwrap();
            cur_name = name.to_string();
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        println!("{}", line);
        let (destination_range_start, source_range_start, range_length) = sre
        .split(&line)
        .map(|x| x.parse::<i64>().unwrap())
        .collect_tuple().unwrap();
        cur_ranges.push(Range {
            destination_range_start,
            source_range_start,
            range_length,
        });

    }
    let map = Map {
        name: cur_name.clone(),
        ranges: cur_ranges.clone(),
    };
    maps.push(map);
    println!("Maps: {:#?}", maps);

    let mut translation = seeds.iter().map(|x| (*x, *x)).collect::<Vec<(i64, i64)>>();
    for m in maps {
        for i in 0..translation.len() {
            for r in m.ranges.iter() {
                let x = translation[i].0;
                if x >= r.source_range_start && x < r.source_range_start + r.range_length {
                    if i==1 {
                    println!("match! {} {} {}", x, r.source_range_start, r.range_length);}
                    translation[i].0 = r.destination_range_start + x - r.source_range_start;
                    break;
                }
            }
        }
        println!("translation: {} {:?}", m.name, translation);
    }
    translation.sort();
    println!("nearest location: {:?}", translation[0]);
}
