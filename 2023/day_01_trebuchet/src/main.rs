use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);
    part2(content);
}

fn part1(lines: &String) {
    let mut numbers = Vec::new();

    for text_sequence in lines.split("\n") {
        let mut vector = Vec::new();
        for char in text_sequence.chars() {
            if char.is_digit(10) {
                vector.push(char)
            }
        }
        let mut number_string: String = String::from(vector.get(0).unwrap().to_string());
        number_string.push_str(&vector.last().unwrap().to_string());
        numbers.push(number_string.parse::<i32>().unwrap());
    }
    let sum: i32 = numbers.iter().sum();
    println!("Part 1: {:?}", sum)
}

fn part2(lines: String) {
    let mut numbers = Vec::new();

    for text_sequence in lines.split("\n") {
        let mut vector = Vec::new();
        for (i, char) in text_sequence.chars().enumerate() {
            let sub_string = &text_sequence[i..];
            if char.is_digit(10) {
                vector.push(char)
            }
            if sub_string.starts_with("one") {
                vector.push('1')
            }
            if sub_string.starts_with("two") {
                vector.push('2')
            }
            if sub_string.starts_with("three") {
                vector.push('3')
            }
            if sub_string.starts_with("four") {
                vector.push('4')
            }
            if sub_string.starts_with("five") {
                vector.push('5')
            }
            if sub_string.starts_with("six") {
                vector.push('6')
            }
            if sub_string.starts_with("seven") {
                vector.push('7')
            }
            if sub_string.starts_with("eight") {
                vector.push('8')
            }
            if sub_string.starts_with("nine") {
                vector.push('9')
            }
        }
        let mut number_string: String = String::from(vector.get(0).unwrap().to_string());
        number_string.push_str(&vector.last().unwrap().to_string());
        numbers.push(number_string.parse::<i32>().unwrap());
    }
    let sum: i32 = numbers.iter().sum();
    println!("Part 2: {:?}", sum)
}
