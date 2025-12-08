use std::ops::RangeInclusive;
use std::cmp;

type Range64 = RangeInclusive::<u64>;

fn parse_input(input: &str) -> (Vec<Range64>, Vec<u64>) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            start..=end
        })
        .collect();
    let numbers = numbers.lines().map(|line| line.parse().unwrap()).collect();
    (ranges, numbers)
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut count = 0;
    let (ranges, numbers) = parse_input(input);
    for number in numbers {
        if ranges.iter().any(|range| range.contains(&number)) {
            count += 1;
        }
    }
    println!("Result: {count}");
    Ok(())
}

fn can_expand_range(r1: &Range64, r2: &Range64) -> Option<Range64> {
    // order the two ranges by min to keep it simple:
    if r2.end() < r1.start() {
        None
    } else if r2.start() > r1.end() {
        // ranges (i,don)'t intersect
        None
    } else {
        let min = cmp::min(r1.start(), r2.start()).clone();
        let max = cmp::max(r1.end(), r2.end()).clone();
        println!("Ranges {:?} {:?} reduced to {:?}", r1, r2, min..=max);
        Some(min..=max)
    }
}
pub fn reduce_ranges(mut ranges: Vec<Range64>) -> Vec<Range64> {
    let mut reduced_ranges : Vec<RangeInclusive<u64>> = Vec::new();
    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        if let Some((i,reduced)) = ranges.iter().enumerate().find_map(|(i, r)| {
            if let Some(reduced_range) = can_expand_range(&range, &r) {
                Some((i,reduced_range))
            } else {
                None
            }
        }) {
            // replace the range with the reduced range
            ranges[i] = reduced.clone();
        } else {
            reduced_ranges.push(range);
        }
    }
    reduced_ranges
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let (ranges, _) = parse_input(input);
    let reduced_ranges = reduce_ranges(ranges);
    let count : u64  = reduced_ranges.iter().map(|range| range.end() - range.start() + 1).sum();
    println!("Result: {count}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let mut count = 0;
    let (ranges, numbers) = parse_input(input);
    for number in numbers {
        println!("Number: {number} in range?");
        if ranges.iter().any(|range| range.contains(&number)) {
            count += 1;
        }
    }
    assert_eq!(count, 3);
}

#[test]
fn test_part2() {
    let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let (ranges, _) = parse_input(input);
    let reduced_ranges = reduce_ranges(ranges);
    let count : u64  = reduced_ranges.iter().map(|range| range.end() - range.start() + 1).sum();
    assert_eq!(count, 14);
}
