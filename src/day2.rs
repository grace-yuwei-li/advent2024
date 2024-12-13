use std::fs::read_to_string;

fn parse_lines(filename: &str) -> Vec<Vec<i32>> {
    let mut levels: Vec<Vec<i32>> = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let nums : Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        levels.push(nums);
    }
    levels
}

fn check_signs(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|&x| x == -1 || x == -2 || x == -3) || 
    diffs.iter().all(|&x| x == 1 || x == 2 || x == 3)
}

fn check_pos_trend(diffs: &Vec<i32>) -> bool {
    let mut positive_count = 0;
    let mut negative_count = 0;
    
    for &num in diffs.iter() {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        }
    }
    positive_count >= negative_count
}

fn calculate_diffs(line: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = vec![];
    let mut i = 0;
    while i < line.len()  - 1 {
        let diff = &line[i + 1] - &line[i];
        diffs.push(diff);
        i += 1;
    }
    diffs
}

fn pos_out_of_bounds(a: i32) -> bool {
    a < 1 || a > 3
}

fn neg_out_of_bounds(a: i32) -> bool {
   a < -3 || a > -1
}

fn part_1(lines: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for line in lines {
        let mut safe = true;
        let mut i = 0;
        let diffs = calculate_diffs(&line);
        while i < line.len()  - 1 {
            if check_signs(&diffs) {
                i += 1;
            } else {
                safe = false;
                break;
            }
        }
        if safe {
            result += 1;
        }
    }
    result
}

fn part_2(lines: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for line in lines {
        let diffs = calculate_diffs(&line);
        let positive = check_pos_trend(&diffs);

        let mut temp_vec: Vec<i32> = vec![];
        if check_signs(&diffs) {
            result += 1;
            } else {
                let mut j = 0;
                let mut tolerance = true;
            while j < diffs.len() {
                    if (positive && pos_out_of_bounds(diffs[j]) || !positive && neg_out_of_bounds(diffs[j])) && tolerance  {
                        tolerance = false;
                        if j == 0 {
                            if positive && pos_out_of_bounds(diffs[j + 1]) {
                                temp_vec.push(diffs[j] + diffs[j + 1]);
                            }
                            if !positive && neg_out_of_bounds(diffs[j + 1]) {
                                temp_vec.push(diffs[j] + diffs[j + 1]);
                            }
                            j += 2;
                            continue;
                        }
                        if j == diffs.len() - 1 {
                            break;
                        }
                        temp_vec.push(diffs[j] + diffs[j + 1]);
                        j += 2;
                    } else {
                        temp_vec.push(diffs[j]);
                        j += 1;
                        continue;
                    }
                }
            if check_signs(&temp_vec) || tolerance {
                result += 1;
            }
        }
    }
    result
}

pub fn day_2 () {
    let lines = parse_lines("./input/day2.txt");
    let result1 = part_1(&lines);
    let result2 = part_2(&lines);

    println!("Part 1: {} reports are safe.", result1);
    println!("Part 2: {} reports are safe.", result2);
}

