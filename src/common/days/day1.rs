fn process(input: &str, rotation: fn(i32, i32) -> (i32, u32)) -> (i32, u32) {
    let lines = input.lines();
    let (current, count) = lines.fold((50, 0), |(current, count), line| {
        let (dir, n) = line.split_at(1);

        println!("Processing line: {}, ({},{})", line, dir, n);
        let change = match dir {
            "L" => -1 * n.parse::<i32>().expect("Not a i32"),
            "R" => 1 * n.parse::<i32>().expect("Not a i32"),
            _ => unreachable!(),
        };
        let (new, zeroed) = rotation(current, change);
        println!("    New value: {new}, zeroed: {zeroed}");
        (new, count + zeroed)
    });
    (current, count)
}

// Rotates the current value by the given change, returning the new value and if the end value is zero.
fn rotation_part1(current: i32, change: i32) -> (i32, u32) {
    let new = (current + 100 + change) % 100;
    if new == 0 { (new, 1) } else { (new, 0) }
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let (_, count) = process(&input, rotation_part1);
    println!("Result: {count}");
    Ok(())
}

// Rotates the current value by the given change, returning the new value and the number of times the value was zeroed.
fn rotation_part2(current: i32, change: i32) -> (i32, u32) {
    // we must take into account the number of loops when rotating.
    let loops: u32 = change.abs() as u32 / 100;
    let change = change % 100;
    let new: i32 = (current + change + 100) % 100;
    println!("    loops: {loops}, new: {new}");
    if new == 0 {
        (new, loops + 1)
    } else if current != 0 && change > 0 && new < current {
        (new, loops + 1)
    } else if current != 0 && change < 0 && new > current {
        (new, loops + 1)
    } else {
        (new, loops)
    }
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let (_, count) = process(&input, rotation_part2);
    println!("Result: {count}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    let (current, count) = process(input, rotation_part1);
    assert_eq!(current, 32);
    assert_eq!(count, 3);
}

#[test]
fn test_part2() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    let result = process(input, rotation_part2);
    assert_eq!(result, (32, 6));
}
