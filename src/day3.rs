use crate::common::read_lines;
use regex::Regex;

const SAMPLE_PATH: &str = "./input/day3/sample.txt";
const SAMPLE_PATH2: &str = "./input/day3/sample2.txt";
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

const PATTERN_DO: &str = r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)";

fn there_is_no_try(line: &str, global_active: bool) -> (i32, bool) {
    let mut sum = 0;
    let mut active = global_active;
    let re = Regex::new(PATTERN_DO).unwrap();
    for caps in re.captures_iter(line) {
        match &caps[0] {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    let first: i32 = caps[1].parse::<i32>().unwrap();
                    let second: i32 = caps[2].parse::<i32>().unwrap();
                    sum += first * second;
                }
            }
        }
    }
    return (sum, active);
}

pub fn solve() {
    let mut sum = 0;
    let mut global_active = true;
    if let Ok(lines) = read_lines(INPUT_PATH) {
        for line in lines.flatten() {
            //result += execute_multiplication(&line);
            let result = there_is_no_try(&line, global_active);
            sum += result.0;
            global_active = result.1;
        }
    }
    println!("Multiplication Result: {}", sum);
}
