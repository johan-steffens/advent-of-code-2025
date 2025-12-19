use std::{collections::{HashMap, VecDeque}, fs};

use crate::shared::{Part};

const PATH_PREFIX: &str = "data/day10/";

#[derive(Debug, Clone)]
struct Combination {
    diagram: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltage: Vec<u64>,
}

pub fn run(part: Part, example: bool) {
    println!("day ten");

    let input = fs::read_to_string(format!("{}{}", PATH_PREFIX, if example { "example.txt" } else { "actual.txt" })).unwrap();
    
    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input),
    }
}

fn part_one(input: String) {
    println!("part one");

    println!("input: {}", input);

    let combinations = input.lines()
        .map(|line| {
            let diagram_segment = &line.to_string()[line.find("[").unwrap() + 1..line.find("] (").unwrap()];
            let buttons_segment = &line.to_string()[line.find("(").unwrap() + 1..line.find(") {").unwrap()];
            let joltage_segment = &line.to_string()[line.find("{").unwrap() + 1..line.find("}").unwrap()];

            let diagram = diagram_segment.chars()
                .map(|c| c == '#')
                .collect::<Vec<bool>>();

            let buttons = buttons_segment.replace(") (", ";")
                .split(";")
                .map(|button| {
                    button.split(",")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>();

            let joltage = joltage_segment.split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            Combination { diagram, buttons, joltage }
        })
        .collect::<Vec<Combination>>();

    let mut total_button_presses = 0;
    for combination in combinations.iter() {
        total_button_presses += calculate_lowest_button_press(combination);
    }

    println!("total button presses: {}", total_button_presses);
}

fn calculate_lowest_button_press(combination: &Combination) -> u64 {
    let num_lights = combination.diagram.len();
    let num_buttons = combination.buttons.len();
    
    let target_state = combination.diagram.clone();
    
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    
    // Start with all lights off
    let initial_state = vec![false; num_lights];
    queue.push_back((initial_state.clone(), 0u64));
    visited.insert(initial_state, 0);
    
    let mut min_presses = u64::MAX;
    
    while let Some((current_state, presses)) = queue.pop_front() {
        // Check if we've reached the target
        if current_state == target_state {
            min_presses = min_presses.min(presses);
            continue;
        }
        
        // Prune if we already have a better solution
        if presses >= min_presses {
            continue;
        }
        
        // Try pressing each button
        for button_idx in 0..num_buttons {
            let mut new_state = current_state.clone();
            
            // This button affects all lights mentioned in its combination
            for &light_idx in &combination.buttons[button_idx] {
                if (light_idx as usize) < num_lights {
                    new_state[light_idx as usize] = !new_state[light_idx as usize];
                }
            }
            
            let new_presses = presses + 1;
            
            // Only explore if we haven't seen this state with fewer presses
            if !visited.contains_key(&new_state) || visited[&new_state] > new_presses {
                visited.insert(new_state.clone(), new_presses);
                queue.push_back((new_state, new_presses));
            }
        }
    }
    
    if min_presses == u64::MAX { 0 } else { min_presses }
}

fn part_two(input: String) {
    println!("part two");

    println!("input: {}", input);

    
}
