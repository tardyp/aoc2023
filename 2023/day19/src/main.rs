use itertools::Itertools;
use nom;
use std::ops::Mul;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, line_ending, space0},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cmp {
    Less,
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    name: usize,
    cmp: Cmp,
    value: i64,
    next: String,
}
fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, (name, cmp, value, _, next)) = tuple((
        take_while1(|c: char| c.is_ascii_alphabetic()),
        alt((
            map(char('<'), |_| Cmp::Less),
            map(char('>'), |_| Cmp::Greater),
        )),
        map_res(digit1, |s: &str| s.parse::<i64>()),
        tag(":"),
        take_while1(|c: char| c.is_ascii_alphabetic()),
    ))(input)?;
    Ok((
        input,
        Step {
            name: // either x, m, a, or s
            match name {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!("Invalid name"),
            },
            cmp,
            value,
            next: next.to_string(),
        },
    ))
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    steps: Vec<Step>,
    default: String,
}
// workflow is in the form: px{a<2006:qkq,m>2090:A,rfg}
// note that the last step does not include comparison
fn parse_workflow(input: &str) -> Result<Workflow, Box<dyn std::error::Error + '_>> {
    let (input, (name, _, steps, _, default, _)) = tuple((
        take_while1(|c: char| c.is_ascii_alphabetic()),
        tag("{"),
        separated_list1(tag(","), parse_step),
        tag(","),
        take_while1(|c: char| c.is_ascii_alphabetic()),
        tag("}"),
    ))(input)?;
    Ok(Workflow {
        name: name.to_string(),
        steps,
        default: default.to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vals {
    ranges: [RangeInclusive<i64>; 4],
    state: String,
}
#[cfg(debug_assertions)]
const FILENAME: &str = "ex.txt";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap().to_string())
        .collect::<Vec<String>>();
    let mut wfs: HashMap<String, Workflow> = HashMap::new();
    let (first, second) = lines.split(|x| x.len() == 0).collect_tuple().unwrap();
    for line in first {
        let wf = parse_workflow(&line).unwrap();
        wfs.insert(wf.name.clone(), wf);
    }
    let mut sum = 0;
    let mut ranges: Vec<Vals> = vec![Vals {
        ranges: [1..=4000, 1..=4000, 1..=4000, 1..=4000],
        // ranges: [1679..=1679,44..=44,2067..=2067,496..=496],
        state: "in".to_string(),
    }];
    while !ranges.is_empty() {
        let mut values = ranges.pop().unwrap();
        if values.state == "A" {
            let v = values
                .ranges
                .iter()
                .map(|x| *x.end() - *x.start() + 1)
                .reduce(i64::mul)
                .unwrap();
            println!("Found A for {:?} {v}", values.ranges);
            sum += v;
            continue;
        }
        if values.state == "R" {
            println!("Found R for {:?}", values.ranges);
            continue;
        }
        let wf = wfs.get(&values.state).unwrap();
        values.state = wf.default.clone();
        for s in &wf.steps {
            let r = &values.ranges[s.name];
            match s.cmp {
                Cmp::Less => {
                    if !(r.contains(&s.value)) {
                        if *r.end() < s.value {
                            values.state = s.next.clone();
                            break;
                        }
                        continue;
                    }
                    let mut new_values = values.clone();
                    new_values.ranges[s.name] = *r.start()..=(s.value - 1);
                    new_values.state = s.next.clone();
                    values.ranges[s.name] = s.value..=*r.end();
                    ranges.push(new_values);
                }
                Cmp::Greater => {
                    if !(r.contains(&s.value)) {
                        // no need to split, but we take the full range to next step
                        if *r.start() > s.value {
                            values.state = s.next.clone();
                            break;
                        }
                        continue;
                    }
                    let mut new_values = values.clone();
                    new_values.ranges[s.name] = (s.value + 1)..=*r.end();
                    new_values.state = s.next.clone();
                    values.ranges[s.name] = *r.start()..=s.value;
                    ranges.push(new_values);
                }
            }
        }
        let mut empty = false;
        for r in &values.ranges {
            if r.is_empty() {
                empty = true;
                break;
            }
        }
        if !empty {
            ranges.push(values);
        }
    }
    println!("Part 2: {}", sum);
}
