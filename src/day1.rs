use crate::common::read_lines;

const SAMPLE_PATH: &str = "./input/day1/riddle1.txt";

pub fn solve() {
    let mut left_ids: Vec<i32> = vec![];
    let mut right_ids: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(SAMPLE_PATH) {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            left_ids.push(numbers[0]);
            right_ids.push(numbers[1]);
        }

        left_ids.sort();
        right_ids.sort();

        let mut dist: i32 = 0;
        for i in 0..left_ids.len() {
            dist += (left_ids[i] - right_ids[i]).abs();
        }

        println!("{}", dist);
    }
}
