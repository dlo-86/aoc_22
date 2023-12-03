use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);
    part2(&content);
}

fn part1(content: &String) {
    let mut sum = 0;
    for line in content.split("\n") {
        let line_parts: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let game = line_parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let cube_rounds = line_parts[1];
        let mut skip: bool = false;
        for cube_round in cube_rounds.split(";") {
            for cube in cube_round.split(",") {
                let cube_parts = cube.trim().split(" ").collect::<Vec<&str>>();
                let amount = cube_parts[0].parse::<u32>().expect("Expected a number");
                let colour = cube_parts[1];

                if colour == "green" && amount > 13
                    || colour == "red" && amount > 12
                    || colour == "blue" && amount > 14
                {
                    skip = true;
                    break;
                }
            }
            if skip == true {
                break;
            }
        }
        if !skip {
            sum += game;
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(content: &String) {
    let mut sum = 0;
    for line in content.split("\n") {
        let line_parts: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let game = line_parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let cube_rounds = line_parts[1];
        let mut max_red: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;
        for cube_round in cube_rounds.split(";") {
            for cube in cube_round.split(",") {
                let cube_parts = cube.trim().split(" ").collect::<Vec<&str>>();
                let amount = cube_parts[0].parse::<u32>().expect("Expected a number");
                let colour = cube_parts[1];
                if colour == "green" && amount > max_green {
                    max_green = amount;
                }
                if colour == "blue" && amount > max_blue {
                    max_blue = amount;
                }
                if colour == "red" && amount > max_red {
                    max_red = amount;
                }
            }
        }

        sum += max_blue * max_red * max_green;
    }
    println!("Part 2: {}", sum);
}
