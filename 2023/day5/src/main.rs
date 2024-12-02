use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

#[derive(Clone, Debug)]
struct TranslationRange {
    pub translation: i64,
    pub source_range: Range<i64>,
}
#[derive(Clone, Debug)]
struct Map {
    pub name: String,
    pub ranges: Vec<TranslationRange>,
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
    let seeds = seed_ranges_to_seed(seeds);
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
        let (destination_range_start, source_range_start, range_length) = sre
            .split(&line)
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        cur_ranges.push(TranslationRange {
            translation: destination_range_start - source_range_start,
            source_range: Range {
                start: source_range_start,
                end: source_range_start + range_length,
            },
        });
    }
    let map = Map {
        name: cur_name.clone(),
        ranges: cur_ranges.clone(),
    };
    maps.push(map);
    let mut ranges = seeds.clone();
    for m in maps {
        // first iteration, we split the ranges so that translation ranges don't overlap
        for tr in m.ranges.iter() {
            let mut next_ranges: Vec<Range<i64>> = vec![];
            for ri in ranges {
                let ri = ri.clone();
                //[    ] TR
                //   [] RI
                if tr.source_range.contains(&ri.start) && tr.source_range.contains(&ri.end) {
                    next_ranges.push(ri.clone());
                }
                //[    ] TR
                //   [ |   ] RI
                else if tr.source_range.contains(&ri.start) && !tr.source_range.contains(&ri.end)
                {
                    next_ranges.push(Range {
                        start: ri.start,
                        end: tr.source_range.end,
                    });
                    next_ranges.push(Range {
                        start: tr.source_range.end,
                        end: ri.end,
                    });
                }
                //      [    ] TR
                //   [  |  ] RI
                else if !tr.source_range.contains(&ri.start) && tr.source_range.contains(&ri.end)
                {
                    next_ranges.push(Range {
                        start: ri.start,
                        end: tr.source_range.start,
                    });
                    next_ranges.push(Range {
                        start: tr.source_range.start,
                        end: ri.end,
                    });
                }
                //     [ ] TR
                //   [ | | ] RI
                else if ri.contains(&tr.source_range.start) && ri.contains(&tr.source_range.end) {
                    next_ranges.push(Range {
                        start: ri.start,
                        end: tr.source_range.start,
                    });
                    next_ranges.push(Range {
                        start: tr.source_range.start,
                        end: tr.source_range.end,
                    });
                    next_ranges.push(Range {
                        start: tr.source_range.end,
                        end: ri.end,
                    });
                } else {
                    next_ranges.push(ri.clone());
                }
            }
            ranges = next_ranges;
        }
        let mut next_ranges = vec![];
        for ri in ranges {
            let mut found = false;
            for tr in m.ranges.iter() {
                if tr.source_range.contains(&ri.start) && tr.source_range.contains(&(ri.end-1)) {
                    next_ranges.push(Range {
                        start: ri.start + tr.translation,
                        end: ri.end + tr.translation,
                    });
                    found = true;
                    break;
                }
            }
            if !found {
                next_ranges.push(ri.clone());
            }
        }
        ranges = next_ranges;
    }
    let min = ranges.iter().map(|x| x.start).min().unwrap();
    println!("min: {min}");
    assert_eq!(min, 37384986);
}

fn seed_ranges_to_seed(seed_ranges: Vec<i64>) -> Vec<Range<i64>> {
    seed_ranges
        .iter()
        .tuple_windows()
        .step_by(2)
        .map(|(a, b)| (*a..*a + *b))
        .collect::<Vec<Range<i64>>>()
}
