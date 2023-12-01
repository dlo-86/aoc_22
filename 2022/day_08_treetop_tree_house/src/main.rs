use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use mapcomp::vecc;

fn main() {
    let mut trees: Vec<Vec<u32>> = vec![];
    let lines = read_lines("./input.txt").expect("could not read file");
    for line in lines {
        if let Ok(ok_line) = line {
            let tree_line: Vec<u32> = transform_tree_string_into_digit_vector(ok_line);
            trees.push(tree_line);
        }
    }
    part1(&trees);
    part2(&trees);
}

fn part1(trees: &Vec<Vec<u32>>) {
    let width = trees.first().expect("No tree line in trees").len();
    let mut sum = 0;
    for y in 0..trees.len() {
        for x in 0..width {
            if y == 0 || y == trees.len() - 1 || x == 0 || x == width - 1 {
                sum += 1;
                continue;
            }
            let value = trees[y][x];
            if value > *trees[y][0..x].iter().max().expect("list is empty")
                || value
                    > *trees[y][(x + 1)..width]
                        .iter()
                        .max()
                        .expect("list is empty")
                || value > collect_vertical_trees(&trees, x, 0 as usize, y)
                || value > collect_vertical_trees(&trees, x, y + 1, trees.len())
            {
                sum += 1;
            }
        }
    }
    println!("Part1: {:?}", sum);
}

fn part2(trees: &Vec<Vec<u32>>) {
    let width = trees.first().expect("No tree line in trees").len();
    let mut view_distance_max = 0;
    for y in 0..trees.len() {
        for x in 0..width {
            if y == 0 || y == trees.len() - 1 || x == 0 || x == width - 1 {
                // right at the edge one direction is zero, thus the score is always 0 and we can skip it
                continue;
            }
            let view_distance = get_upword_distance(trees, x, y)
                * get_downword_distance(trees, x, y)
                * get_left_distance(trees, x, y)
                * get_right_distance(trees, x, y);
            if view_distance > view_distance_max {
                view_distance_max = view_distance;
            }
        }
    }
    println!("Part2: {:?}", view_distance_max);
}

fn get_upword_distance(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = trees[y][x];
    for i in (0..y).rev() {
        if trees[i][x] >= value {
            return (y - i) as u32;
        }
    }
    return y as u32;
}

fn get_downword_distance(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = trees[y][x];
    let height = trees.len();
    for i in y + 1..height {
        if trees[i][x] >= value {
            return (i - y) as u32;
        }
    }
    return (height - y - 1) as u32;
}

fn get_left_distance(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = trees[y][x];
    for i in (0..x).rev() {
        if trees[y][i] >= value {
            return (x - i) as u32;
        }
    }
    return x as u32;
}

fn get_right_distance(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = trees[y][x];
    let width = trees.first().expect("No tree line in trees").len();
    for i in x + 1..width {
        if trees[y][i] >= value {
            return (i - x) as u32;
        }
    }
    return (width - x - 1) as u32;
}

fn collect_vertical_trees(trees: &Vec<Vec<u32>>, x: usize, y_start: usize, y_end: usize) -> u32 {
    return *vecc![tree_line[x]; for tree_line in trees[y_start..y_end].iter(); if true]
        .iter()
        .max()
        .expect("list is empty");
}

fn transform_tree_string_into_digit_vector(line: String) -> Vec<u32> {
    return line.chars().map(|x| x.to_digit(10).unwrap()).collect();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
