use std::{collections::{HashMap, HashSet}, fs};

use crate::shared::{Matrix, Point, Part};

const PATH_PREFIX: &str = "data/day7/";

#[derive(Debug, Clone, Copy)]
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

    let mut split_count = 0;
    for y in 0..matrix.height() - 1 {
        for x in 0..matrix.width() {
            let point = matrix.at(x as i32, y as i32).get();

            if point.is_ray {
                if matrix.at(x as i32, y as i32).down(1).get().is_splitter {
                    // Split left
                    matrix.at_mut(x as i32, y as i32)
                        .down(1)
                        .left(1)
                        .get_mut().is_ray = true;

                    // Split right
                    matrix.at_mut(x as i32, y as i32)
                        .down(1)
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

    let matrix = Matrix::from_rows(positions);

    print_matrix(&matrix);

    // Find starting position
    let mut start_x = 0;
    let mut start_y = 0;
    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            if matrix.get(x as i32, y as i32).is_ray {
                start_x = x;
                start_y = y;
            }
        }
    }

    let mut dp: HashMap<(usize, usize), u64> = HashMap::new();

    let bottom_y = matrix.height() - 1;
    for x in 0..matrix.width() {
        dp.insert((x, bottom_y), 1);
    }

    // Process rows from bottom to top
    for y in (0..matrix.height()).rev() {
        for x in 0..matrix.width() {
            if y == bottom_y {
                continue;
            }

            let next_y = y + 1;
            let point = matrix.get(x as i32, next_y as i32);

            if point.is_splitter {
                // Sum paths from going left and right
                let mut total = 0;

                // Try going left
                if x > 0 {
                    if let Some(&count) = dp.get(&(x - 1, next_y)) {
                        total += count;
                    }
                }

                // Try going right
                if x < matrix.width() - 1 {
                    if let Some(&count) = dp.get(&(x + 1, next_y)) {
                        total += count;
                    }
                }

                dp.insert((x, y), total);
            } else {
                if let Some(&count) = dp.get(&(x, next_y)) {
                    dp.insert((x, y), count);
                }
            }
        }
    }

    let timeline_count = dp.get(&(start_x, start_y)).copied().unwrap_or(0);

    println!();
    println!("timeline count: {}", timeline_count);
}