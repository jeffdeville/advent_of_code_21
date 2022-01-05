use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("day1/input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let sonar_readings: Vec<i32> = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let num_deeper_measurements = count_num_deepers(&sonar_readings, 2);
    let num_deeper_measurements_4 = count_num_deepers(&sonar_readings, 4);

    println!("Total Window 2: {}", num_deeper_measurements);
    println!("Total Window 4: {}", num_deeper_measurements_4);
}

fn count_num_deepers(input: &Vec<i32>, window_size: usize) -> u32
{
    input.windows(window_size).fold(0, |acc, window| {
        let prev = window[0..window_size-1].iter().sum::<i32>();
        let curr = window[1..window_size].iter().sum::<i32>();
        if curr > prev { acc + 1 } else { acc }
    })
}

#[test]
fn test_count_num_deepers_with_window_2() {
    let input = vec![1, 2, 3];
    assert_eq!(count_num_deepers(&input, 2), 2);

    let input = vec![3, 2, 1];
    assert_eq!(count_num_deepers(&input, 2), 0);

    let input = vec![3, 3, 3];
    assert_eq!(count_num_deepers(&input, 2), 0);

    let input = vec![3, 2, 3, 4, 5];
    assert_eq!(count_num_deepers(&input, 2), 3);
}

#[test]
fn test_count_num_deepers_with_window_4() {
    let input = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];
    assert_eq!(count_num_deepers(&input, 4), 5);
}
