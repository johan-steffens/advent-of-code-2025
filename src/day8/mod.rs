use std::{collections::HashMap, fs};

use glam::Vec3;
use union_find::{QuickUnionUf, UnionByRank, UnionFind};

use crate::shared::{Part};

#[derive(Debug, Clone, Copy)]
struct Junction {
    id: usize,
    position: Vec3,
}

#[derive(Debug, Clone, Copy)]
struct Connection {
    first: Junction,
    second: Junction,
    distance: f32,
}

const PATH_PREFIX: &str = "data/day8/";

pub fn run(part: Part, example: bool) {
    println!("day seven");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    let connections = match example {
        true => 10,
        false => 1000,
    };
    
    match part {
        Part::One => part_one(input, connections),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String, number_of_connections: usize) {
    println!("part one");

    println!("input: {}", input);

        let mut junctions: Vec<Junction> = Vec::new();
        for i in 0..input.lines().count() {
            let line = input.lines().nth(i).unwrap();
            let values: Vec<f32> = line.split(",")
                .map(|value| value.parse::<f32>().unwrap())
                .collect();

            assert_eq!(values.iter().count(), 3);

            let position = Vec3::new(values[0], values[1], values[2]);
            junctions.push(Junction { id: i, position });
        }

    let mut pairs: Vec<Connection> = Vec::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let first = junctions[i];
            let second = junctions[j];
            let distance = (first.position - second.position).length();
            pairs.push(Connection { first, second, distance });
        }
    }

    pairs.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut union = QuickUnionUf::<UnionByRank>::new(junctions.len());

    let k = number_of_connections.min(pairs.len());
    for i in 0..k {
        let pair = pairs[i];
        union.union(pair.first.id, pair.second.id);
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for id in 0..junctions.len() {
        let root = union.find(id);
        *counts.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = counts.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    println!("total: {}", sizes[0] * sizes[1] * sizes[2]);
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    let mut junctions: Vec<Junction> = Vec::new();
    for i in 0..input.lines().count() {
        let line = input.lines().nth(i).unwrap();
        let values: Vec<f32> = line.split(",")
            .map(|value| value.parse::<f32>().unwrap())
            .collect();

        assert_eq!(values.iter().count(), 3);

        let position = Vec3::new(values[0], values[1], values[2]);
        junctions.push(Junction { id: i, position });
    }

    let mut pairs: Vec<Connection> = Vec::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let first = junctions[i];
            let second = junctions[j];
            let distance = (first.position - second.position).length();
            pairs.push(Connection { first, second, distance });
        }
    }

    pairs.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut union = QuickUnionUf::<UnionByRank>::new(junctions.len());

    let mut components = junctions.len();
    for pair in pairs {
        let r1 = union.find(pair.first.id);
        let r2 = union.find(pair.second.id);

        if r1 != r2 {
            union.union(pair.first.id, pair.second.id);
            components -= 1;

            if components == 1 {
                let product = pair.first.position.x * pair.second.position.x;
                println!("total: {}", product);
                break;
            }
        }
    }
}