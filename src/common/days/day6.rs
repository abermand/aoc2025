use regex::Regex;

fn parse_input(input: &str) -> (Vec<char>,Vec<Vec<u64>>) {
    let mut result : Vec<Vec<u64>> = Vec::new();
    let mut operators : Vec<char> = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    for line in lines {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        if let Ok(_) = tokens[0].parse::<u64>() {
            let numbers = tokens.iter().map(|token| token.parse::<u64>().unwrap()).collect();
            result.push(numbers);
        } else {
            // this is the last line of operations
            operators = tokens.iter().map(|token| token.parse::<char>().unwrap()).collect();
        }
    }
    (operators, result)
}

fn op_to_fn(op: char) -> fn(u64, u64) -> u64 {
    match op {
        '+' => |a, b| a + b,
        '*' => |a, b| a * b,
        _ => panic!("Invalid operator"),
    }
}

// get the identity of the operation
fn ident(op: char) -> u64 {
    match op {
        '+' => 0,
        '*' => 1,
        _ => panic!("Invalid operator"),
    }
}


fn apply_operations(ops: Vec<char>, numbers: Vec<Vec<u64>>) -> u64 {
    let results = ops.iter().enumerate().map(|(i, op)| {
        let func = op_to_fn(*op);
        let identity = ident(*op);

        let result = numbers.iter().fold(identity, |acc, row| {
            func(acc, row[i])
        });
        println!("For op '{op}': {result}");
        result
    });
    results.sum::<u64>()
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let (operators, result) = parse_input(input);
    let total = apply_operations(operators, result);
    println!("Result: {total}");
    Ok(())
}

fn parse_part2(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut lines = input.lines().rev();
    // we read the lines right to left
    let operators_line = lines.next().unwrap().chars().rev().collect::<String>();
    let reg = Regex::new(r"([+*])").unwrap();
    let starts = reg.captures_iter(&operators_line).map(|cap| {
        let m = cap.get(1).unwrap();
        let c = m.as_str().chars().next().unwrap();
        (m.start(), c)
    }).collect::<Vec<_>>();

    let mut operators : Vec<(usize, usize, char)> = vec![];
    for i in 0..starts.len() {
        let (pos, op) = starts[i];
        if i > 0 {
            let (pos2, _) = starts[i-1];
            operators.push((pos2+2, pos, op));
        } else {
            operators.push((0, pos, op));
        }
    }

    let lines = lines.rev().map(|str| {  str.chars().rev().collect::<String>()}).collect::<Vec<String>>();

    // println!("Operators: {:?}", &operators);

    operators.into_iter().map(|(start, end, op)| {
        // println!("Operator at index {start}: {:?}", (start, end, op));
        let mut operands: Vec<u64> = vec![0; end - start +1];
            for i in start..=end {
                for line in &lines {
                // println!("    Current Line: {:?}, len={}", line, line.len());
                let ch = line.chars().collect::<Vec<char>>();
                    // println!("    Character at index {i}: {:?}", ch[i]);
                    if ch[i] == ' ' {
                        continue;
                    }
                    let digit = ch[i].to_digit(10).unwrap() as u64;
                    operands[i-start] = 10 * operands[i-start] +digit;
                }
            }
        // println!("Operands: {:?}", &operands);
        (op, operands)
    }).collect()
}

fn calculate_operation(op: char, operands: &Vec<u64>) -> u64 {
    let func = op_to_fn(op);
    let identity = ident(op);
    let result = operands.iter().fold(identity, |acc, operand| { func(acc, *operand) });
    result
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let operations = parse_part2(input);
    let mut total = 0;
    for (operation, operands) in operations.iter() {
        let result = calculate_operation(*operation, operands);
        println!("For operation {operation}: operands: {:?}, result: {result}", operands);
        total += result;
    }
    println!("Result: {total}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = "123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +  ";
    let (operators, result) = parse_input(input);
    let total = apply_operations(operators, result);
    assert_eq!(total, 4277556);
}

#[test]
fn test_part2() {
    let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    let operations = parse_part2(input);
    let mut total = 0;
    for (operation, operands) in operations.iter() {
        let result = calculate_operation(*operation, operands);
        println!("For operation {operation}: operands: {:?}, result: {result}", operands);
        total += result;
    }
    assert_eq!(total, 3263827);
}
