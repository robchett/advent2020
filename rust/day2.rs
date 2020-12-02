use std::fs;

pub fn run() {
    let contents = fs::read_to_string("../inputs/day2.txt").expect("Input file not found");
    let lines = contents.split("\n");
    let mut points = Vec::new();
    for line in lines {
        points.push(PasswordLine::new(line));
    }
    println!("Part 1 output: {}", part1(&points));
    println!("Part 2 output: {}", part2(&points));
}

struct PasswordLine {
    letter: char,
    min: u32,
    max: u32,
    password: String,
}

impl PasswordLine {
    fn new(line: &str) -> PasswordLine {
        let mut letter: char = '_';
        let mut min: u32 = 0;
        let mut max: u32 = 0;
        let mut password: String = "".to_owned();
        let mut is_max = false;
        let mut is_password = false;
        for c in line.trim().chars() {
            match c {
                '0'..='9' => {
                    if is_max {
                        if max > 0 { max *= 10}
                        max += c.to_digit(10).unwrap_or_default()
                    } else {
                        if min > 0 { min *= 10}
                        min += c.to_digit(10).unwrap_or_default()
                    }
                }
                'a'..='z' => {
                    if is_password {
                        password.push(c)
                    } else {
                        letter = c
                    }
                }
                '-' => is_max = true,
                ':' => is_password = true,
                ' ' => (),
                _ => println!("Unexpect character found in line- {}", c)

            } 
        }
        return PasswordLine {letter, min: u32::from(min), max: u32::from(max), password: password.to_string()};
    }
    fn valid_length(&self) -> bool {
        let mut matches = 0;
        for c in self.password.chars() {
            if c == self.letter {
                matches+=1;
            }
        }
        return matches >= self.min && matches <= self.max;
    }
    fn valid_positions(&self) -> bool {
        let match1 = self.password.chars().nth(self.min as usize - 1).unwrap() == self.letter;
        let match2 = self.password.chars().nth(self.max as usize - 1).unwrap() == self.letter;
        return (match1 || match2) && !(match1 && match2)
    }
}

fn part1(points: &Vec<PasswordLine>) -> i32 {
    let mut count = 0;
    for point in points {
        if point.valid_length(){
            count += 1;
        }
    }
    return count;
}

fn part2(points: &Vec<PasswordLine>) -> i32 {       
    let mut count = 0;
    for point in points {
        if point.valid_positions() {
            count += 1;
        }
    }
    return count;
}