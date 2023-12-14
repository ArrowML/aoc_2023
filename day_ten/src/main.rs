use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, Clone, Default)]
struct Point {
    joint: Joint,
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Default)]
struct Joint {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

fn main() {

    let file: File = File::open("./src/test.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let joint_types: HashMap<char, Joint> = HashMap::from([
        ('|', Joint{north: true, south: true, east:false, west: false}),
        ('-', Joint{north: false, south: false, east:true, west: true}),
        ('L', Joint{north: true, south: false, east:true, west: false}),
        ('J', Joint{north: true, south: false, east:false, west: true}),
        ('7', Joint{north: false, south: true, east:false, west: true}),
        ('F', Joint{north: false, south: true, east:true, west: false}),
        ('.', Joint{north: false, south: false, east:false, west: false}),
        ('S', Joint{north: true, south: true, east:true, west: true})
    ]);

    let mut grid:Vec<Vec<Point>> = Vec::new();
    let mut start: Point = Point::default();
    for line in reader.lines().enumerate() {
        let mut row:Vec<Point> = Vec::new();
        for c in line.1.unwrap().chars().enumerate() {
            let point = Point{joint: joint_types[&c.1].clone(), x: c.0, y: line.0};
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
    let _ = grid;

    let mut end = false;
    let mut counter = 0;
    let mut next = start.clone();
    let mut prev = Point::default();

    while !end {
        let adjacents = get_next_pipes(grid, &prev, &next);
        println!("{:?}", adjacents);
        prev = next.clone();
        next = get_next_joint(adjacents, &prev);

        counter += 1;
        /*
        if same_point(&next, start){
            end = true;
        }
        */

        if counter == 5 {
            end = true;
        }
    }

}

fn get_next_joint(adjacents: Vec<Point>, current: &Point) -> Point {

    for a in adjacents {
        if current.joint.north && a.joint.north {
            println!("{:?}", a);
            return a.clone();
        }
        if current.joint.south && a.joint.south {
            println!("{:?}", a);
            return a.clone();
        }
        if current.joint.east && a.joint.east {
            println!("{:?}", a);
            return a.clone();
        }
        if current.joint.west && a.joint.west {
            println!("{:?}", a);
            return a.clone();
        }
    }
    Point::default()
}

fn same_point(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x && p1.y == p2.y
}

fn get_next_pipes(grid: &Vec<Vec<Point>>, prev: &Point, to: &Point) -> Vec<Point> {

    let max_col = grid.len() - 1;
    let max_row = grid[0].len();

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