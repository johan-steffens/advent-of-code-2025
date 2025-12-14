use core::panic;
use std::fs;

use crate::shared::Part as Part;

#[derive(Debug)]
struct Rotation {
    pub direction: char,
    pub amount: i32,
}

#[derive(Debug)]
struct RotatedDial {
    pub dial: i32,
    pub clicks: i32,
}

const PATH_PREFIX: &str = "data/day1/";

pub fn run(part: Part, example: bool) {
    println!("day one");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

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

        let new_dial = rotate(dial, rotation);
        dial = new_dial.dial;

        println!("new dial: {:?}", dial);

        if dial == 0 {
            zeroes += 1;
        }
    }

    println!("zeroes: {}", zeroes);
}

fn part_two(input: String) {
    println!("part two");

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

        let new_dial = rotate(dial, rotation);
        dial = new_dial.dial;
        println!("new dial: {:?}, clicks: {}", dial, new_dial.clicks);

        zeroes += new_dial.clicks;
    }

    println!("zeroes: {}", zeroes);
}

fn rotate(dial: i32, rotation: Rotation) -> RotatedDial {
    let mut clicks = rotation.amount / 100;
    let new_dial = match rotation.direction {
        'L' => dial - (rotation.amount % 100),
        'R' => dial + (rotation.amount % 100),
        _ => panic!("unknown direction"),
    };

    if rotation.direction == 'L' && dial > 0 && rotation.amount % 100 >= dial {
        clicks += 1;
    } else if rotation.direction == 'R' && dial > 0 && dial + rotation.amount % 100 >= 100 {
        clicks += 1;
    }

    if new_dial < 0 {
        return RotatedDial { dial: new_dial + 100, clicks };
    } else if new_dial > 99 {
        return RotatedDial { dial: new_dial - 100, clicks };
    } else {
        return RotatedDial { dial: new_dial, clicks };
    }
}