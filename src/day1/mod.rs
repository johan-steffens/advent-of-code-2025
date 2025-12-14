use core::panic;
use std::fs;

use crate::shared::Part as Part;

#[derive(Debug)]
struct Rotation {
    pub direction: char,
    pub amount: i32,
}

impl Rotation {
    pub fn new(direction: char, amount: i32) -> Rotation {
        Rotation { direction, amount }
    }
}

const PATH_PREFIX: &str = "data/day1/";

pub fn run(part: Part, example: bool) {
    println!("day one");
    
    match part {
        Part::One => part_one(example),
        Part::Two => part_two(),
    }
}

fn part_one(example: bool) {
    println!("part one");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "part1.txt" })).unwrap();

    println!("input: {}", input);

    let mut dial: i32 = 50;
    let rotations = input.lines().map(|line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let amount = chars.take_while(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap() as i32;
        Rotation { direction, amount }
    }).collect::<Vec<Rotation>>();

    let mut zeroes = 0;
    for rotation in rotations {
        println!("dial: {}, rotation: {:?}", dial, rotation);
        dial = rotate(dial, rotation);
        println!("new dial: {:?}", dial);

        if dial == 0 {
            zeroes += 1;
        }
    }

    println!("zeroes: {}", zeroes);
}

fn rotate(dial: i32, rotation: Rotation) -> i32 {
    let new_dial = match rotation.direction {
        'L' => dial - (rotation.amount % 100),
        'R' => dial + (rotation.amount % 100),
        _ => panic!("unknown direction"),
    };

    if new_dial < 0 {
        return new_dial + 100;
    } else if new_dial > 99 {
        return new_dial - 100;
    } else {
        return new_dial;
    }
}

fn part_two() {
    println!("part two");
}
