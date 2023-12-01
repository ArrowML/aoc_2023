use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    let _ = part1();

    let _ = part2();

    Ok(())

}

fn part1() -> std::io::Result<()> {

    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let expect: [char; 10] = ['1', '2', '3', '4', '5','6', '7', '8', '9', '0'];

    let mut total: i32 = 0;
    for line in reader.lines() {
        let res: Vec<char> = line?.chars().filter(|c| expect.contains(c)).collect();
        let num1 = res[0];
        let num2 = res.last().unwrap();

        let num = (num1.to_string() + &num2.to_string()).parse::<i32>().unwrap();
        total = total + num
    }

    println!("{}", total);

    Ok(())
}

fn part2() -> std::io::Result<()> {

    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut values: HashMap<String, &str> = HashMap::new();
    values.insert("1".to_string(), "1");
    values.insert("2".to_string(), "2");
    values.insert("3".to_string(), "3");
    values.insert("4".to_string(), "4");
    values.insert("5".to_string(), "5");
    values.insert("6".to_string(), "6");
    values.insert("7".to_string(), "7");
    values.insert("8".to_string(), "8");
    values.insert("9".to_string(), "9");
    values.insert("0".to_string(), "0");
    values.insert("one".to_string(), "1");
    values.insert("two".to_string(), "2");
    values.insert("three".to_string(), "3");
    values.insert("four".to_string(), "4");
    values.insert("five".to_string(), "5");
    values.insert("six".to_string(), "6");
    values.insert("seven".to_string(), "7");
    values.insert("eight".to_string(), "8");
    values.insert("nine".to_string(), "9");

    let mut total: i32 = 0;

    for line in reader.lines() {

        let line_str = line?;

        let f_idx = 0;
        let b_idx = line_str.len();
        let mut num1 = "";
        let mut num2 = "";

        for (i, _) in line_str.chars().enumerate() {

            if num1 == "" {
                let front = &line_str[f_idx..i+1];
                num1 = check_num(front, &values)
            } 

            if num2 == "" {
                let back = &line_str[b_idx - i..b_idx];
                num2 = check_num(back, &values)
            }   
        }  

        if num2 == "" {
            num2 = num1;
        }

        let num = (num1.to_string() + &num2.to_string()).parse::<i32>().unwrap();
        total = total + num;
    }

    println!("{}", total);

    Ok(())
}

fn check_num<'a>(s: &'a str, vals: &HashMap<String, &'a str>) -> &'a str {
    for (key, value) in vals {
        if s.find(key).is_some() {
           return value;
        }
    }
    return "";
}