use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Add;

#[derive(Debug, Clone, Hash)]
struct Point {
    val: char,
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Hash)]
struct Number {
    val: i64,
    co_ords: Vec<Point>,
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
        row.push(Point { val: '.', x: row.len() + 1, y: line.0});
        grid.push(row)
    }

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<Point>>) {

    let mut total: i64 = 0;
    let numbers = parse_numbers(&grid);
    for num in numbers {
        if test_adjacents(grid, &num) {
            total = total.add(num.val);
        } 
    }
    println!("{}", total);
}

fn part2(grid: &Vec<Vec<Point>>) {
    
    let mut total: i64 = 0;
    let numbers: Vec<Number> = parse_numbers(&grid);
    let stars = parse_stars(grid);

    let mut ratios: Vec<Vec<Number>> = Vec::new();

    for star in stars {
        let adjacents = get_adjacents(grid, &star);
        let mut adj_nums: Vec<Number> = Vec::new();
        for num in &numbers {
            if check_intersect(&adjacents, &num.co_ords) {
                adj_nums.push(num.clone());
            }
        }
        ratios.push(adj_nums);
    }

    for ratio in ratios {
        if ratio.len() == 2 {
            let r = ratio[0].val * ratio[1].val;
            total = total.add(r);
        }
    }

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
                let number = Number{val: number_val, co_ords: co_ords.clone()};
                numbers.push(number);
                number_str = "".to_string();
                co_ords.clear();
            }
        }
    }

    // println!("{:?}", numbers);
    numbers
}

fn parse_stars(grid: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut stars: Vec<Point> = Vec::new();
    for row in grid {
        for point in row {
            if point.val == '*' {
                stars.push(point.clone())
            }
        }
    }
    // println!("{:?}", stars);
    stars
}

fn test_adjacents(grid: &Vec<Vec<Point>>, num: &Number) -> bool {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len() - 1;

    for co in &num.co_ords {

        if co.y > 0 {
            if is_symbol(&grid[co.y - 1][co.x].val) {
                return true
            }
        }

        if co.x > 0 && co.y > 0 {
            if is_symbol(&grid[co.y - 1][co.x - 1].val) {
                return true
            }
        }

        if co.x > 0 {
            if is_symbol(&grid[co.y][co.x - 1].val) {
                return true
            }
        }

        if co.x > 0 && co.y + 1 <= max_col {
            if is_symbol(&grid[co.y + 1][co.x - 1].val) {
                return true
            }
        }

        if co.y + 1 <= max_col {
            if is_symbol(&grid[co.y + 1][co.x].val) {
                return true
            }
        }

        if co.x + 1 <= max_row && co.y + 1 <= max_col {
            if is_symbol(&grid[co.y + 1][co.x + 1].val) {
                return true
            }
        }

        if co.x + 1 <= max_row  {
            if is_symbol(&grid[co.y][co.x + 1].val) {
                return true
            }
        }

        if co.x + 1 <= max_row && co.y > 0 {
            if is_symbol(&grid[co.y - 1][co.x + 1].val) {
                return true
            }
        }
    }

    false
}

fn get_adjacents(grid: &Vec<Vec<Point>>, co: &Point) -> Vec<Point> {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len() - 1;

    let mut adjacents: Vec<Point> = Vec::new();

    if co.y > 0 {
        adjacents.push(Point { val: '-', x: co.x, y: co.y - 1 })
    }

    if co.x > 0 && co.y > 0 {
        adjacents.push(Point { val: '-', x: co.x - 1, y: co.y - 1 })
    }

    if co.x > 0 {
        adjacents.push(Point { val: '-', x: co.x - 1, y: co.y })
    }

    if co.x > 0 && co.y + 1 <= max_col {
        adjacents.push(Point { val: '-', x: co.x - 1, y: co.y + 1 })
    }

    if co.y + 1 <= max_col {
        adjacents.push(Point { val: '-', x: co.x, y: co.y + 1 })
    }

    if co.x + 1 <= max_row && co.y + 1 <= max_col {
        adjacents.push(Point { val: '-', x: co.x + 1, y: co.y + 1 })
    }

    if co.x + 1 <= max_row  {
        adjacents.push(Point { val: '-', x: co.x + 1, y: co.y })
    }

    if co.x + 1 <= max_row && co.y > 0 {
        adjacents.push(Point { val: '-', x: co.x + 1, y: co.y - 1 })
    }

    adjacents
    
}

fn check_intersect(one: &Vec<Point>, two: &Vec<Point>) -> bool {
    for p1 in one {
        for p2 in two {
            if p1.x == p2.x && p1.y == p2.y {
                return true
            }
        }
    }
    false
}

