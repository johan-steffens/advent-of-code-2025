use std::fs;

use crate::shared::Part as Part;

const PATH_PREFIX: &str = "data/day2/";

pub fn run(part: Part, example: bool) {
    println!("day two");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

    println!("input: {}", input);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    
    input.lines().for_each(|line| {
        let range_string = line.split(',');
        for range in range_string {
            let split = range.split_once('-').unwrap();
            ranges.push((split.0.parse::<i64>().unwrap(), split.1.parse::<i64>().unwrap()));
        }
    });

    let mut invalid_sum: i64 = 0;
    for range in ranges {
        println!("range: {}-{}", range.0, range.1);

        for current_id in range.0..=range.1 {
            println!("current_id: {}", current_id);

            let valid: bool = id_valid(current_id);

            if !valid {
                println!("invalid: {}", current_id);
                invalid_sum += current_id;
            }
        }
    }

    println!("invalid_sum: {}", invalid_sum);    
}

fn id_valid(id: i64) -> bool {
    let id_string: String = format!("{id}");
    if id_string.len() % 2 != 0 {
        return true;
    }

    let middle = id_string.len() / 2;
    let left = &id_string[..middle];
    let right = &id_string[middle..];
    println!("middle: {} parts: {}={}", middle, left, right);

    return left != right;
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    
    input.lines().for_each(|line| {
        let range_string = line.split(',');
        for range in range_string {
            let split = range.split_once('-').unwrap();
            ranges.push((split.0.parse::<i64>().unwrap(), split.1.parse::<i64>().unwrap()));
        }
    });

    let mut invalid_sum: i64 = 0;
    for range in ranges {
        println!("range: {}-{}", range.0, range.1);

        for current_id in range.0..=range.1 {
            println!("current_id: {}", current_id);

            let valid: bool = id_valid_part_two(current_id);

            if !valid {
                println!("invalid: {}", current_id);
                invalid_sum += current_id;
            }
        }
    }

    println!("invalid_sum: {}", invalid_sum);
}

fn id_valid_part_two(id: i64) -> bool {
    let id_string: String = format!("{id}");
    let length = id_string.len();

    for pattern_length in 1..(length / 2) + 1 {
        if length % pattern_length == 0 {
            let pattern = &id_string[..pattern_length];

            let repeated = length / pattern_length;
            if pattern.repeat(repeated) == id_string {
                return false;
            }
        }
    }

    return true;
}