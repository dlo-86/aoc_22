use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use array_tool::vec::Intersect;


fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        let mut sum_badge = 0;
        let mut last_three: Vec<Vec<char>> = vec![];
        for line in lines {
            if let Ok(rucksack) = line {
                let (left_compartment, right_compartment) = rucksack.split_at(rucksack.len()/2);
                let intersect = left_compartment.chars().collect::<Vec<char>>().intersect(right_compartment.chars().collect::<Vec<char>>());
                for intersect_char in intersect {
                    sum += get_priority(intersect_char);
                }
                last_three.push(rucksack.chars().collect());
                if last_three.len() == 3 {
                    let first = &last_three[0];
                    let second = &last_three[1];
                    let third = &last_three[2];
                    let intersect_group_of_three = first.intersect(second.to_vec()).intersect(third.to_vec());
                    for intersect_char in intersect_group_of_three {
                        sum_badge += get_priority(intersect_char);
                    }
                    last_three.clear();
                }
            }
        }
        println!("Part 1: {:?}", sum);
        println!("Part 2: {:?}", sum_badge);
    }
}

fn get_priority(intersect_char: char) -> u32
{
    let value = intersect_char as u32;
    if value > 96 {
        return value - 96;
    } else {
        return value - 38;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}