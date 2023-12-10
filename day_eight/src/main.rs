use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;
use std::time::{Instant};

fn main() {
    //part1();
    part2();
}

fn part1() {
    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut instructions: Vec<usize> = Vec::new();
    let mut points: HashMap<String, Vec<String>> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {

        if i == 0 {
            instructions = line.as_ref().unwrap().chars().map(|c| {if c == 'L' {0} else {1}}).collect();
            continue;
        }

        if line.as_ref().unwrap().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.as_deref().unwrap().split('=').collect();
        let mapping: Vec<String> = parts[1].replace(['(', ')'], "").split(", ").map(|s| s.trim().to_string()).collect();
        points.insert(parts[0].trim().to_string(), mapping.clone());
    }
    
    let mut curr_maps = points.get("AAA").unwrap();
    let mut point = "".to_string();
    let mut counter = 0;
    let mut instruct = 0;
    while point != *"ZZZ" {
        let next = instructions[instruct];
        point = curr_maps[next].clone();
        curr_maps = points.get(&point).unwrap();
        if instruct + 1 >= instructions.len() {
            instruct = 0;
        } else {
            instruct += 1;
        }
        counter += 1
    }

    println!("{}", counter);

}

fn part2() {

    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut instructions: Vec<usize> = Vec::new();
    let mut points: HashMap<String, Vec<String>> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {

        if i == 0 {
            instructions = line.as_ref().unwrap().chars().map(|c| {if c == 'L' {0} else {1}}).collect();
            continue;
        }

        if line.as_ref().unwrap().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.as_deref().unwrap().split('=').collect();
        let mapping: Vec<String> = parts[1].replace(['(', ')'], "").split(", ").map(|s| s.trim().to_string()).collect();
        points.insert(parts[0].trim().to_string(), mapping.clone());
    }

    let starting_nodes: Vec<String> = points.keys().filter(|k| k.ends_with('A')).cloned().collect();
    
    let mut ends: Vec<i64> = Vec::new();

    for n in starting_nodes {
        let mut curr_maps = points.get(&n).unwrap();
        let mut point = "".to_string();
        let mut counter = 0;
        let mut instruct = 0;

        while !point.ends_with('Z') {
            let next = instructions[instruct];
            point = curr_maps[next].clone();
            curr_maps = points.get(&point).unwrap();
            if instruct + 1 >= instructions.len() {
                instruct = 0;
            } else {
                instruct += 1;
            }
            counter += 1
        }
        ends.push(counter);
    }

    let total = findlcm(ends);
    
    println!("{:?}", total);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a%b)
}
 
// Returns LCM of array elements
fn findlcm(nums: Vec<i64>) -> i64 {
     
    // Initialize result
    let mut ans = nums[0];
    let n = nums.len();

    for i in 0..n {
        ans = (nums[i] * ans) / (gcd(nums[i], ans));
    }

    ans
}