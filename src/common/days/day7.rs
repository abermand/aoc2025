// use crate::common;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}


fn count_splits(lines : Vec<Vec<char>>) -> u64 {
    let mut lines = lines.clone();
    let mut count = 0_u64;
    // get initial beam position:
    let first_line = &lines[0];
    let beam_start = first_line.iter().position(|&c| c == 'S').unwrap();
    lines[0][beam_start] = '|';
    for y in 1..lines.len() {
        // println!("---- Row #{}", y);
        let previous_line = lines[y-1].clone();
        let line = lines.get_mut(y).unwrap();
        // println!("  Prev: {}", previous_line.clone().into_iter().collect::<String>());
        // println!("  Cur:  {}", line.clone().into_iter().collect::<String>());
        for x in 0..line.len() {
            let current_char = line[x];
            let previous_char = previous_line[x];
            if previous_char == '|' && current_char != '|' {
                if current_char == '^' {
                    count += 1;

                    if x - 1 > 0 && line[x-1] == '.' {
                        line[x-1] = '|';
                    }
                    if x + 1 < line.len() && line[x+1] == '.' {
                        line[x+1] = '|';
                    }
                } else {
                    line[x] = '|';
                }
            }
        }
        // println!("  After:{}", line.clone().into_iter().collect::<String>());
    }
    count
}

fn count_timelines(lines : Vec<Vec<char>>) -> u64 {
    let mut lines = lines.clone();
    // get initial beam position:
    let first_line = &lines[0];
    let width = first_line.len();
    let beam_start = first_line.iter().position(|&c| c == 'S').unwrap();
    lines[0][beam_start] = '|';
    let mut timelines : Vec<u64> = vec![0; width];
    timelines[beam_start] = 1;
    // println!("Initial Timelines: {:?}", timelines);
    for y in 1..lines.len() {
        // println!("---- Row #{}", y);
        let previous_line = lines[y-1].clone();
        let line = lines.get_mut(y).unwrap();
        // println!("  Prev: {}", previous_line.clone().into_iter().collect::<String>());
        let mut generated_timelines = timelines.clone();
        for x in 0..width {
            let current_char = line[x];
            let previous_char = previous_line[x];
            if previous_char == '|' && current_char != '|' {
                if current_char == '^' {
                    // we split the timelines here
                    let count_timelines = timelines[x];
                    generated_timelines[x] = 0;
                    if x - 1 > 0 {
                        line[x-1] = '|';
                        generated_timelines[x-1] += count_timelines;
                    }
                    if x + 1 < width {
                        line[x+1] = '|';
                        generated_timelines[x+1] += count_timelines;
                    }
                } else {
                    line[x] = '|';
                }
            }
        }
        // println!("  Cur:  {}", line.clone().into_iter().collect::<String>());
        //retain the information for the next row
        timelines = generated_timelines.clone();
        // println!("Generated: {:?}", generated_timelines);
    }
    // println!("All beams: \n");
    lines.into_iter().map(|line| line.into_iter().collect::<String>())
        .for_each(|path| println!("{}", path));
    timelines.into_iter().sum::<u64>() + 1
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    // Parse input
    let lines = parse_input(input);

    let result = count_splits(lines);
    println!("Result: {result}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    // Parse input
    let lines = parse_input(input);

    let result = count_timelines(lines);
    println!("Result: {result}");
    Ok(())
}


#[test]
fn test_part1() {
    let input = include_str!("../../../data/test-day7.txt");
    let lines = parse_input(&input);

    let result = count_splits(lines);
    println!("Result: {result}");

    assert_eq!(result, 21);
}

#[test]
fn test_part2() {
    let input = include_str!("../../../data/test-day7.txt");
    let lines = parse_input(&input);

    let result = count_timelines(lines);
    println!("Result: {result}");

    assert_eq!(result, 40);
}
