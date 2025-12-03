fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|range| {
        //println!("range: {range}");
        let (start, end) = range.trim().split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    })
}

fn part1_is_invalid(num: &u64) -> bool {
    let str = num.to_string();
    if str.len() % 2 == 1 {
        false
    } else {
        let (left, right) = str.split_at(str.len() / 2);
        left == right
    }
}

fn part2_is_invalid(num: &u64) -> bool {
    // println!("Trying to find repeats in {num}");
    let str = num.to_string();
    for size in 1..=str.len() / 2 {
        let times = str.len() / size;
        if str[0..size].repeat(times) == str {
            // println!(
            // "-> Number '{str}' has repeats: s={size} str='{}'",
            // &str[0..size]
            // );

            return true;
        }
    }

    false
}

fn add_invalid_numbers(input: &str, is_invalid: fn(&u64) -> bool) -> u64 {
    let ranges = parse(input);
    let result = ranges
        .map(|(start, end)| (start..=end).filter(|num| is_invalid(num)).sum::<u64>())
        .sum();
    result
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let result = add_invalid_numbers(input, part1_is_invalid);
    println!("Result: {result}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = "1-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(add_invalid_numbers(input, part1_is_invalid), 1227775554);
}

#[test]
fn test_part2() {
    let input = "1-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(add_invalid_numbers(input, part2_is_invalid), 4174379265);
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let result = add_invalid_numbers(input, part2_is_invalid);
    println!("Result: {result}");
    Ok(())
}
