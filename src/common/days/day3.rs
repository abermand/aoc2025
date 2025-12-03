fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn find_joltage(battery: &Vec<u8>) -> u64 {
    let mut joltage = 0u64;
    for i in 0..battery.len() - 1 {
        for j in i + 1..battery.len() {
            let v: u64 = 10 * (battery[i] as u64) + battery[j] as u64;
            if joltage < v {
                joltage = v;
            }
        }
    }
    joltage
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage).sum::<u64>();
    println!("Result: {result}");
    Ok(())
}

fn find_joltage2(battery: &Vec<u8>) -> u64 {
    let mut numbers_left = 12;
    let mut current = 0;
    let mut joltage = 0;
    // println!("Battery: {:?}", battery);
    while numbers_left > 0 && current < battery.len() {
        // look ahead if a bigger number can be taken with enough room to spare
        let mut lookahead = current + 1;
        let mut max_found = battery[current];
        let mut max_index = current;
        while lookahead < battery.len() && lookahead + numbers_left - 1 < battery.len() {
            if battery[lookahead] > max_found {
                max_found = battery[lookahead];
                max_index = lookahead;
            }
            lookahead += 1;
        }
        // the current number is the highest possible candidate: take it
        joltage = joltage * 10 + max_found as u64;
        // println!("    Best candidate [{max_index}] -> {max_found}- joltage = {joltage}");
        numbers_left -= 1;
        current = max_index + 1;
    }

    return joltage;
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage2).sum::<u64>();
    println!("Result: {result}");
    Ok(())
}

#[test]
fn test_part1a() {
    let input = "123456789";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage).sum::<u64>();
    assert_eq!(result, 89);
}
#[test]
fn test_part1b() {
    let input = "9111112345678";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage).sum::<u64>();
    assert_eq!(result, 98);
}

#[test]
fn test_part1() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage).sum::<u64>();
    assert_eq!(result, 357);
}

#[test]
fn test_part2a() {
    let input = "987654321111111";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage2).sum::<u64>();
    assert_eq!(result, 987654321111);
}
#[test]
fn test_part2b() {
    let input = "811111111111119";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage2).sum::<u64>();
    assert_eq!(result, 811111111119);
}

#[test]
fn test_part2() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    let batteries = parse(input);

    let result = batteries.iter().map(find_joltage2).sum::<u64>();
    assert_eq!(result, 3121910778619);
}
