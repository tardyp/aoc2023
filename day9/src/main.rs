use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let mut lines = BufReader::new(file).lines();
    let tab = lines.map(|line| {
        let line = line.unwrap();
        sre.split(&line).map(|x|x.parse::<i64>().unwrap()).collect_vec()
    }).collect::<Vec<_>>();
    let mut sum = 0;
    for l in tab {
        // substract each contigous pair of numbers
        let mut tab2 = vec![l];

        loop {
            let cur = tab2[tab2.len()-1].iter().tuple_windows().map(|(a,b)| b-a).collect_vec();
            if cur.iter().all(|x| *x == 0) {
                break;
            }
            tab2.push(cur);
        }
        tab2.reverse();
        sum += tab2.iter().map(|x|x.last().unwrap()).sum::<i64>();
        println!("{:?}", sum);

    }
}
