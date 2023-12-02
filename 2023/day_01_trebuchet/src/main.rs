use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        part1(lines);
        part2(lines);
    }
}

fn part1(lines: io::Lines<io::BufReader<File>>) {
    let mut numbers = Vec::new();

    for line in lines {
    
        let mut vector = Vec::new();
        if let Ok(text_sequence) = line {
            for char in text_sequence.chars() {
                if char.is_digit(10) {
                    vector.push(char)
                }
            }
            let mut number_string: String = String::from(vector.get(0).unwrap().to_string());
            number_string.push_str(&vector.last().unwrap().to_string());
            numbers.push(number_string.parse::<i32>().unwrap());
        }
    }
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {:?}", sum)
}

fn part2(lines: io::Lines<io::BufReader<File>>) {
    let mut numbers = Vec::new();

    for line in lines {
    
        let mut vector = Vec::new();
        if let Ok(text_sequence) = line {
            for i, char in text_sequence.chars().enumerate() {
                if char.is_digit(10) {
                    vector.push(char)
                }
                else {
                    
                }
            }
            let mut number_string: String = String::from(vector.get(0).unwrap().to_string());
            number_string.push_str(&vector.last().unwrap().to_string());
            numbers.push(number_string.parse::<i32>().unwrap());
        }
    }
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {:?}", sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}