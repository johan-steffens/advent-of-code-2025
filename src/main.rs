pub mod shared;
pub mod day1;

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
        args[3].parse::<String>().unwrap() == "y"
    } else {
        false
    };

    let matched_part = match part {
        1 => Part::One,
        2 => Part::Two,
        _ => panic!("Invalid part: {}", part),
    };

    match day {
        1 => day1::run(matched_part, example),
        _ => panic!("Day {} not implemented", day),
    }
}
