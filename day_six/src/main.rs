use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Race {
    time: Vec<i64>,
    distance: Vec<i64>
}

fn main() {
    part1();
    part2();
}


fn part1() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut race: Race = Race { time: Vec::new(), distance: Vec::new() };

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            race.time = line.as_ref().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|n: &str| n.parse::<i64>().unwrap()).collect();
        }
        if i == 1 {
            race.distance = line.unwrap().split(":").nth(1).unwrap().split_whitespace().map(|n: &str| n.parse::<i64>().unwrap()).collect();
        }
    }

    get_wins(&race);
}


fn part2() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut race: Race = Race { time: Vec::new(), distance: Vec::new() };

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            let race_time: Vec<&str> = line.as_ref().unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
            let mut num_str: String = "".to_string();
            for sec in race_time {
                num_str = num_str+ sec;
            }
            race.time.push(num_str.parse::<i64>().unwrap());
        }
        if i == 1 {
            let race_distance: Vec<&str> = line.as_ref().unwrap().split(":").nth(1).unwrap().split_whitespace().collect();
            let mut dist_str: String = "".to_string();
            for sec in race_distance {
                dist_str = dist_str+ sec;
            }
            race.distance.push(dist_str.parse::<i64>().unwrap());
        }
    }

    get_wins(&race);
}

fn get_wins(race: &Race) {
    let mut winning: Vec<i64> = Vec::new();

    for (i, r) in race.time.iter().enumerate() {
        let mut wins = 0;
        for s in 1..*r {
            let dist = (r - s) * s;
            if dist > race.distance[i] {
                wins = wins + 1;
            }
        }
        winning.push(wins);
    }

    let mut total: i64 = 1;
    for w in winning {
        total = total * w;
    }
    println!("{}", total);
}