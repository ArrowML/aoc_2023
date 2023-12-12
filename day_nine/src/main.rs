use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    part1()
}


fn part1() {
    let file: File = File::open("./src/test.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let report = line.unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        reports.push(report);
    }

    let mut values: Vec<i64> = Vec::new();
    for r in reports {
        let val: i64 = extrapolate_value(&r);
    }
    
    let total: i64 = values.into_iter().sum();
    println!("{}", total);
}

//0 3 6 9 12 15

fn extrapolate_value(report: &Vec<i64>) -> i64 {
    let total: i64 = 0;
    total
}