use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;



fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ok_line) = line {
                println!("Part 1: {}", get_marker_position(&ok_line, 4));
                println!("Part 2: {}", get_marker_position(&ok_line, 14));
            }

        }
    }
}

fn get_marker_position(line: &String, marker_length: usize) -> usize{
    let char_array: Vec<char> = line.chars().collect();
    for i in 0..char_array.len()-marker_length {
        let slize = &char_array[i..(i+marker_length)];
        let set: HashSet<char> = HashSet::from_iter(slize.iter().cloned());
        if set.len() == marker_length {
            return i + marker_length;
        }
    }
    return 0;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}