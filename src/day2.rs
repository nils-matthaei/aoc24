use std::collections::HashSet;

use crate::common::read_lines;

const SAMPLE_PATH: &str = "./input/day2/sample.txt";
const INPUT_PATH: &str = "./input/day2/input.txt";

fn check_direction(prev: &i32, current: &i32, is_increasing: &bool) -> bool {
    let direction = *prev < *current;
    return direction == *is_increasing;
}

fn check_difference(x: &i32, y: &i32) -> bool {
    let abs_diff = (x - y).abs();
    if abs_diff < 1 || abs_diff > 3 {
        return false;
    }
    return true;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut result: bool = true;
    if report.len() < 2 {
        return false;
    };

    let first_level = report[0];
    let second_level = report[1];

    if !check_difference(&first_level, &second_level) {
        return false;
    }

    let is_increasing: bool = first_level < second_level;

    let mut prev_level = second_level;
    for i in 2..report.len() {
        let current_level = report[i];
        if !check_difference(&prev_level, &current_level)
            || !check_direction(&prev_level, &current_level, &is_increasing)
        {
            result = false;
            break;
        }
        prev_level = current_level;
    }

    return result;
}

fn is_safe_dampener(report: &Vec<i32>) -> bool {
    let mut faulty_levels = HashSet::new();

    if report.len() < 2 {
        return false;
    };

    let first_level = report[0];
    let second_level = report[1];

    if !check_difference(&first_level, &second_level) {
        faulty_levels.insert(0);
        faulty_levels.insert(1);
    }

    let is_increasing: bool = first_level < second_level;

    let mut prev_level = second_level;
    for i in 2..report.len() {
        let current_level = report[i];
        if !check_difference(&prev_level, &current_level)
            || !check_direction(&prev_level, &current_level, &is_increasing)
        {
            faulty_levels.insert(i);
            faulty_levels.insert(i - 1);
        }
        prev_level = current_level;
    }

    if faulty_levels.len() == 0 {
        return true;
    }

    let mut damp_report: Vec<i32>;
    // iterating over faulty_levels does not work for some reason it misses two cases, unsure why
    for i in 0..report.len() {
        damp_report = report.to_vec();
        damp_report.remove(i);
        if is_safe(&damp_report) {
            return true;
        }
    }
    return false;
}

pub fn solve() {
    let mut safe_ctr: i32 = 0;
    let mut safe_ctr_damp: i32 = 0;

    if let Ok(lines) = read_lines(INPUT_PATH) {
        for line in lines.flatten() {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            if is_safe(&report) {
                safe_ctr += 1;
            }
            if is_safe_dampener(&report) {
                safe_ctr_damp += 1;
            }
        }

        println!("Safety: {}", safe_ctr);
        println!("Safety (with dampener): {}", safe_ctr_damp);
    }
}
