use std::fs;

use crate::shared::Part as Part;

const PATH_PREFIX: &str = "data/day3/";

struct Bank {
    battery: Vec<u32>,
}

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

    let banks: Vec<Bank> = input.lines()
        .map(|line| Bank { battery: line.chars().map(|c| c.to_digit(10).unwrap()).collect() })
        .collect();

    let mut jolt_sum = 0;
    for bank in banks {
        println!("bank: {:?}", bank.battery);

        jolt_sum += get_max_jolts(bank);
    }

    println!("jolt sum: {}", jolt_sum);
}

fn get_max_jolts(bank: Bank) -> u32 {
    let mut max_jolts = 0;

    for i in 0..bank.battery.len() {
        for j in i+1..bank.battery.len() {
            let jolt = format!("{}{}", bank.battery[i], bank.battery[j]).parse::<u32>().unwrap();
            max_jolts = max_jolts.max(jolt);
        }
    }

    max_jolts
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);
}