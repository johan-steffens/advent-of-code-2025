use std::{fs};

use crate::shared::{Part};

const PATH_PREFIX: &str = "data/day9/";

pub fn run(part: Part, example: bool) {
    println!("day eight");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

    println!("input: {}", input);

    let red_tiles: Vec<(u64, u64)> = input.lines()
        .map(|line| {
            let coordinates = line.split(",").collect::<Vec<&str>>();

            assert_eq!(coordinates.len(), 2);

            (coordinates[0].parse::<u64>().unwrap(), coordinates[1].parse::<u64>().unwrap())
        })
        .collect();

    let mut largest_area = 0;
    for i in 0..red_tiles.len() {
        for j in 0..red_tiles.len() {
            if i == j {
                continue;
            }

            let first = red_tiles[i];
            let second = red_tiles[j];
            let mut area = 0;

            if first.0 > second.0 {
                area += first.0 - second.0 + 1;
            } else {
                area += second.0 - first.0 + 1;
            }

            if first.1 > second.1 {
                area *= first.1 - second.1 + 1;
            } else {
                area *= second.1 - first.1 + 1;
            }

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("largest area: {}", largest_area);
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);
}