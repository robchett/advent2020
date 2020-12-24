mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
use std::env;
use std::fs;
use std::time::{Instant};

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
    let max = 24;
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
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        21 => day21::run,
        22 => day22::run,
        23 => day23::run,
        24 => day24::run,
        _ => return Err("Task not yet implemented"),
    };
    let now = Instant::now();
    let contents =
        fs::read_to_string(format!("../inputs/day{}.txt", day)).expect("Input file not found");
    match method(contents.replace("\r", "")) {
        Ok(v) => {
            println!("Day {:0>2}: {} & {}", day, v.0, v.1);
            println!("Day {:0>2}: took {:.4}ms", day, now.elapsed().as_micros() as f64 / 1000.);
            return Ok(v);
        }
        Err(e) => {
            println!("Day {} failed: {}", day, e);
            return Err(e);
        }
    }
}
