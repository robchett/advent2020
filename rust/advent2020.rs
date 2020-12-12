mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use std::env;
use std::fs;

fn main() {
    for argument in env::args() {
        let parsed_arg = argument.parse::<i32>();
        if parsed_arg.is_ok() {
            match run_day(parsed_arg.unwrap()) {
                Ok(_) => return,
                Err(e) => panic!(e),
            }
        };
    }
    match run_all() {
        Ok(_) => return,
        Err(e) => panic!(e),
    }
}

fn run_all() -> Result<(i64, i64), &'static str> {
    let max = 12;
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

fn run_day(day: i32) -> Result<(i64, i64), &'static str> {
    let method: fn(String) -> Result<(i64, i64), &'static str> = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        4 => day4::run,
        5 => day5::run,
        6 => day6::run,
        7 => day7::run,
        8 => day8::run,
        9 => day9::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        _ => return Err("Task not yet implemented"),
    };
    let contents =
        fs::read_to_string(format!("../inputs/day{}.txt", day)).expect("Input file not found");
    match method(contents.replace("\r", "")) {
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
