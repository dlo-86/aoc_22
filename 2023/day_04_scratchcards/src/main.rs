use std::{fs, collections::HashSet};

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);

    
}

fn part1(content: &String){
    let mut sum = 0;
    for line in content.split("\n"){
        let parts = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
        let having_numbers = parts[2].trim().split_whitespace().collect::<Vec<&str>>();
        let common_elements: Vec<_> = winning_numbers.iter().cloned().filter(|&x| having_numbers.contains(&x)).collect();
        let list_size: u32 = common_elements.len() as u32;
        if list_size > 0 {
            sum += 2_u32.pow(list_size - 1);
            
        }
    }
    println!("Part 1: {}", sum);
}
