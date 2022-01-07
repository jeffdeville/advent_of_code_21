use itertools::Itertools;
use hashbrown::HashMap;
fn main() {
    // ✅ for each line, split on the arrow
    // ✅ convert to 2 tuples
    // ✅ filter the list to just include tuples with a shared element
    // create a range between the other elements
    // increment counts in the grid
    // find all grid slots w/ values >= 2
    let lines = include_str!("../input.txt");
    let valid_vent_lines  = lines
        .trim()
        .split("\n")
        .filter_map(|line|
            line.split(" -> ")
                .map(|x| x.split(","))
                .flatten()
                .map(|x| x.parse().unwrap())
                .collect_tuple()
        )
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect::<Vec<_>>();
    let answer = num_overlapping(valid_vent_lines.iter().copied());

    println!("Num overlaps {}", answer);
}

 fn num_overlapping(lines: impl Iterator<Item=(i32, i32, i32, i32)>) -> usize {
    let mut points = HashMap::new();
    lines.for_each(|(x1, y1, x2, y2)| {
        if x1 == x2 {
            for y in y1..y2+1 {
                *points.entry((x1, y)).or_insert(0) += 1;
            }
        } else {
            for x in x1..x2+1 {
                *points.entry((x, y1)).or_insert(0) += 1;
            }
        }
    });
    points.values().filter(|&&x| x >= 2).count()
 }
