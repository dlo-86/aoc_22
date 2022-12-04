use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut my_sum = 0;
        let mut my_sum_part2 = 0;
        for line in lines {
            if let Ok(segments) = line {
                let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
                for cap in re.captures_iter(&segments) {
                    let seg1_start = &cap[1].parse::<u32>().unwrap();
                    let seg1_end = &cap[2].parse::<u32>().unwrap();
                    let seg2_start = &cap[3].parse::<u32>().unwrap();
                    let seg2_end = &cap[4].parse::<u32>().unwrap();
                    if (seg1_start >= seg2_start && seg1_end <= seg2_end) || (seg2_start >= seg1_start && seg2_end <= seg1_end) {
                        my_sum += 1;
                    }
                    if in_range(seg1_start, seg2_start, seg2_end) ||
                       in_range(seg1_end, seg2_start,   seg2_end) ||
                       in_range(seg2_start, seg1_start,  seg1_end) ||
                       in_range(seg2_end, seg1_start,   seg1_end)
                    {
                        my_sum_part2 += 1;
                    }
                }
            }
        }
        println!("{:?}", my_sum);
        println!("{:?}", my_sum_part2);
    }
}

fn in_range(value: &u32, range_start: &u32, range_end: &u32) -> bool {
    return range_start <= value && value <= range_end
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}