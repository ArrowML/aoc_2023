use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, Clone, Default)]
struct Point {
    val: char,
    x: usize,
    y: usize
}


fn main() {

    let file: File = File::open("./src/test.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut grid:Vec<Vec<Point>> = Vec::new();
    let mut start: Point = Point::default();
    for line in reader.lines().enumerate() {
        let mut row:Vec<Point> = Vec::new();
        for c in line.1.unwrap().chars().into_iter().enumerate() {
            let point = Point{val: c.1, x: c.0, y: line.0};
            if c.1 == 'S' {
                start = point.clone();
            }
            row.push(point);
        }
        grid.push(row)
    }

    part1(&grid, &start);

}

fn part1(grid: &Vec<Vec<Point>>, start: &Point) {

    let mut end = false;
    let mut counter = 0;
    while !end {

        let mut next = Point::default();
        let mut heading: char = '.';

        counter += 1;
        if same_point(&next, start){
            end = true;
        }
        
    }

}

fn same_point(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

fn get_next_pipes(grid: &Vec<Vec<Point>>, from: &Point, to: &Point) -> Vec<Point> {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len();

    let mut adjacents: Vec<Point> = Vec::new();

    if to.y > 0 {
        let top = Point { val: grid[to.y-1][to.x].val, x: to.x, y: to.y - 1 };
        if !same_point(from, &top) {
            adjacents.push(top)
        } 
    }

    if to.x > 0 {
        let left = Point { val: grid[to.y][to.x-1].val, x: to.x - 1, y: to.y };
        if !same_point(from, &left) {
            adjacents.push(left)
        } 
    }

    if to.y < max_col {
        let bottom = Point { val: grid[to.y+1][to.x].val, x: to.x, y: to.y + 1 };
        if !same_point(from, &bottom) {
            adjacents.push(bottom)
        } 
    }

    if to.x < max_row  {
        let right = Point { val: grid[to.y][to.x+1].val, x: to.x + 1, y: to.y };
        if !same_point(from, &right) {
            adjacents.push(right)
        } 
    }

    adjacents
    
}