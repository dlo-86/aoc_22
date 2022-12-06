use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut vector = Vec::new();
    let mut sum = 0;
    let empty_string: String = String::from("");
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(colories) = line {
                if empty_string.eq(&colories) {
                    vector.push(sum);
                    sum = 0;
                } else {
                    sum += colories.parse::<i64>().unwrap();
                }
            }
        }
        vector.push(sum);
        vector.sort();
        vector.reverse();
        let top3 = &vector[0..3]; // not including 3
        let max = top3[0];
        println!("Max: {:?}", max);
        assert_eq!(max, 72602);
        println!("Top 3: {:?}", top3.iter().sum::<i64>());
        
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}