use crate::common::read_lines;
use regex::Regex;

const SAMPLE_PATH: &str = "./input/day3/sample.txt";
const INPUT_PATH: &str = "./input/day3/input.txt";

const PATTERN: &str = r"mul\((\d+),(\d+)\)";

fn execute_multiplication(line: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(PATTERN).unwrap();
    for caps in re.captures_iter(line) {
        let first: i32 = caps[1].parse::<i32>().unwrap();
        let second: i32 = caps[2].parse::<i32>().unwrap();
        sum += first * second;
    }
    return sum;
}

pub fn solve() {
    let mut result = 0;
    if let Ok(lines) = read_lines(INPUT_PATH) {
        for line in lines.flatten() {
            result += execute_multiplication(&line);
        }
    }
    println!("Multiplication Result: {}", result);
}
