use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Could not read file");
    part1(&content);
    part2(&content);
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

fn get_gears(map: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut gears = HashSet::new();
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
                // println!("{}, {}, {}", map[x][y], x, y);
                if map[x_ref][y_ref] == '*' {
                    gears.insert((x_ref, y_ref));
                }
            }
        }
    }
    return gears;
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
}

fn part2(content: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in content.split("\n") {
        let chars = line.chars().collect();
        map.push(chars);
    }
    let mut number_string = String::from("");
    let mut gear_dict: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut found_gears: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y].is_digit(10) {
                number_string.push(map[x][y]);
                found_gears.extend(get_gears(&map, x, y).iter());
            } else if number_string != "" {
                assign_number_to_gear(&mut found_gears, &mut gear_dict, &mut number_string);
            }
        }
        if number_string != "" {
            assign_number_to_gear(&mut found_gears, &mut gear_dict, &mut number_string);
        }
    }
    for numbers in gear_dict.values() {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }
    println!("Part 2: {}", sum)
}

fn assign_number_to_gear(
    found_gears: &mut HashSet<(usize, usize)>,
    gear_dict: &mut HashMap<(usize, usize), Vec<u32>>,
    number_string: &mut String,
) {
    let number = number_string.parse::<u32>().expect("Expected a number");
    *number_string = String::from("");
    for gear in &*found_gears {
        if !gear_dict.contains_key(&gear) {
            gear_dict.insert(*gear, Vec::new());
        }
        gear_dict.get_mut(gear).unwrap().push(number);
    }
    // println!("{}", number);
    // println!("{:?}", gear_dict);
    found_gears.clear();
}

fn count_number(near_symbol: &mut bool, number_string: &mut String, sum: &mut u32) {
    // println!("{}", near_symbol);
    if !*near_symbol {
        *number_string = String::from("")
    } else {
        let number = number_string.parse::<u32>().expect("Expected a number");
        // println!("{}", number);
        *sum += number;
        // println!("Sum: {}", sum);
        *number_string = String::from("");
        *near_symbol = false;
    }
}
