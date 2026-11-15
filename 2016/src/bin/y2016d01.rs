use std::{collections::HashSet, fs};

fn main() {
    let raw_directions = get_input();
    let instructions = parse(&raw_directions);

    println!("Part 1: {}", part1(&instructions));
    match part2(&instructions) {
        Some(distance) => println!("Part 2: {distance}"),
        None => println!("Part 2: no location visited twice"),
    }
}

fn parse(input: &str) -> Vec<(&str, i32)> {
    let directions = input.split(",").map(|d| d.trim());
    let mut instructions = Vec::new();
    for direction in directions {
        let (rotation, distance) = direction.split_at(1);
        instructions.push((rotation, distance.parse::<i32>().expect("Unable to parse")));
    }
    instructions
}

fn part1(instructions: &[(&str, i32)]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_facing = 0;

    for &(rotation, distance) in instructions {
        current_facing = get_direction(rotation, current_facing);
        match current_facing {
            0 => y += distance,
            1 => x += distance,
            2 => y -= distance,
            3 => x -= distance,
            _ => println!("Unexpected result"),
        }
    }
    x.abs() + y.abs()
}

fn part2(instructions: &[(&str, i32)]) -> Option<i32> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_facing = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for &(rotation, distance) in instructions {
        current_facing = get_direction(rotation, current_facing);
        for _ in 1..distance + 1 {
            match current_facing {
                0 => y += 1,
                1 => x += 1,
                2 => y -= 1,
                3 => x -= 1,
                _ => println!("Unexpected result"),
            }
            if !visited.insert((x, y)) {
                return Some(x.abs() + y.abs());
            }
        }
    }
    None
}

fn get_input() -> String {
    fs::read_to_string("inputs/day01.txt").expect("Could not read input")
}

fn get_direction(rotation: &str, current_direction: i8) -> i8 {
    if rotation == "R" {
        return (current_direction + 1) % 4;
    }
    (current_direction + 3) % 4
}
