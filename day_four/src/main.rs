
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Game {
    winning: Vec<i64>,
    player: Vec<i64>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut total: i64 = 0;
    for line in reader.lines() {
        let game = parse_game(line.unwrap());
        let res: Vec<i64> = game.player.into_iter().filter(|n| game.winning.contains(n)).collect();
        let points = get_points(res.len() as i64);
        total = total + points;
    }
    println!("{}", total);
}

fn part2() {
    let file: File = File::open("./src/test.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut card_counter: HashMap<i64, i64> = HashMap::new();

    for (card_no, line) in reader.lines().enumerate() {
        let game = parse_game(line.unwrap());
        let wins = get_wins(&game);
    }

    let total: i64 = card_counter.values().sum();
    println!("{}", total);
}

fn get_wins(game: &Game) -> usize {
    let res: Vec<&i64> = game.player.iter().filter(|n| game.winning.contains(n)).collect::<Vec<&i64>>();
    return res.len();
}

fn parse_game(line: String) -> Game {
    let parts: Vec<&str> = line.split(":").collect();
    let game_sections: Vec<&str> = parts[1].split("|").collect();
    let winning_nums: Vec<i64> = game_sections[0].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
    let player_nums: Vec<i64> = game_sections[1].trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
    return Game { winning: winning_nums, player: player_nums }
}

fn get_points(count: i64) -> i64 {
    if count == 0 {
        return count;
    }
    let mut score = 1;
    for n in 1..count {
        score = score + score;
    }
    return score;
}