mod common;

use anyhow::{Result, anyhow};
use common::get_solver;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'd')]
    day: u8,
    #[arg(short = 'p')]
    part: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let day = args.day;
    let part = args.part;

    println!("day: {day}");
    println!("part: {part}");

    let mut file = format!("data/sample-d{day}.txt");
    let file2 = format!("data/sample-d{day}-p{part}.txt");
    if Path::new(&file2).exists() {
        file = file2;
    }
    if !Path::new(&file).exists() {
        return Err(anyhow!("File '{file}' cannot be found"));
    }

    let contents = common::get_input(&file)?;

    let solver = get_solver(day, part)?;

    return solver(&contents).map(|_| {
        ()
    });

    // Ok(())
}
