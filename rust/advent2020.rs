mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
use std::env;
use std::fs;

fn main() {
    for argument in env::args() {
        let parsed_arg = argument.parse::<i32>();
        let _ = match parsed_arg {
            Ok(v) => run_day(v),
            Err(_) => run_all(),
        };
    }
}

fn run_all() -> Result<(i32, i32), &'static str> {
    let max = 6;
    for i in 1..max + 1 {
        let res = run_day(i);
        match res {
            Ok(_v) => {}
            Err(e) => {
                println!("Day {} failed: {}", i, e);
                return Err(e);
            }
        }
    }
    return Ok((0, 0));
}

fn run_day(day: i32) -> Result<(i32, i32), &'static str> {
    let method: fn(String) -> Result<(i32, i32), &'static str> = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        4 => day4::run,
        5 => day5::run,
        6 => day6::run,
        _ => return Err("Task not yet implemented"),
    };
    let contents =
        fs::read_to_string(format!("../inputs/day{}.txt", day)).expect("Input file not found");
    match method(contents) {
        Ok(v) => {
            println!("Day {}: {} & {}", day, v.0, v.1);
            return Ok(v);
        }
        Err(e) => {
            println!("Day {} failed: {}", day, e);
            return Err(e);
        }
    }
}
