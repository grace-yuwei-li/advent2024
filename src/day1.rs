use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let nums : Vec<_> = line.split_whitespace().collect();
        first_list.push(nums[0].parse().unwrap());
        second_list.push(nums[1].parse().unwrap());
    }
    (first_list, second_list)
}

pub fn day_1 () {
    let (mut first_list, mut second_list) = read_lines("./input/day1.txt");

    first_list.sort();
    second_list.sort();

    // Part 1
    let mut distance = 0;
    for (a, b) in first_list.iter().zip(second_list.iter()) {
        distance += (b - a).abs();
    }
    println!("Part 1: The total distance is {}.", distance);

    // Part 2
    let mut num_counts: HashMap<i32, i32> = HashMap::new();
    for a in second_list {
        // get value at a, if it doesn't exist set it to 0, then increment the value by 1
        *num_counts.entry(a).or_insert(0) += 1;
    }
    let mut similarity: i32 = 0;
    for a in first_list {
        similarity += a * num_counts.get(&a).unwrap_or(&0);
    }

    println!("Part 2: The total similarity is {}.", similarity);
}