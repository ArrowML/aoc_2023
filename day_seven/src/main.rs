use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Hand {
    cards: Vec<i64>,
    bid: i64,
    hand_type: i32
}

const FIVE_OF_A_KIND: i32 = 7;
const FOUR_OF_A_KIND: i32 = 6;
const FULL_HOUSE: i32 = 5;
const THREE_OF_A_KIND: i32 = 4;
const TWO_PAIR: i32 = 3;
const PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

fn main() {
    let card_values: HashMap<char, i64> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    let card_values_joker: HashMap<char, i64> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    //part1(&card_values);
    part2(&card_values_joker);
}


fn part1(card_values: &HashMap<char, i64>) {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut hands: Vec<Hand> = Vec::new() ;

    for line in reader.lines() {
        let parts: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();
        let cards: Vec<i64> = parts[0].chars().map(|c| card_values[&c]).collect();
        let bid = parts[1].parse::<i64>().unwrap();
        let hand_type = get_hand_type(&cards);
        hands.push(Hand{cards: cards, bid: bid, hand_type: hand_type});
    }

    hands.sort_by(|a, b| {
        let first = a.hand_type.cmp(&b.hand_type);
        let second = a.cards[0].cmp(&b.cards[0]);
        let third = a.cards[1].cmp(&b.cards[1]);
        let fourth = a.cards[2].cmp(&b.cards[2]);
        let fifth = a.cards[3].cmp(&b.cards[3]);
        let six = a.cards[4].cmp(&b.cards[4]);
        first.then(second).then(third).then(fourth).then(fifth).then(six)
    });

    let total = get_rank_bid_total(hands);
    println!("{}", total);
}



fn part2(card_values: &HashMap<char, i64>) {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut hands: Vec<Hand> = Vec::new() ;

    for line in reader.lines() {
        let parts: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();
        let cards: Vec<i64> = parts[0].chars().map(|c| card_values[&c]).collect();
        let bid = parts[1].parse::<i64>().unwrap();
        let hand_type = get_hand_type_jokers(&cards);
        hands.push(Hand{cards: cards, bid: bid, hand_type: hand_type});
    }

    hands.sort_by(|a, b| {
        let first = a.hand_type.cmp(&b.hand_type);
        let second = a.cards[0].cmp(&b.cards[0]);
        let third = a.cards[1].cmp(&b.cards[1]);
        let fourth = a.cards[2].cmp(&b.cards[2]);
        let fifth = a.cards[3].cmp(&b.cards[3]);
        let six = a.cards[4].cmp(&b.cards[4]);
        first.then(second).then(third).then(fourth).then(fifth).then(six)
    });

    for h in &hands {
       //  println!("{:?}", h);
    }

    let total = get_rank_bid_total(hands);
    println!("{}", total);
}


fn get_hand_type(cards: &Vec<i64>) -> i32 {
    let mut card_count: HashMap<i64, i32> = HashMap::new();
    for c in cards {
        *card_count.entry(*c).or_insert(0) += 1;
    }

    if card_count.len() == 5 {
        return HIGH_CARD;
    }

    if card_count.len() == 4 {
        return PAIR;
    }

    if card_count.len() == 3 {

        let count = card_count.values().max().unwrap();
        if *count == 3 {
            return THREE_OF_A_KIND;
        }
        return TWO_PAIR;
    }

    if card_count.len() == 2 {
        let count = card_count.values().max().unwrap();
        if *count == 4 {
            return FOUR_OF_A_KIND;
        }
        return FULL_HOUSE;        
    }

    FIVE_OF_A_KIND
}

fn get_hand_type_jokers(cards: &Vec<i64>) -> i32 {
    let mut card_count: HashMap<i64, i32> = HashMap::new();
    for c in cards {
        *card_count.entry(*c).or_insert(0) += 1;
    }

    let joker_count = card_count.get(&1).unwrap_or(&0);
    println!("{}", joker_count);

    if card_count.len() == 5 {
        if joker_count > &0 {
            return PAIR;
        }
        return HIGH_CARD;
    }

    if card_count.len() == 4 {
        if joker_count > &0 {
            return THREE_OF_A_KIND;
        }
        return PAIR;
    }

    if card_count.len() == 3 {

        let count = card_count.values().max().unwrap();
        if *count == 3 {
            if joker_count > &0 {
                return FOUR_OF_A_KIND;
            }
            return THREE_OF_A_KIND;
        }

        if joker_count == &1 {
            return FULL_HOUSE;
        }
        if joker_count == &2 {
            return FOUR_OF_A_KIND;
        }

        return TWO_PAIR;
    }

    if card_count.len() == 2 {
        let count = card_count.values().max().unwrap();
        if *count == 4 {
            if joker_count > &0 {
                return FIVE_OF_A_KIND;
            }
             return FOUR_OF_A_KIND;
        }
        if joker_count > &0 {
            return FIVE_OF_A_KIND;
        }
        return FULL_HOUSE;        
    }

    FIVE_OF_A_KIND
}

fn get_rank_bid_total(hands: Vec<Hand>) -> i64 {
    let mut total: i64 = 0;
    for (r, hand) in hands.iter().enumerate() {
        total += hand.bid * (r as i64 + 1)
    }
    total
}
