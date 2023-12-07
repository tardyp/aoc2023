use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    ocards: String,
    cards: Vec<i64>,
    bid: i64
}
fn char_to_int(c: char) -> i64 {
    // order is A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, then 2
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i64
    }
}
impl Hand {
    fn new(ocards: String, bid: i64) -> Self {
        let cards = ocards.chars().map(char_to_int).collect::<Vec<i64>>();
        Self { ocards, cards, bid }
    }
    fn to_power(&self) -> i64 {
        // group the card by their value and count cards of each value
        let mut scards = self.cards.clone();
        scards.sort();
        let groups = scards.iter().group_by(|&c| c);
        let mut groups = groups.into_iter().map(|(c, group)| (group.count(), *c)).collect::<Vec<(usize, i64)>>();
        groups.sort();
        groups.reverse();
        if groups[0].0 == 5 {
            return 7;
        }
        if groups[0].0 == 4 {
            return 6;
        }
        if groups[0].0 == 3 && groups[1].0 == 2 {
            return 5;
        }
        if groups[0].0 == 3 {
            return 4;
        }
        if groups[0].0 == 2 && groups[1].0 ==2 {
            return 3;
        }
        if groups[0].0 == 2{
            return 2;
        }
        1
    }
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p1 = self.to_power();
        let p2 = other.to_power();
        if p1 != p2 {
            return Some(p1.cmp(&p2));
        }
        Some(self.cards.cmp(&other.cards))
    }
}
impl Ord for Hand{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
fn main() {
    let file = File::open("input.txt").expect("File not found");
    let sre = regex::Regex::new(r" +").unwrap();
    let lines = BufReader::new(file).lines();
    let mut hands = lines.map(|line|{
        let line = line.unwrap();
        let (hand, bid) = sre.split(&line).collect_tuple().unwrap();
        let hand = Hand::new(hand.to_string(), bid.parse().unwrap());
        hand.to_power();
        println!("{:?}", hand);
        hand
    }).collect::<Vec<_>>();
    hands.sort();
    let mut sum: i64 = 0;
    for (i, h) in hands.iter().enumerate() {
        println!("{} {} {:?}", i+1, h.to_power(), h);
        sum += ((i+1) as i64) * h.bid;
    }
    println!("{}", sum);
}
