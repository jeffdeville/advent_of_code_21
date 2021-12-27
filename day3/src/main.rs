use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let file = File::open("day3/input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let readings: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    println!("Power Level: {}", get_power_level(&readings));
}

fn convert_to_decimal(input: &Vec<u8>) -> i32 {
    let mut result: i32 = 0;
    for i in input {
        result = result * 2 + (*i as i32);
    }
    result
}

fn sum_cols_vertically(readings: &Vec<String>) -> (Vec<i32>, u32) {
    // Make a Vec<i32> and then sum the digits in the whole list, and then if
    // the average is greater than 0.5, use 1, or less than that, and use 0
    let mut vals: Vec<i32> = vec![0; 12];
    let mut num_lines = 0;
    for line in readings {
        num_lines += 1;
        for (idx, val) in line.char_indices() {
            let val = if val == '1' { 1 } else { 0 };
            vals[idx] += val;
        }
    }
    (vals, num_lines)
}

fn calc_most_popular_bit(vals: &Vec<i32>, num_lines: i32) -> Vec<u8> {
    vals.iter()
        .map(|x| if *x as f32 / num_lines as f32 > 0.5 { 1 } else { 0 })
        .collect::<Vec<u8>>()
}

fn get_power_level(readings: &Vec<String>) -> i32 {
    let (vals, num_lines) = sum_cols_vertically(readings);
    let gamma = calc_most_popular_bit(&vals, num_lines);
    // let gamma = vals.iter()
    //     .map(|x| if *x as f32 / num_lines as f32 > 0.5 { 1 } else { 0 })
    //     .collect::<Vec<i32>>();
    let epsilon = gamma.clone().iter()
        .map(|x| (x - 1).abs())
        .collect::<Vec<u8>>();
    convert_to_decimal(&epsilon) * convert_to_decimal(&gamma)
}

#[test]
fn test_convert_to_decimal() {
    assert_eq!(convert_to_decimal(&vec![1,0,1,1,1,0,1,0,0,1,0,1]), 2981);
    assert_eq!(convert_to_decimal(&vec![1,1,1,1,1,1,1,1,1,1,0,0]), 4092);
}

#[test]
fn test_get_power_level() {
    let input = vec![
        "000000000000".to_string(),
        "000000000011".to_string(),
        "000000000011".to_string(),
    ];
    // Should result in 0011 == 3 and 111111111100 == 4092, multiplied to get 12276
    assert_eq!(get_power_level(&input), 12276);
}
