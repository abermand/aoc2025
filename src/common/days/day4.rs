fn expand_input(input: &str) -> String {
    // expand the input so that getting items will not produce out of bounds errors when accessing adjacent elements
    let line_width = input.find('\n').unwrap_or(input.len());
    let blank_line = ".".repeat(line_width + 2);
    let mut appended_string = blank_line.clone();
    appended_string.push('\n');
    input
        .lines()
        .for_each(|line| appended_string.push_str(format!(".{line}.\n").as_str()));
    appended_string.push_str(&blank_line);

    appended_string
}

fn count_neighbours(input: &Vec<Vec<char>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in x.saturating_sub(1)..=x + 1 {
        for j in y.saturating_sub(1)..=y + 1 {
            if i == x && j == y {
                continue;
            }
            if input[j][i] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn count_movable_tp(input: &str) -> u32 {
    let input_width = input.find('\n').unwrap_or(input.len());
    let expanded_input = expand_input(input);
    // println!("-------\nExpanded Input:\n{}\n-------", expanded_input);
    let vec_input = expanded_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for y in 1..vec_input.len() - 1 {
        for x in 1..input_width + 1 {
            if vec_input[y][x] == '@' && count_neighbours(&vec_input, x, y) < 4 {
                count += 1;
            }
        }
    }
    count
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let result = count_movable_tp(input);
    println!("Result: {result}");
    Ok(())
}

pub fn count_and_remove(input: &mut Vec<Vec<char>>, width: usize, height: usize) -> u32 {
    let mut to_be_removed: Vec<(usize, usize)> = Vec::new();
    let mut count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if input[y][x] == '@' && count_neighbours(&input, x, y) < 4 {
                count += 1;
                to_be_removed.push((x, y));
            }
        }
    }
    for (x, y) in to_be_removed {
        input[y][x] = '.';
    }
    count
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let input_width = input.find('\n').unwrap_or(input.len());
    let expanded_input = expand_input(input);
    // println!("-------\nExpanded Input:\n{}\n-------", expanded_input);
    let mut vec_input = expanded_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = input_width + 2;
    let height = vec_input.len();
    let mut total_count = 0;
    let mut pass = 1;
    loop {
        let count = count_and_remove(&mut vec_input, width, height);
        println!("Pass {pass}: {count} found.");
        if count == 0 {
            break;
        }
        total_count += count;
        pass += 1;
    }
    println!("Result: {total_count}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    assert_eq!(count_movable_tp(input), 13);
}

#[test]
fn test_part2() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    let input_width = input.find('\n').unwrap_or(input.len());
    let expanded_input = expand_input(input);
    let mut vec_input = expanded_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = input_width + 2;
    let height = vec_input.len();
    let mut total_count = 0;
    let mut pass = 1;
    loop {
        let count = count_and_remove(&mut vec_input, width, height);
        println!("Pass {pass}: {count} found.");
        if count == 0 {
            break;
        }
        total_count += count;
        pass += 1;
    }
    println!("Result: {total_count}");

    assert_eq!(total_count, 43);
}
