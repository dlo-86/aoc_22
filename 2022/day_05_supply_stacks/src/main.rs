use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut first_line = true;
        let mut mode = "stack_init";
        let mut stacks:Vec<Vec<char>> = vec![];
        let mut num_stacks = 0;
        for line in lines {
            if let Ok(ok_line) = line {
                if first_line {
                    num_stacks = (ok_line.len() + 1) / 4;
                    for _i in 0..num_stacks {
                        stacks.push(vec![]);
                    }
                    first_line = false;
                } 
                if mode == "stack_init" {
                    if ok_line == "" {
                        mode = "stack_movement"
                    }
                    else {
                        for i in 0..num_stacks {
                            let stack_char = ok_line.chars().nth(i*4+1).unwrap();
                            if stack_char == ' ' || stack_char.is_ascii_digit() {continue;}
                            stacks[i].insert(0, ok_line.chars().nth(i*4+1).unwrap());
                        }
                    }
                }
                else {
                    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
                    for cap in re.captures_iter(&ok_line){
                        let amount = &cap[1].parse::<u32>().unwrap();
                        let from_stack = &cap[2].parse::<u32>().unwrap() - 1;
                        let to_stack = &cap[3].parse::<u32>().unwrap() - 1;
                        move_part2(&mut stacks, *amount, from_stack, to_stack);
                    }
                }
            }
        }
        for i in 0..stacks.len() {
            match stacks[i].last() {
                Some(value) => {println!("{}", value)} 
                None => {println!("ERROR")}
            }
            
        }
    }
}

fn move_part1(stacks: &mut Vec<Vec<char>>, amount: u32, from_stack: u32, to_stack: u32) {
    for _i in 0..amount {
        match stacks[from_stack as usize].pop() {
            Some(value) => {stacks[to_stack as usize].push(value); }
            None => {}
        }
    }
}

fn move_part2(stacks: &mut Vec<Vec<char>>, amount: u32, from_stack: u32, to_stack: u32) {
    let mut temp_vec: Vec<char> = vec![];
    for _i in 0..amount {
        match stacks[from_stack as usize].pop() {
            Some(value) => {temp_vec.insert(0, value); }
            None => {}
        }
    }
    stacks[to_stack as usize].append(&mut temp_vec);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}