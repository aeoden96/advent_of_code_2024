use utils::read_lines;
use std::iter::zip;
use std::collections::HashMap;
use std::io::Lines;
use std::io::BufReader;
use std::fs::File;
fn main() {
    let file = read_lines("inputs/day01.txt");

    let lines = match file {
        Ok(file) => file,
        Err(e) => panic!("Error reading file: {}", e),
    };

    second_part(lines);
}


fn first_part(lines: Lines<BufReader<File>>) {
    let mut first_column: Vec<i32> = vec![];
    let mut second_column: Vec<i32> = vec![];

    for line_res in lines {
        if let Ok(line) = line_res {
            let nums: Vec<i32> = line
                .split_whitespace() // Split by spaces
                .map(|s| s.parse().unwrap()) // Convert to numbers
                .collect();

            first_column.push(nums[0]);
            second_column.push(nums[1]);
        }
    }

    first_column.sort();
    second_column.sort();

    let  mut sum = 0;

    for  (i,j) in zip(first_column, second_column) {
        sum += i32::abs(i-j);
    }

    println!("Sum: {}", sum);

}


fn second_part(lines: Lines<BufReader<File>>) {
    let mut occurances = HashMap::new();

    let mut first_column: Vec<i32> = vec![];

    for line_res in lines {
        if let Ok(line) = line_res {
            let nums: Vec<i32> = line
                .split_whitespace() // Split by spaces
                .map(|s| s.parse().unwrap()) // Convert to numbers
                .collect();
                    first_column.push(nums[0]);
                let second_num = nums[1];

                if occurances.contains_key(&second_num) {
                    occurances.insert(second_num, occurances.get(&second_num).unwrap() + 1);
                } else {
                    occurances.insert(second_num, 1);
                }
        }
    }

    let mut sum = 0;
    for i in first_column {
        if occurances.contains_key(&i) {
            sum += occurances.get(&i).unwrap() * i;
        }
    }

    println!("Sum: {}", sum);


}