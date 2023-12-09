use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let mut sum = 0;
    for line in content.split("\n") {
        let parts = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
        let having_numbers = parts[2].trim().split_whitespace().collect::<Vec<&str>>();
        let common_elements: Vec<_> = winning_numbers
            .iter()
            .cloned()
            .filter(|&x| having_numbers.contains(&x))
            .collect();
        let list_size: u32 = common_elements.len() as u32;
        if list_size > 0 {
            sum += 2_u32.pow(list_size - 1);
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(content: &String) {
    let mut sum = 0;
    let count_amounts = content.split("\n").collect::<Vec<&str>>().len();
    let mut counts = vec![1; count_amounts];
    for (i, line) in content.split("\n").enumerate() {
        let parts = line.split([':', '|']).collect::<Vec<&str>>();
        let winning_numbers = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
        let having_numbers = parts[2].trim().split_whitespace().collect::<Vec<&str>>();
        let common_elements: Vec<_> = winning_numbers
            .iter()
            .cloned()
            .filter(|&x| having_numbers.contains(&x))
            .collect();
        let list_size = common_elements.len();
        for _count in 0..counts[i] {
            for increase_number in i + 1..i + 1 + list_size {
                if increase_number < count_amounts {
                    counts[increase_number] += 1;
                }
            }
        }
    }
    for count in counts {
        sum += count;
    }
    println!("Part 2: {}", sum);
}
