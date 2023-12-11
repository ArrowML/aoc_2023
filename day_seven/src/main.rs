use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
    rank: i64
}

fn main() {
    part1();
   // part2();
}


fn part1() {
    let file: File = File::open("./src/test.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut hands: Vec<Hand> = Vec::new() ;

    for line in reader.lines() {
        let parts: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();
        let cards: Vec<char> = parts[0].chars().collect();
        let bid = parts[1].parse::<i64>().unwrap();
        hands.push(Hand { cards: cards, bid: bid, rank: 0});
    }

    println!("{:?}",hands);
}


fn part2() {

}

