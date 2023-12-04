use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, Clone, Hash)]
struct Point {
    val: char,
    x: usize,
    y: usize
}

#[derive(Debug, Hash)]
struct Number {
    val: i64,
    co_ords: Vec<Point>,
    found: bool
}

const PERIOD: char = '.';

fn main() {

    let file: File = File::open("./src/input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut grid:Vec<Vec<Point>> = Vec::new();
    for line in reader.lines().enumerate() {
        let mut row:Vec<Point> = Vec::new();
        for c in line.1.unwrap().chars().into_iter().enumerate() {
            let point = Point{val: c.1, x: c.0, y: line.0};
            row.push(point);
        }
        grid.push(row)
    }

    part1(&grid);
    part2();
}

fn part1(grid: &Vec<Vec<Point>>) {

    let mut total: i64 = 0;
    let mut all: i64 = 0;
    let mut not: i64 = 0;

    let numbers = parse_numbers(&grid);
    for num in numbers {
        all = all.add(num.val);


        if test_adjacents(grid, &num) {
            total = total.add(num.val);
            println!("number: {} - value: {}", num.val, total);
        } else {
            not = not.add(num.val);
        }
    }
    let diff = all - not;
}

fn part2() {
    let mut total: i32 = 0;
    println!("{}", total);
}

fn is_symbol(c: &char) -> bool {
    return !c.is_alphabetic() && 
    !c.is_digit(10) &&
    c != &PERIOD 
}

fn parse_numbers(grid: &Vec<Vec<Point>>) -> Vec<Number> {

    let mut numbers: Vec<Number> = Vec::new();
    for row in grid {
        let mut number_str: String = String::from("");
        let mut co_ords: Vec<Point> = Vec::new();
        for point in row {
            if point.val.is_digit(10) {
                number_str.push(point.val);
                co_ords.push(Point{val: point.val, x: point.x, y: point.y});
                continue;
            }

            if (point.val == PERIOD || is_symbol(&point.val)) && number_str != "" {
                let number_val: i64 = number_str.parse::<i64>().unwrap();
                let number = Number{val: number_val, co_ords: co_ords.clone(), found: false};
                numbers.push(number);
                number_str = "".to_string();
                co_ords.clear();
            }
        }
    }

    //println!("{:?}", numbers);
    numbers
}

fn parse_symbols(grid: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut symbols: Vec<Point> = Vec::new();
    for row in grid {
        for point in row {
            if is_symbol(&point.val) {
                symbols.push(point.clone())
            }
        }
    }
    //println!("{:?}", symbols);
    symbols
}

fn test_adjacents(grid: &Vec<Vec<Point>>, num: &Number) -> bool {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len() - 1;

    for co in &num.co_ords {

        if co.y > 0 {
            let val = get_point_value(grid, co.x, co.y - 1);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x > 0 && co.y > 0 {
            let val = get_point_value(grid, co.x - 1, co.y - 1);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x > 0 {
            let val = get_point_value(grid, co.x - 1, co.y);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x > 0 && co.y + 1 <= max_col {
            let val = get_point_value(grid, co.x - 1, co.y + 1);
            if is_symbol(&val) {
                return true
            }
        }

        if co.y + 1 <= max_col {
            let val = get_point_value(grid, co.x, co.y + 1);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x + 1 <= max_row && co.y + 1 <= max_col {
            let val = get_point_value(grid, co.x + 1, co.y + 1);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x + 1 <= max_row  {
            let val = get_point_value(grid, co.x + 1, co.y);
            if is_symbol(&val) {
                return true
            }
        }

        if co.x + 1 <= max_row && co.y > 0 {
            let val = get_point_value(grid, co.x + 1, co.y -1);
            if is_symbol(&val) {
                return true
            }
        }
    }

    false
}

fn get_point_value(grid: &Vec<Vec<Point>>, x: usize, y: usize) -> char {
    return grid[y][x].val;
}

