use core::panic;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time,
};
fn hash(s: &str) -> u64 {
    let mut h = 0;
    for c in s.chars() {
        h = h + c as u64;
        h *= 17;
        h &= 0xff;
    }
    h
}
#[derive(Debug)]
struct Val{
    val: i64,
    key: String,
    insertion_time: usize,
}
impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.insertion_time.partial_cmp(&other.insertion_time)
    }
}
impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.insertion_time == other.insertion_time
    }
}
impl Eq for Val {}
impl Ord for Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.insertion_time.cmp(&other.insertion_time)
    }
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r"[-=]").unwrap();
    let data = BufReader::new(file).lines().next().unwrap().unwrap();
    let data = data.split(",").collect_vec();
    println!("{:?}", data);
    let mut boxes = HashMap::new();
    for (i, d) in data.iter().enumerate(){
        let m = sre.find(d).unwrap();
        let k = &d[..m.start()];
        let h = hash(&d[..m.start()]);
        let entry = boxes.entry(h).or_insert(HashMap::new());
        
        if m.as_str() == "-"{
            entry.remove(&k);
        } else {
            let rem = d[m.start()+1..].parse::<i64>().unwrap();
            entry.entry(k).or_insert(Val{val:rem, key:k.to_owned(), insertion_time: i}).val = rem;
        }
    }
    let mut sum = 0;
    for (k, v) in boxes.iter(){
        let mut vals = v.values().collect_vec();
        vals.sort();
        for (i, v) in vals.iter().enumerate(){
            sum += (k+1) as usize *(i+1)* (v.val as usize);
            println!("{k} {i} {:?}", v)
        }
    }
    println!("{:#?}", sum);
}
