use std::fs;

pub fn run() {
    let contents = fs::read_to_string("../inputs/day5.txt").expect("Input file not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let (result1, result2) = calc(&lines);
    println!("Part 1 output: {}", result1);
    println!("Part 2 output: {}", result2);
}

fn calc(lines: &Vec<&str>) -> (i32, i32) {
    let mut max = 0;
    let mut min = 999;
    let mut our_seat = 0;
    let mut seats = vec![];
    for line in lines {
        let mut col = (0, 127);
        let mut row = (0, 7);
        for c in line.chars() {
            match c {
                'F' => col.1 = (col.0 + col.1 + 1) / 2,
                'B' => col.0 = (col.0 + col.1 + 1) / 2,
                'L' => row.1 = (row.0 + row.1 + 1) / 2,
                'R' => row.0 = (row.0 + row.1 + 1) / 2,
                _ => {}
            }
        }
        let seat = col.0 * 8 + row.0;
        seats.push(col.0 * 8 + row.0);
        if seat < min {
            min = seat;
        }
        if seat > max {
            max = seat;
        }
    }
    for i in min..max {
        if !seats.contains(&i) {
            our_seat = i;
            break;
        }
    }
    return (max, our_seat);
}