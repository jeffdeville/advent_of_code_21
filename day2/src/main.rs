use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("day2/input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let directions: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let (horizontal_pos, depth) = determine_position_1(&directions);
    println!("Method1: Forward: {}, Depth: {} - Total: {}", horizontal_pos, depth, horizontal_pos * depth);

    let (horizontal_pos, depth) = determine_position_2(&directions);
    println!("Method2: Forward: {}, Depth: {} - Total: {}", horizontal_pos, depth, horizontal_pos * depth);

}

fn determine_position_1(directions: &Vec<String>) -> (i32, i32) {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for direction in directions {
        let v: Vec<&str> = direction.split(' ').collect();
        match &v[..] {
            ["forward", distance] => { horizontal_pos += distance.parse::<i32>().unwrap(); },
            ["down", distance] => { depth += distance.parse::<i32>().unwrap(); },
            ["up", distance] => { depth -= distance.parse::<i32>().unwrap(); },
            other => println!("{:?}", other)
        }
    }
    (horizontal_pos, depth)
}

fn determine_position_2(directions: &Vec<String>) -> (i32, i32) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim: i32 = 0;

    for direction in directions {
        let v: Vec<&str> = direction.split(' ').collect();
        let combo: (&str, i32) = (v[0], v[1].parse::<i32>().unwrap());
        match &combo {
            ("forward", distance) => {
                horizontal_pos += distance;
                depth += aim * distance;
            },
            ("down", distance) => {
                aim += distance;
            },
            ("up", distance) => aim -= distance,
            other => println!("{:?}", other)
        }
        // println!("{:?}: Horizontal: {}, Depth: {}, Aim: {}", combo, horizontal_pos, depth, aim);
    }
    (horizontal_pos, depth)
}

#[test]
fn test_directions_1() {
    let directions = vec![
        "forward 1".to_string(),
        "down 9".to_string(),
        "up 1".to_string(),
        "forward 3".to_string()
        ];
    assert_eq!(determine_position_1(&directions), (4, 8));
}

#[test]
fn test_directions_2() {
    let directions = vec![
        "forward 5".to_string(),
        "down 5".to_string(),
        "forward 8".to_string(),
        "up 3".to_string(),
        "down 8".to_string(),
        "forward 2".to_string()
    ];
    assert_eq!(determine_position_2(&directions), (15, 60));
}
