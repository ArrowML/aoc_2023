use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone, Default)]
struct Point {
    val: char,
    x: usize,
    y: usize
}

fn main() {

    let file: File = File::open("./src/test2.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut grid:Vec<Vec<Point>> = Vec::new();
    let mut start: Point = Point::default();
    for line in reader.lines().enumerate() {
        let mut row:Vec<Point> = Vec::new();
        for c in line.1.unwrap().chars().enumerate() {
            let point = Point{val: c.1, x: c.0, y: line.0};
            if c.1 == 'S' {
                start = point.clone();
            }
            row.push(point);
        }
        grid.push(row)
    }

    //part1(&grid, &start);

    part2(&grid, &start);

}

fn part1(grid: &Vec<Vec<Point>>, start: &Point) {
    let _ = grid;

    let mut end = false;
    let mut counter = 0;
    let mut next = start.clone();
    let mut prev = Point::default();

    while !end {
        let adjacents = get_next_pipes(grid, &prev, &next);
        prev = next.clone();
        next = get_next_joint(adjacents, &prev);

        counter += 1;
        if same_point(&next, start) {
            end = true;
        } 
    }

    println!("{}",counter/2);
}

fn part2(grid: &Vec<Vec<Point>>, start: &Point) {
    let _ = grid;

    let mut end = false;
    let mut next = start.clone();
    let mut prev = Point::default();

    let mut pipe_points: HashMap<(usize, usize), Point> = HashMap::new();
    pipe_points.insert((start.x, start.y), start.clone());

    while !end {
        let adjacents = get_next_pipes(grid, &prev, &next);
        prev = next.clone();
        next = get_next_joint(adjacents, &prev);
        pipe_points.insert((next.x, next.y), next.clone());
        if same_point(&next, start) {
            end = true;
        } 
    }

    let mut area: Vec<Point> = Vec::new();
    let mut inside: bool = false;

    for r in grid {
        for p in r {
            let co = (p.x, p.y);
            //get ranges
        }
    }

    println!("{:?}",pipe_points);
}

fn get_next_joint(adjacents: Vec<Point>, current: &Point) -> Point {

    let mut ops: Vec<Point> = Vec::new();

    for a in adjacents.clone() {
        if current.val == 'S' && a.x > current.x && (a.val == '-' || a.val == 'J' || a.val == '7')  {
            ops.push(a.clone())
        }
        if current.val == 'S' && a.y > current.y && (a.val == '|' || a.val == 'J' || a.val == 'L') {
            ops.push(a.clone())
        }
        if current.val == '|' && a.y < current.y && (a.val == '|' || a.val == '7' || a.val == 'F' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == '|' && a.y > current.y && (a.val == '|' || a.val == 'J' || a.val == 'L' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == '-' && a.x < current.x && (a.val == '-' || a.val == 'F' || a.val == 'L' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == '-' && a.x > current.x && (a.val == '-' || a.val == 'J' || a.val == '7' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'L' && a.x > current.x && (a.val == '-' || a.val == 'J' || a.val == '7' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'L' && a.y < current.y && (a.val == '|' || a.val == 'F' || a.val == '7' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'J' && a.x < current.x && (a.val == '-' || a.val == 'F' || a.val == 'L' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'J' && a.y < current.y && (a.val == '|' || a.val == 'F' || a.val == '7' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == '7' && a.x < current.x && (a.val == '-' || a.val == 'F' || a.val == 'L' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == '7' && a.y > current.y && (a.val == '|' || a.val == 'L' || a.val == 'J' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'F' && a.x > current.x && (a.val == '-' || a.val == 'J' || a.val == '7' || a.val == 'S') {
            ops.push(a.clone())
        }
        if current.val == 'F' && a.y > current.y && (a.val == '|' || a.val == 'L' || a.val == 'J' || a.val == 'S') {
            ops.push(a.clone())
        }
    }
    return ops.first().unwrap().clone();
    
}

fn same_point(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

fn get_next_pipes(grid: &Vec<Vec<Point>>, prev: &Point, to: &Point) -> Vec<Point> {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len() -1;

    let mut adjacents: Vec<Point> = Vec::new();

    if to.y > 0 {
        let top = grid[to.y-1][to.x].clone();
        if !same_point(prev, &top) {
            adjacents.push(top)
        }
    }

    if to.x > 0 {
        let left = grid[to.y][to.x-1].clone();
        if !same_point(prev, &left) {
            adjacents.push(left)
        }
    }

    if to.y < max_col {
        let bottom = grid[to.y+1][to.x].clone();
        if !same_point(prev, &bottom) {
            adjacents.push(bottom)
        }
        
    }

    if to.x < max_row  {
        let right = grid[to.y][to.x+1].clone();
        if !same_point(prev, &right) {
            adjacents.push(right)
        }
    }

    adjacents
    
}