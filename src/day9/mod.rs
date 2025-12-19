use std::{fs};

use crate::shared::{Matrix, Part};

const PATH_PREFIX: &str = "data/day9/";

pub fn run(part: Part, example: bool) {
    println!("day nine");

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
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let width = (x1.abs_diff(x2) as u128) + 1;
            let height = (y1.abs_diff(y2) as u128) + 1;
            let area = width * height;

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

    let red_tiles: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            let coordinates = line.split(",").collect::<Vec<&str>>();

            assert_eq!(coordinates.len(), 2);

            (coordinates[0].parse::<i64>().unwrap(), coordinates[1].parse::<i64>().unwrap())
        })
        .collect();

    let mut largest_area = 0;
    
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            if is_rectangle_in_polygon(&red_tiles, x_min, x_max, y_min, y_max) {
                let width = ((x_max - x_min) as u128) + 1;
                let height = ((y_max - y_min) as u128) + 1;
                let area = width * height;

                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    println!("largest area: {}", largest_area);
}

fn is_rectangle_in_polygon(polygon: &Vec<(i64, i64)>, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    let corners = [
        (x_min, y_min),
        (x_max, y_min),
        (x_min, y_max),
        (x_max, y_max),
    ];

    for corner in &corners {
        if !point_in_polygon_or_boundary(polygon, *corner) {
            return false;
        }
    }

    let width = x_max - x_min;
    let height = y_max - y_min;
    
    if width <= 500 && height <= 500 {
        // For small rectangles, check every point
        for x in x_min..=x_max {
            if !point_in_polygon_or_boundary(polygon, (x, y_min)) 
                || !point_in_polygon_or_boundary(polygon, (x, y_max)) {
                return false;
            }
        }
        
        for y in (y_min + 1)..y_max {
            if !point_in_polygon_or_boundary(polygon, (x_min, y)) 
                || !point_in_polygon_or_boundary(polygon, (x_max, y)) {
                return false;
            }
        }
    } else {
        // For large rectangles, sample points
        let x_sample = (width / 100).max(1);
        let y_sample = (height / 100).max(1);
        
        for x in (x_min..=x_max).step_by(x_sample as usize) {
            if !point_in_polygon_or_boundary(polygon, (x, y_min)) 
                || !point_in_polygon_or_boundary(polygon, (x, y_max)) {
                return false;
            }
        }
        
        for y in (y_min..=y_max).step_by(y_sample as usize) {
            if !point_in_polygon_or_boundary(polygon, (x_min, y)) 
                || !point_in_polygon_or_boundary(polygon, (x_max, y)) {
                return false;
            }
        }
        
        if !point_in_polygon_or_boundary(polygon, (x_max, y_min)) 
            || !point_in_polygon_or_boundary(polygon, (x_max, y_max))
            || !point_in_polygon_or_boundary(polygon, (x_min, y_max)) {
            return false;
        }
    }

    true
}

fn point_in_polygon_or_boundary(polygon: &Vec<(i64, i64)>, point: (i64, i64)) -> bool {
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        
        if a.0 == b.0 {
            // Vertical edge
            let y_min = a.1.min(b.1);
            let y_max = a.1.max(b.1);
            if point.0 == a.0 && point.1 >= y_min && point.1 <= y_max {
                return true;
            }
        } else if a.1 == b.1 {
            // Horizontal edge
            let x_min = a.0.min(b.0);
            let x_max = a.0.max(b.0);
            if point.1 == a.1 && point.0 >= x_min && point.0 <= x_max {
                return true;
            }
        }
    }
    
    point_in_polygon(polygon, point)
}

fn point_in_polygon(polygon: &Vec<(i64, i64)>, point: (i64, i64)) -> bool {
    let (px, py) = point;
    let mut inside = false;

    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if (yi > py) != (yj > py) && px < (xj - xi) * (py - yi) / (yj - yi) + xi {
            inside = !inside;
        }

        j = i;
    }

    inside
}