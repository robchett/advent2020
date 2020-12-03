use std::fs;

pub fn run() {
    let contents = fs::read_to_string("../inputs/day3.txt").expect("Input file not found");

    println!("Part 1 output: {}", part1(&contents));
    println!("Part 2 output: {}", part2(&contents));
}

fn part1(contents: &str) -> i32 {
    let mut lines = contents.split("\n");
    let mut count = 0;
    let mut x = 0;
    let mut line = lines.nth(0).unwrap();
    let length = line.len() - 1;
    loop {
        if line.chars().nth(x % length).unwrap() == '#' {
            count += 1;
        }
        let line_option = lines.next();
        x += 3;
        match line_option {
            Some(x) => line = x,
            None => break
        }
    }
    return count;
}

fn part2(contents: &str) -> i32 {       
    let mut lines = contents.split("\n");
    let mut counts = [0, 0, 0, 0, 0];
    let mut row = 0;
    let mut line = lines.nth(0).unwrap();
    let length = line.len() - 1;
    let indexes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    loop {
        for (i, coords) in indexes.iter().enumerate() {
            let (x, y) = coords;
            if row % y == 0  {
                if line.chars().nth(row * x / y % length).unwrap() == '#' {
                    counts[i] += 1;
                }   
            }     
        }   
        let line_option = lines.next();
        row += 1;
        match line_option {
            Some(x) => line = x,
            None => break
        }
    }
    println!("{}, {}, {}, {}, {}", counts[0], counts[1], counts[2], counts[3], counts[4]);
    return counts[0] * counts[1] * counts[2] * counts[3] * counts[4];
}