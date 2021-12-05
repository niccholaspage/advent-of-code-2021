use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_one() {
    let file = File::open("inputs/day_1.txt")
        .expect("Unable to open input file!");

    let mut lines = BufReader::new(file).lines();

    let mut prev: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut increases = 0;

    for line in lines {
        let current: u32 = line.unwrap().parse().unwrap();

        if current > prev {
            increases += 1;
        }

        prev = current;
    }

    println!("There were {} increases!", increases);
}

pub fn part_two() {
    let file = File::open("inputs/day_1.txt")
        .expect("Unable to open input file!");

    let mut increases = 0;

    let mut lines = BufReader::new(file).lines();

    let mut a: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut b: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut c: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        let prev: u32 = a + b + c;
        let line: u32 = line.unwrap().parse().unwrap();
        let current = b + c + line;

        if current > prev {
            increases += 1;
        }

        a = b;
        b = c;
        c = line;
    }

    println!("There were {} increases!", increases);
}