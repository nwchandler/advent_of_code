use std::env;
use std::process;

use aoc23::*;

const ARGUMENT_ERROR: i32 = 1;
const FUNCTION_ERROR: i32 = 2;

fn main() {
    // skip the first argument, which is the command name
    let mut args = env::args().skip(1);
    let day: usize;
    if let Some(i) = args.next() {
        day = i.parse().unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(ARGUMENT_ERROR);
        });
    } else {
        println!("must specify the day as a number");
        process::exit(ARGUMENT_ERROR);
    }

    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
        day22::run,
        day23::run,
        day24::run,
        day25::run,
    ];

    if day > days.len() {
        eprintln!("day {day} is not implemented");
        process::exit(ARGUMENT_ERROR);
    }
    let func = days[day - 1];

    let input = std::io::read_to_string(std::io::stdin()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(ARGUMENT_ERROR);
    });

    let result = func(&input);

    match result {
        Ok(output) => {
            print_result(day, &output);
        }
        Err(err) => {
            eprintln!("error from function: {err}");
            process::exit(FUNCTION_ERROR);
        }
    }
}

fn print_result(day: usize, solution: &Solution) {
    println!("-- Day {:02} --", day);
    println!("   Part 1: {}", solution.part1);
    println!("   Part 2: {}", solution.part2);
}
