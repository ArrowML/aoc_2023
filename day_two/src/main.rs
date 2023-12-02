use std::fs::File;
use std::io::{BufReader, BufRead};

use std::str::Split;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}

#[derive(Debug)]
struct Set {
    blue: i32,
    green: i32,
    red: i32
}

impl Default for Set {
    fn default () -> Set {
        Set { blue: 0, green: 0, red: 0 }
    }
}

const BLUE: &str = "blue";
const GREEN: &str = "green";
const RED: &str = "red";

const WIN_SET: Set = Set{blue: 14, green: 13, red: 12};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let game = parse_game(line.unwrap());
        if check_game(&game) {
            total = total + game.id;
        }
    }
    println!("{}", total);
}

fn part2() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut total: i32 = 0;
    for line in reader.lines() {
        let game = parse_game(line.unwrap());
        let min: Set = get_minimums(&game);
        let pow = min.blue * min.green * min.red;
        total = total + pow;
    }
    println!("{}", total);
}

fn parse_game(line: String) -> Game {
    let parts: Vec<&str> = line.split(":").collect();
    let id: i32 = get_id(parts[0]);
    let sets: Vec<Set> = get_sets(parts[1]);
    let game = Game {id: id, sets: sets};
    game
}

fn get_id(id: &str) -> i32 {
    let id_parts: Vec<&str> = id.split(" ").collect();
    return id_parts[1].parse::<i32>().unwrap();
}
//  2 green
fn get_sets(sets_string: &str) -> Vec<Set> {
    let set_parts: Split<&str> = sets_string.trim().split(";");
    let mut sets: Vec<Set> = Vec::new();
    for set in set_parts {
        let mut curr_set = Set::default();
        let part: Split<&str> = set.trim().split(",");
        for colour in part {
            let col_parts: Vec<&str> = colour.trim().split(" ").collect();
            let count = col_parts[0].parse::<i32>().unwrap();
            if col_parts[1].trim() == BLUE {
                curr_set.blue = count;
            }
            if col_parts[1].trim() == GREEN {
                curr_set.green = count;
            }
            if col_parts[1].trim() == RED {
                curr_set.red = count;
            }
        }
        sets.push(curr_set)
    }
    return sets;
}

fn check_game(game: &Game) -> bool {
    for set in &game.sets {
        if set.blue > WIN_SET.blue {
            return false
        }
        if set.green > WIN_SET.green {
            return false
        }
        if set.red > WIN_SET.red {
            return false
        }
    }
    true
}

fn get_minimums(game: &Game) -> Set {
    let mut min_set: Set = Set::default();
    for set in &game.sets {
        if set.blue > min_set.blue {
            min_set.blue = set.blue;
        }
        if set.green > min_set.green {
            min_set.green = set.green;
        }
        if set.red > min_set.red {
            min_set.red = set.red;
        }
    }
    return min_set;
}

