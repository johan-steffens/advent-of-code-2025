use std::{fs};

use crate::shared::Part;

const PATH_PREFIX: &str = "data/day6/";

pub fn run(part: Part, example: bool) {
    println!("day six");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

    let normalised_input = input.lines()
        .map(|line| {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let equation = normalised_input.lines().last().unwrap();

    let mut numbers: Vec<&str> = normalised_input.lines().collect();
    numbers = numbers[..numbers.len() - 1].to_vec();

    let mut groups: Vec<Vec<u64>> = Vec::new();
    let height = numbers.len();
    let width = numbers[0].split(" ").count();

    for x in 0..width {
        let mut group = Vec::new();
        for y in 0..height {
            group.push(numbers[y].split(" ").nth(x).unwrap().parse::<u64>().unwrap());
        }
        groups.push(group);
    }
    
    assert_eq!(groups.len(), equation.split(" ").count());

    println!("sum: {}", sum_groups_part_one(groups, equation));
}

fn sum_groups_part_one(groups: Vec<Vec<u64>>, equation: &str) -> u64 {
    let mut sum = 0;
    for i in 0..groups.len() {
        let nums = groups.get(i).unwrap();
        let equation = equation.split(" ").nth(i).unwrap();

        let mut num = nums.get(0).unwrap().clone();
        for j in 1..nums.len() {
            if equation == "+" {
                num += nums.get(j).unwrap();
            } else if equation == "*" {
                num *= nums.get(j).unwrap();
            }
        }

        sum += num;
    }
    sum
}

fn part_two(input: String) {
    println!("part two");

    let lines: Vec<&str> = input.lines().collect();
    let equation_line = *lines.last().unwrap();
    let normalised_input: Vec<&str> = lines[..lines.len() - 1].to_vec();

    let groups = extract_groups_part_two(normalised_input);

    let equation = equation_line
        .chars()
        .rev()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("sum: {}", sum_groups_part_one(groups, &equation));
}

fn extract_groups_part_two(lines: Vec<&str>) -> Vec<Vec<u64>> {
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut row: Vec<char> = l.chars().collect();
            row.resize(width, ' ');
            row
        })
        .collect();

    let mut groups: Vec<Vec<u64>> = Vec::new();
    let mut current: Vec<u64> = Vec::new();
    let mut prev_x: Option<usize> = None;

    for x in (0..width).rev() {
        for y in (0..height).rev() {
            let ch = grid[y][x];
            if !ch.is_ascii_digit() {
                continue;
            }

            let below_is_digit = y + 1 < height && grid[y + 1][x].is_ascii_digit();
            if below_is_digit {
                continue;
            }

            if let Some(px) = prev_x {
                if px > x + 1 {
                    if !current.is_empty() {
                        groups.push(current);
                        current = Vec::new();
                    }
                }
            }
            prev_x = Some(x);

            let mut yy = y;
            let mut digits: Vec<char> = Vec::new();
            while yy < height && grid[yy][x].is_ascii_digit() {
                digits.push(grid[yy][x]);
                if yy == 0 { break; }
                yy -= 1;
            }

            digits.reverse();
            let s: String = digits.into_iter().collect();
            current.push(s.parse::<u64>().unwrap());
        }
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}