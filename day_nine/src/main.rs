use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    part1()
}

//1842168671 - correct
//1842168680 - incorrect

fn part1() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let report = line.unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        reports.push(report);
    }

    let mut total: i64 = 0;
    for r in reports {
        let mut set: Vec<Vec<i64>> = vec![r];
        set = build_layers(&set);
        total += calc_value(&set);
    }
    println!("{}", total);
}

fn get_deltas(row: Vec<i64>) -> Vec<i64> {
    let mut deltas: Vec<i64> = Vec::new();
    for i in 1..row.len() {
        deltas.push(row[i] - row[i-1]);
    }
    deltas
}

fn build_layers(layers: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut new_layers = layers.clone();
    let last = new_layers.last().unwrap();
    if last.iter().sum::<i64>() != 0 {
        let row = get_deltas(last.to_vec());
        new_layers.push(row);
        new_layers = build_layers(&new_layers).to_vec();
    }
    new_layers.to_vec()
}

fn calc_value(set: &Vec<Vec<i64>>) -> i64 {
    let mut current = set.clone();
    for i in (0..set.len()).rev() {
        if i == set.len() - 1 {
            current[i].push(0);
            continue;
        }
        let a = *current[i].last().unwrap();
        let b = *current[i + 1].last().unwrap();
        current[i].push(a + b);
    }
    for row in &current {
       println!("{:?}", row);
    }
    *current[0].last().unwrap()
}