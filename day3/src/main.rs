use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let file = File::open("day3/input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let readings: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    println!("Power Level: {}", get_power_level(&readings));
    let o2_gen_rating = get_o2_generator_ratings(&readings);
    println!("O2 Gen Rating: {}", o2_gen_rating);
}

fn convert_to_decimal(input: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    for i in input { result = result * 2 + (*i as u32); }
    result
}

fn sum_col_at_index(readings: &Vec<String>, col: usize) -> u8 {
    let (vals, num_lines) = sum_cols_vertically(readings);

    if vals[col] as f32 / num_lines as f32 >= 0.5 { 1 } else { 0 }
}

fn sum_cols_vertically(readings: &Vec<String>) -> (Vec<i32>, u32) {
    // Make a Vec<i32> and then sum the digits in the whole list, and then if
    // the average is greater than 0.5, use 1, or less than that, and use 0

    let mut vals: Vec<i32> = vec![0; readings.first().unwrap().len()];
    let mut num_lines = 0;
    for line in readings {
        num_lines += 1;
        // skip/take would work, but that's awful...
        for (idx, val) in line.char_indices() {
            let val = if val == '1' { 1 } else { 0 };
            vals[idx] += val;
        }
    }
    (vals, num_lines)
}

fn calc_most_popular_bit(vals: &Vec<i32>, num_lines: u32) -> Vec<u8> {
    vals.iter()
        .map(|x| if *x as f32 / num_lines as f32 >= 0.5 { 1 } else { 0 })
        .collect::<Vec<u8>>()
}

fn flip_bits(vals: &Vec<u8>) -> Vec<u8> {
    vals.iter()
        .map(|val| if *val == 0 { 1 } else { 0 } )
        .collect()
}

fn get_power_level(readings: &Vec<String>) -> u32 {
    let (vals, num_lines) = sum_cols_vertically(readings);
    let gamma_bit_vec = calc_most_popular_bit(&vals, num_lines);
    let gamma_dec = convert_to_decimal(&gamma_bit_vec);
    let epsilon = convert_to_decimal(&flip_bits(&gamma_bit_vec));
    gamma_dec * epsilon
}

fn get_o2_generator_ratings(readings: &Vec<String>) -> i32 {
    // Keep track of current bit to examine
    // go through, flat map a length 1 slick at that bit
    // sum
    // divide by len of that vec
    // figure 0 or 1
    // filter list
    // if length == 1,
    //      convert to decimal
    // if length > 1
    //      next bit
    //      loop

    let mut curr_bit = 0;
    let mut filtered_list  = readings
        .iter()
        .map(|line| line.chars().map(|ch| if ch == '1' { 1 } else { 0 }).collect())
        .collect::<Vec<Vec<u8>>>();

    while filtered_list.len() > 1 {
        let sum_vals_in_col = filtered_list
            .iter()
            .fold(0, |acc, vec| acc + (*vec.get(curr_bit).unwrap() as u32));
        let target_bit: u8 = if sum_vals_in_col as f32 / filtered_list.len() as f32 >= 0.5 { 1 } else { 0 };
        // I'm stuck here because I can't repeatedly filter into the same variable,
        // because something about "filter" returns references to entries, rather than the entries themselves
        filtered_list = filtered_list
            .iter()
            .filter(|vec| {
                (*vec).get(curr_bit).unwrap() == &target_bit
            })
            .map(|line| line.clone())
            .collect::<Vec<Vec<u8>>>();

        curr_bit += 1;
    }
    println!("{:?}", filtered_list);
    23
    // convert_to_decimal(filtered_list.first())
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
