use itertools::{Itertools, chain};
use std::iter;
use std::collections::BTreeMap;

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
    Blank,
}

struct TilesMapAsBTree {
    width: usize,
    height: usize,
    data: BTreeMap<(usize, usize), Point>,
}

fn create_map(points: &Vec<(u64,u64)>) -> TilesMapAsBTree {
    let width : usize = (points.iter().map(|(x, _)| x).max().unwrap() + 1) as usize ;
    let height : usize = (points.iter().map(|(_, y)| y).max().unwrap() + 1) as usize;

    let mut map= TilesMapAsBTree{
        width, 
        height,
        data: BTreeMap::new(),
    };
    for (x, y) in points {
        map.data.insert((*x as usize,*y as usize),Point::Red);
    }

    let mut count = 0;
    for (a, b) in points.iter().circular_tuple_windows() {
        let a = (a.0 as usize, a.1 as usize);
        let b = (b.0 as usize, b.1 as usize);
        let (x1, y1) = a;
        let (x2, y2) = b;
        if x1 == x2 && y1 == y2 {
            continue;
        }
        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);
            for y in min+1..max {
                map.data.insert((x1,y), Point::Green);
            }
        } else if y1 == y2 {
            let min = x1.min(x2);
            let max = x1.max(x2);
            for x in min+1..max {
                map.data.insert((x,y1), Point::Green);
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



fn is_inside(x: usize, y: usize, map: &TilesMapAsBTree, width: usize, height: usize) {
    // let left = (0..height).into_iter().zip(iter::repeat(0_usize));
    // let top = iter::repeat(0_usize).zip((0..width).into_iter());
    // let right = (0..height).into_iter().zip(iter::repeat(width - 1));
    // let bottom = iter::repeat(height -1).zip((0..width).into_iter());
    // let all_directions = chain!(left, top, right, bottom);
    // let (px, py) = (point.x, point.y);
    // for (dir_y, dir_x) in all_directions {

    // }
}


fn print_map(map: &TilesMapAsBTree) {
    let width = map.width;
    let height = map.height;
    let data = &map.data;
    for y in 0..height {
        for x in 0..width {
            match data.get(&(y,x)) {
                Some(Point::Red) => print!("\x1b[0;41m#"),
                Some(Point::Green) => print!("\x1b[0;42mX"),
                Some(Point::Blank) => print!("\x1b[0m."),
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

fn is_filled(map: &TilesMapAsBTree, x: usize, y: usize) -> bool {
    match map.data.get(&(x,y)) {
        Some(Point::Red) => true,
        Some(Point::Green) => true,
        Some(Point::Blank) => false,
        None => false,
    }
}

fn is_rect_filled(map: &TilesMapAsBTree, x1: u64, y1: u64, x2: u64, y2: u64) -> bool {
    let xmin = x1.min(x2);
    let xmax = x1.max(x2);
    let ymin = y1.min(y2);
    let ymax = y1.max(y2);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if !is_filled(&map, x as usize, y as usize) {
                return false;
            }
        }
    }
    true
}

fn find_largest_filled_area(points: &Vec<(u64,u64)>, map: &TilesMapAsBTree) -> u64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            if !is_rect_filled(&map, x1, y1, x2, y2) {
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

    print_map(&map);

    let result = find_largest_filled_area(&tiles, &map);
    assert_eq!(result, 24);
}
