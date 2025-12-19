mod shared;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use crate::shared::Part as Part;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: <day> <part> <use example data (y/n)>, e.g. 1 1 y");
        return;
    }

    let day = args[1].parse::<u32>().unwrap();
    let part = args[2].parse::<u32>().unwrap();

    let example: bool = if args.len() > 3 {
        args[3].parse::<String>().unwrap_or("n".to_string()) == "y"
    } else {
        false
    };

    let matched_part: Part = match part {
        1 => Part::One,
        2 => Part::Two,
        _ => panic!("Invalid part: {}", part),
    };

    match day {
        1 => day1::run(matched_part, example),
        2 => day2::run(matched_part, example),
        3 => day3::run(matched_part, example),
        4 => day4::run(matched_part, example),
        5 => day5::run(matched_part, example),
        6 => day6::run(matched_part, example),
        7 => day7::run(matched_part, example),
        8 => day8::run(matched_part, example),
        9 => day9::run(matched_part, example),
        10 => day10::run(matched_part, example),
        _ => panic!("Day {} not implemented", day),
    }
}
