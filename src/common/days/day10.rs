
use std::ops::BitXor;
use itertools::Itertools;

#[derive(Debug)]
struct Machine{
    indicators: u32,
    wiring: Vec<u32>,
    joltages: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let machines = input.lines().map(|line| {
        let (left, rest) = line.split_once(']').unwrap();
        let indicators: u32 = left[1..].chars().fold(0u32, |acc, c| {
            (acc << 1) | if c == '#' { 1 } else { 0 }
        });
        let (wiring_str, joltages_str) = rest[1..].split_once(" {").unwrap();
        println!("Wiring: {}, Joltages: {}", wiring_str, joltages_str); 
        let wiring = wiring_str[1..].split(' ').map(|part| {
            part.trim_matches(|c| c == '(' || c == ')' ).split(',').fold(0_u32, |acc, num| {
                let pos = num.parse::<u32>().unwrap();
                acc | (1 << pos)
            })
        }).collect();   
        let joltages = joltages_str.trim_end_matches('}').split(',').map(|num| num.parse().unwrap()).collect();
        Machine {
            indicators,
            wiring,
            joltages,
        }
    }).collect();
    println!("Machines: {:?}", machines);    
    machines
}  

fn toggle_indicators(statuses: u32, wiring: &Vec<u32>, index: usize) -> u32 {
    statuses.bitxor(wiring[index])
}

fn print_states(statuses: u32, num_indicators: usize) {
    let mut s = String::new();
    for i in (0..num_indicators).rev() {
        if (statuses & (1 << i)) != 0 {
            s.push('#');
        } else {
            s.push('.');
        }
    }
   print!("{}", s.as_str());
}

fn solve_fewest_presses(machine: &Machine) -> u32 {
    let mut min_presses = usize::MAX;
    let max_wirings  = machine.wiring.len();
    let desired_state = machine.indicators;
    // let total_combinations = 1 << max_wirings;
    // let possibilities: Vec<u32> = (0..total_combinations).collect();
    let combinations = machine.wiring.iter().combinations(max_wirings as usize);
    for combo in combinations {
        println!   ("Trying combo: {:?}", combo);
        let mut state = 0u32;
        let mut count : usize  = 0;
        for step in combo.iter() {

            print!("#{count} ");
            print_states(state, 10);
            state = state.bitxor(*step);
            print!(" -> ");
            print_states(state, 10);
            println!();
            count += 1;
            if state == desired_state {
                println!("  Reached desired state!");
                break;
            }
            if state == 0 {
                println!("  Reached zero state, aborting this combo.");
                break; 
            } 
            if count >= min_presses {
                println!("  Exceeded current minimum of {min_presses} presses, aborting this combo.");
                break;
            }
        }
        if state == desired_state && count < min_presses {
                println!("  New minimum found: {count} presses.");
                min_presses = count;
        }
    }
    min_presses as u32
}


pub fn part1(input: &str) -> anyhow::Result<()> {
    let machines : Vec<Machine> = parse_input(input);
    let result = machines.iter().map(solve_fewest_presses).sum::<u32>();
    println!("Result: {result}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    todo!();
    let result = 0;
    println!("Result: {result}");
    Ok(())
}


#[test]
fn test_part1() {
    let input = include_str!("../../../data/test-day10.txt");
    let machines : Vec<Machine> = parse_input(input);
    let result = machines.iter().map(solve_fewest_presses).sum::<u32>();
    assert_eq!(result, 7);
}