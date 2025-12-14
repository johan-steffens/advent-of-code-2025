use std::{collections::HashSet, fs};

use crate::shared::Part;

const PATH_PREFIX: &str = "data/day5/";

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

    let ranges: Vec<(u64, u64)> = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums = line.split("-")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            assert_eq!(nums.len(), 2);

            (nums[0], nums[1])
        })
        .collect();

    let ingredients: Vec<u64> = input.lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let fresh_ingredients = ingredients.iter()
        .filter(|ingredient| ranges.iter().any(|range| ingredient >= &&range.0 && ingredient <= &&range.1))
        .count();

    println!("fresh ingredients: {:?}", fresh_ingredients);
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    let mut ranges: Vec<(u64, u64)> = input.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums = line.split("-")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            assert_eq!(nums.len(), 2);

            (nums[0], nums[1])
        })
        .collect();

    ranges.sort_by_key(|range| range.0);

    let merged_ranges = ranges.iter()
        .fold(Vec::new(), |mut acc, range| {
            if acc.is_empty() {
                acc.push(*range);
            } else {
                let last = acc.last_mut().unwrap();

                if range.0 <= last.1 {
                    last.1 = std::cmp::max(last.1, range.1);
                } else {
                    acc.push(*range);
                }
            }

            acc
        });

    let fresh_ids = merged_ranges.iter()
        .fold(0, |acc, range| {
            acc + (range.1 - range.0) + 1
        });

    println!("fresh ids: {:?}", fresh_ids);
}
