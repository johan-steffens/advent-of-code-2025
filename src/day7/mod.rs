use std::fs;

use crate::shared::{Matrix, Point, Part};

const PATH_PREFIX: &str = "data/day7/";

#[derive(Debug)]
struct Position {
    is_ray: bool,
    is_splitter: bool,
}

pub fn run(part: Part, example: bool) {
    println!("day seven");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

    println!("input: {}", input);

    let positions: Vec<Vec<Position>> = input.lines()
        .map(|line| line.chars()
                .map(|c| {
                    (match c {
                        '.' => Position { is_ray: false, is_splitter: false },
                        '^' => Position { is_ray: false, is_splitter: true },
                        'S' => Position { is_ray: true, is_splitter: false },
                        _ => panic!("unknown character: {}", c),
                    })
                }).collect())
            .collect();

    let mut matrix = Matrix::from_rows(positions);

    print_matrix(&matrix);

    let mut split_count = 0;
    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let point = matrix.at(x as i32, y as i32).get();

            if point.is_ray {
                if matrix.at(x as i32, y as i32).down(1).get().is_splitter {
                    // Split left
                    matrix.at_mut(x as i32, y as i32)
                        .down(2)
                        .left(1)
                        .get_mut().is_ray = true;

                    // Split right
                    matrix.at_mut(x as i32, y as i32)
                        .down(2)
                        .right(1)
                        .get_mut().is_ray = true;

                    split_count += 1;
                } else {
                    matrix.at_mut(x as i32, y as i32).down(1).get_mut().is_ray = true;
                }
            }
        }
    
        print_matrix(&matrix);
    }

    println!();
    println!("split count: {}", split_count);
}

fn print_matrix(matrix: &Matrix<Position>) {
    println!();

    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let point = matrix.get(x as i32, y as i32);

            print!("{}",
                if point.is_ray {
                    "|"
                } else if point.is_splitter {
                    "^"
                } else {
                    "."
                }
            );
        }
        println!();
    }
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    
}
