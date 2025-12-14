use std::fs;

use crate::shared::{Matrix, Point, Part};

const PATH_PREFIX: &str = "data/day4/";

#[derive(Debug)]
struct Position {
    paper: bool,
    forklift_accessible: bool,
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

    let positions: Vec<Vec<Position>> = input.lines()
        .map(|line| line.chars()
                .map(|c| {
                    (match c {
                        '.' => Position { paper: false, forklift_accessible: false },
                        '@' => Position { paper: true, forklift_accessible: false },
                        _ => panic!("unknown character: {}", c),
                    })
                }).collect())
            .collect();

    let mut matrix = Matrix::from_rows(positions);

    let removed_paper = remove_paper(&mut matrix, false);
    println!("removed_paper: {}", removed_paper);
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    let positions: Vec<Vec<Position>> = input.lines()
        .map(|line| line.chars()
                .map(|c| {
                    (match c {
                        '.' => Position { paper: false, forklift_accessible: false },
                        '@' => Position { paper: true, forklift_accessible: false },
                        _ => panic!("unknown character: {}", c),
                    })
                }).collect())
            .collect();

    let mut matrix = Matrix::from_rows(positions);

    let mut removed_paper = 0;
    
    loop {
        let removed = remove_paper(&mut matrix, true);
        removed_paper += removed;

        if removed == 0 {
            break;
        }
    }
    
    println!("removed_paper: {}", removed_paper);
}

fn remove_paper(matrix: &mut Matrix<Position>, remove_paper: bool) -> u32 {
    let width = matrix.width();
    let height = matrix.height();

    let mut removed_paper = 0;
    for y in 0..height {
        for x in 0..width {
            let accessible = {
                let pos = matrix.at(x as i32, y as i32);

                if !pos.get().paper {
                    false
                } else {
                    let neighbours = pos.neighbors8_bounded();

                    println!("neighbours: {:?}", neighbours.iter().count());

                    neighbours.iter()
                        .filter(|n| n.get().paper)
                        .count() < 4
                }
            };

            if accessible {
                matrix
                    .at_mut(x as i32, y as i32)
                    .get_mut()
                    .forklift_accessible = true;

                if remove_paper {
                    matrix
                        .at_mut(x as i32, y as i32)
                        .get_mut()
                        .paper = false;
                }

                removed_paper += 1;
            }
        }
    }

    removed_paper
}