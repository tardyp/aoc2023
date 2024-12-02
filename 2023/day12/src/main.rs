
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum = 0;
    for line in lines {
        let line = line.unwrap();
        let mut parts = sre.split(&line);
        let data = parts.next().unwrap().chars().collect_vec();
        let oneighbors = parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let mut fdata = data.clone();
        let mut neighbors = oneighbors.clone();
        for _ in 0..4 {
            fdata.push('?');
            fdata.append(&mut data.clone());
            neighbors.append(&mut oneighbors.clone());
        }

        let mut suites: HashMap<Vec<usize>, usize> = HashMap::new();
        suites.insert(vec![0usize], 1);
        for c in fdata.iter() {
            match *c {
                '.' => suites = do_dot(suites, &neighbors),
                '#' => suites = do_sharp(suites, &neighbors),
                '?' => {
                    let mut dot_suite = suites.clone();
                    dot_suite = do_dot(dot_suite, &neighbors);
                    suites = do_sharp(suites, &neighbors);
                    for (k, v) in dot_suite.into_iter() {
                        *suites.entry(k).or_insert(0) += v;
                    }
                }
                _ => (),
            }
            // println!("{:?} {:?} {:?}", suites, &fdata[i..], neighbors);
        }

        for (mut x, count) in suites.into_iter() {
            if x[x.len() - 1] == 0 {
                x.pop();
            }
            if *x == neighbors {
                sum += count;
            }
        }
    }
    println!("sum:{}", sum);
}

fn do_sharp(suites:  HashMap<Vec<usize>, usize>, neighbors: &Vec<usize>) ->  HashMap<Vec<usize>, usize> {
    let mut next = HashMap::new();
    for (mut x, count) in suites.into_iter() {
        if x.len() > neighbors.len() {
            continue;
        }
        let last = x.last_mut().unwrap();
        *last += 1;
        if *last <= neighbors[x.len() - 1] {
            *next.entry(x).or_insert(0) += count;
        }
    }
    next
}

fn do_dot(suites:  HashMap<Vec<usize>, usize>, neighbors: &Vec<usize>) ->  HashMap<Vec<usize>, usize> {
    let suites:  HashMap<Vec<usize>, usize> = suites
        .into_iter()
        .filter(|(x, _)| x[x.len()-1] == 0 || neighbors.starts_with(x))
        .collect();
    let mut next = HashMap::new();
    for (mut x, count) in suites.into_iter() {
        if x[x.len() - 1] != 0 {
            x.push(0);
        }
        *next.entry(x).or_insert(0) += count;
    }
    next
}
