use std::fs;

fn main() {
    let raw_directions = get_input();
    let directions = raw_directions.split(",").map(|d| d.trim());
    let mut instructions = Vec::new();
    for direction in directions {
        let (rotation, distance) = direction.split_at(1);
        instructions.push((rotation, distance.parse::<i32>().expect("Unable to parse")));
    }
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut current_facing = 0;

    for instruction in instructions {
        let (rotation, distance) = instruction;
        current_facing = get_direction(rotation, current_facing);
        match current_facing {
            0 => y += distance,
            1 => x += distance,
            2 => y -= distance,
            3 => x -= distance,
            _ => println!("Unexpected result"),
        }
    }

    println!("{}", x.abs() + y.abs())
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
