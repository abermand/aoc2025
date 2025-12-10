mod days;

use anyhow::anyhow;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub type Solver = fn(&str) -> anyhow::Result<()>;

lazy_static! {
    static ref CHALLENGES: HashMap<u8, (Solver, Solver)> = {
        let mut m: HashMap<u8, (Solver, Solver)> = HashMap::new();
        m.insert(1, (days::day1::part1, days::day1::part2));
        m.insert(2, (days::day2::part1, days::day2::part2));
        m.insert(3, (days::day3::part1, days::day3::part2));
        m.insert(4, (days::day4::part1, days::day4::part2));
        m.insert(5, (days::day5::part1, days::day5::part2));
        m.insert(6, (days::day6::part1, days::day6::part2));
        m.insert(7, (days::day7::part1, days::day7::part2));
        m.insert(8, (days::day8::part1, days::day8::part2));
        m.insert(9, (days::day9::part1, days::day9::part2));
        // m.insert(12, (days::day12::part1, days::day12::part2));
        m
    };
}

pub fn get_input(name: &str) -> anyhow::Result<String> {
    let file = File::open(name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_solver(day: u8, part: u8) -> anyhow::Result<Solver> {
    if let Some((p1, p2)) = CHALLENGES.get(&day) {
        match part {
            1 => return anyhow::Ok(*p1),
            2 => return anyhow::Ok(*p2),
            _ => return Err(anyhow!("Invalid part number: {part}")),
        }
    }
    Err(anyhow!("No implemented challenge for day {day}"))
}
