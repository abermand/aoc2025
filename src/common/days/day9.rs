use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u64,u64)> {
    input.lines().map(|line| {
        let (y, x) = line.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect()
}

fn find_largest_rec_area(points: &Vec<(u64,u64)>) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let width = if x2 > x1 { x2 - x1 + 1}  else { x1 - x2 + 1 } as u64;
            let height = if y2 > y1 { y2 - y1 + 1}  else { y1 - y2 + 1 } as u64;
            let area = width * height;
            println!("Inspecting {:?} {:?} Area: {}", (x1, y1), (x2, y2), area);
            max_area = max_area.max(area);
        }
    }
    max_area
}

#[derive(Clone)]
enum Point {
    Red,
    Green,
}

fn create_map(points: &Vec<(u64,u64)>) -> Vec<Vec<Option<Point>>> {
    let width : u64 = points.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height : u64 = points.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut map = vec![vec![None; width as usize]; height as usize];
    for (x, y) in points {
        map[*y as usize][*x as usize] = Some(Point::Red);
    }

    let mut count = 0;
    for (a, b) in points.iter().circular_tuple_windows() {
        let (x1, y1) = a;
        let (x2, y2) = b;
        if x1 == x2 && y1 == y2 {
            continue;
        }
        if x1 == x2 {
            let min = *y1.min(y2) as u64;
            let max = *y1.max(y2) as u64;
            for y in min+1..max {
                map[y as usize][*x1 as usize] = Some(Point::Green);
            }
        } else if y1 == y2 {
            let min = *x1.min(x2) as u64;
            let max = *x1.max(x2) as u64;
            for x in min+1..max {
                map[*y1 as usize][x as usize] = Some(Point::Green);
            }
        }
        if count > points.len() {
            break;
        } else {
            count += 1;
        }
    }
    map
}

fn print_map(map: Vec<Vec<Option<Point>>>) {
    for row in map {
        for cell in row {
            match cell {
                Some(Point::Red) => print!("\x1b[0;41m#"),
                Some(Point::Green) => print!("\x1b[0;42mX"),
                None => print!("\x1b[0m."),
            }
        }
        println!("\x1b[0m");
    }
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let tiles = parse_input(input);

    let result = find_largest_rec_area(&tiles);
    println!("Result: {result}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let tiles = parse_input(input);

    let result = find_largest_rec_area(&tiles);
    println!("Result: {result}");
    Ok(())
}

fn is_filled(map: &Vec<Vec<Option<Point>>>, x: u64, y: u64) -> bool {
    match map[y as usize][x as usize] {
        Some(Point::Red) => true,
        Some(Point::Green) => true,
        None => false,
    }
}

fn is_rect_filled(map: &Vec<Vec<Option<Point>>>, x1: u64, y1: u64, x2: u64, y2: u64) -> bool {
    let xmin = x1.min(x2);
    let xmax = x1.max(x2);
    let ymin = y1.min(y2);
    let ymax = y1.max(y2);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if !is_filled(map, x, y) {
                return false;
            }
        }
    }
    true
}

fn find_largest_filled_area(points: &Vec<(u64,u64)>, map: &Vec<Vec<Option<Point>>>) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            if !is_rect_filled(map, x1, y1, x2, y2) {
                continue;
            }
            let width = if x2 > x1 { x2 - x1 + 1}  else { x1 - x2 + 1 } as u64;
            let height = if y2 > y1 { y2 - y1 + 1}  else { y1 - y2 + 1 } as u64;
            let area = width * height;
            println!("Inspecting {:?} {:?} Area: {}", (x1, y1), (x2, y2), area);
            max_area = max_area.max(area);
        }
    }
    max_area
}


#[test]
fn test_part1() {
    let input = include_str!("../../../data/test-day9.txt");
    let tiles = parse_input(input);

    let result = find_largest_rec_area(&tiles);
    assert_eq!(result, 50);
}


#[test]
fn test_part2() {
    let input = include_str!("../../../data/test-day9.txt");
    let tiles = parse_input(input);
    let map = create_map(&tiles);

    print_map(map.clone());

    let result = find_largest_filled_area(&tiles, &map);
    assert_eq!(result, 24);
}
