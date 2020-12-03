mod day1;
mod day2;
mod day3;
use std::env;

fn main() {
    for argument in env::args() {
        let parsed_arg = argument.parse::<i32>();
        match parsed_arg {
            Ok(v) => run_day(v),
            Err(_) => (),
        }
    }
}

fn run_day(day: i32) {
    let method: fn() = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        _ => day1::run
    };

    method();
}