use std::fs;
use std::io::{self, BufRead, BufReader};

// Read entire file as a string
 pub fn read_input(s: &str) -> String {
    fs::read_to_string(s).expect("Could not read file")
}

// Read file line by line
pub fn read_lines(s: &str) -> io::Lines<BufReader<fs::File>>
{
    let file = fs::File::open(s).unwrap();
    BufReader::new(file).lines()
}

/*     let file =  read_lines("inputs/day01.txt");

    let lines = match file {
        Ok(file) => file,
        Err(e) => panic!("Error reading file: {}", e),
    };

   
    for line_res in lines {
        if let Ok(line) = line_res {
            println!("Line: {}", line);
        } */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("inputs/day01.txt");
        assert_eq!(input.len(), 100);
    }

    #[test]
    fn test_read_lines() {
        let lines = read_lines("inputs/day01.txt");
        let vec: Vec<String> = lines.map(|l| l.unwrap()).collect();
        assert_eq!(vec.len(), 100);
    }
}
