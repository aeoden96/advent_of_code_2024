use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use utils::read_lines;

fn main() {
    let lines = read_lines("inputs/day02.txt");

    println!("Number of correct rows: {}", second_part(lines)); //326 //381
}

fn are_nums_valid(nums: Vec<i32>) -> bool {
    let max_diff = 3;
    let is_asc = nums[0] < nums[1]; // asc = true, desc = false

    return !nums.windows(2).any(|pair: &[i32]| {
        let diff: i32 = if is_asc {
            pair[1] - pair[0]
        } else {
            pair[0] - pair[1]
        };
        diff < 1 || diff > max_diff
    });
}

fn are_nums_valid_2(nums: Vec<i32>) -> bool { 
    for i in 0..(nums.len()) {
        let mut new_nums = nums.clone();
        new_nums.remove(i);

        if are_nums_valid(new_nums) {
            return true;
        }
    }

    return false;
}

fn line_to_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap()) // Convert to numbers
        .collect()
}

#[allow(dead_code)]
fn first_part(lines: Lines<BufReader<File>>) -> usize {
    lines
    .filter_map(|line_res| line_res.ok())
    .map(|line| line_to_nums(&line))
    .filter(|nums| are_nums_valid(nums.clone()))
    .count()
}



#[allow(dead_code)]
fn second_part(lines: Lines<BufReader<File>>) -> usize {
    lines
    .filter_map(|line_res| line_res.ok())
    .map(|line| line_to_nums(&line))
    .filter(|nums| are_nums_valid_2(nums.clone()))
    .count()
}
