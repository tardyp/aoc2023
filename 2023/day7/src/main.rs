use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    ocards: String,
    cards: Vec<i64>,
    bid: i64,
    power: i64,
}
fn char_to_int(c: char) -> i64 {
    // order is A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, then 2, then J
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 0,
        _ => c.to_digit(10).unwrap() as i64
    }
}
fn to_power(cards: Vec<i64>) -> i64 {
    // group the card by their value and count cards of each value
    let mut scards = cards.clone();
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

impl Hand {
    fn new(ocards: String, bid: i64) -> Self {
        let cards = ocards.chars().map(char_to_int).collect::<Vec<i64>>();
        let power = (1..14).into_iter().map(
            |i| to_power(cards.iter().map(
                |c| if *c == 0 {i} else {*c}).collect::<Vec<i64>>()
            )
            ).max().unwrap();
        Self { ocards, cards, bid, power }
    }
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p1 = self.power;
        let p2 = other.power;
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
        println!("{:?}", hand);
        hand
    }).collect::<Vec<_>>();
    hands.sort();
    let mut sum: i64 = 0;
    for (i, h) in hands.iter().enumerate() {
        println!("{} {} {:?}", i+1, h.power, h);
        sum += ((i+1) as i64) * h.bid;
    }
    println!("{}", sum);
}
