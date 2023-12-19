use itertools::Itertools;
use nom;
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
    name: String,
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
            name: name.to_string(),
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
fn parse_workflow(input: & str) -> Result<Workflow, Box<dyn std::error::Error + '_>> {
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
    //{x=787,m=2655,a=1222,s=2876}
    let mut sum = 0;
    for line in second {
        let mut values = line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|x| {
                let (k, v) = x.split_at(x.find('=').unwrap());
                (k.to_string(), v[1..].parse::<i64>().unwrap())
            })
            .collect::<HashMap<String, i64>>();
        println!("{:?}", values);
        let mut next = "in".to_string();
        loop {
            println!("{:?}", next);
            let wf = wfs.get_mut(&next).unwrap();
            next = wf.default.clone();
            for s in &wf.steps {
                let v = *values.get(&s.name).unwrap();
                if s.cmp == Cmp::Less && v < s.value {
                    next = s.next.clone();
                    break;
                } else if s.cmp == Cmp::Greater && v > s.value {
                    next = s.next.clone();
                    break;
                }
            }
            if next == "A" {
                println!("Found A");
                sum += values.values().sum::<i64>();
                break;
            }
            if next == "R" {
                println!("Found R");
                break;
            }
        }
    }
    println!("Part 1: {}", sum);
}
