use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_one() {
    let file = File::open("inputs/day_2.txt").unwrap();

    let lines = BufReader::new(file).lines();

    let mut x = 0;
    let mut depth = 0;

    for line in lines {
        let test = line.unwrap();
        let mut split = test.split(" ");

        let command = split.next().unwrap();
        let amount: u32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => x += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => ()
        }
    }

    println!("Horizontal: {}", x);
    println!("Depth: {}", depth);
    println!("Result: {}", x * depth);
}

pub fn part_two() {
    let file = File::open("inputs/day_2.txt").unwrap();

    let lines = BufReader::new(file).lines();

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let test = line.unwrap();
        let mut split = test.split(" ");

        let command = split.next().unwrap();
        let amount: u32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => {
                x += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => ()
        }
    }

    println!("Horizontal: {}", x);
    println!("Depth: {}", depth);
    println!("Result: {}", x * depth);
}