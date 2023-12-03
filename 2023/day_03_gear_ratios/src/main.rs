use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);
    // part2(&content);
}

fn check_symbol(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let no_symbol_list = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '\n'];
    if no_symbol_list.contains(&map[x][y]) {
        return false;
    }
    return true;
}

fn check_if_near_symbol(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut x_lower_bound: usize = 0;
    if x > 1 {
        x_lower_bound = x - 1;
    }
    let mut y_lower_bound: usize = 0;
    if y > 1 {
        y_lower_bound = y - 1;
    }
    for x_ref in x_lower_bound..x + 2 {
        for y_ref in y_lower_bound..y + 2 {
            // println!("{}, {}", x_ref, y_ref);
            if x_ref < map.len() && y_ref < map[0].len() {
                if check_symbol(&map, x_ref, y_ref) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn part1(content: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in content.split("\n") {
        let chars = line.chars().collect();
        map.push(chars);
    }
    let mut number_string = String::from("");
    let mut near_symbol = false;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y].is_digit(10) {
                number_string.push(map[x][y]);
                if check_if_near_symbol(&map, x, y) {
                    near_symbol = true;
                }
            } else if number_string != "" {
                count_number(&mut near_symbol, &mut number_string, &mut sum);
            }
        }
        if number_string != "" && near_symbol {
            count_number(&mut near_symbol, &mut number_string, &mut sum);
        }
    }
    println!("Part 1: {}", sum)

    // let mut sum = 0;
    // for line in content.split("\n") {
    //     let line_parts: Vec<&str> = line.split(":").collect::<Vec<&str>>();
    //     let game = line_parts[0].split(" ").collect::<Vec<&str>>()[1]
    //         .parse::<u32>()
    //         .unwrap();
    //     let cube_rounds = line_parts[1];
    //     let mut skip: bool = false;
    //     for cube_round in cube_rounds.split(";") {
    //         for cube in cube_round.split(",") {
    //             let cube_parts = cube.trim().split(" ").collect::<Vec<&str>>();
    //             let amount = cube_parts[0].parse::<u32>().expect("Expected a number");
    //             let colour = cube_parts[1];

    //             if colour == "green" && amount > 13
    //                 || colour == "red" && amount > 12
    //                 || colour == "blue" && amount > 14
    //             {
    //                 skip = true;
    //                 break;
    //             }
    //         }
    //         if skip == true {
    //             break;
    //         }
    //     }
    //     if !skip {
    //         sum += game;
    //     }
    // }
    // println!("Part 1: {}", sum);
}

fn count_number(near_symbol: &mut bool, number_string: &mut String, sum: &mut u32) {
    // println!("{}", near_symbol);
    if !*near_symbol {
        *number_string = String::from("")
    } else {
        let number = number_string.parse::<u32>().expect("Expected a number");
        println!("{}", number);
        *sum += number;
        // println!("Sum: {}", sum);
        *number_string = String::from("");
        *near_symbol = false;
    }
}
