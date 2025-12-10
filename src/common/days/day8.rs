use std::collections::BTreeMap;
use itertools::Itertools;
use std::cmp::Ordering;

type Vector = (i64, i64, i64);

#[derive(PartialEq, Eq, Debug)]
struct Pair {
    a: Vector,
    b: Vector,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b) = (self.a, self.b);
        let (c, d) = (other.a, other.b);
        distance_sq(a,b).cmp(&distance_sq(c,d))
    }


}

fn parse_input(input: &str) -> Vec<Vector> {
    input.lines().map(|line| {
        let numbers : Vec<i64> = line.split(',').map(|token| token.trim().parse::<i64>().unwrap()).collect();
        (numbers[0], numbers[1], numbers[2])
    }).collect()
}

fn distance_sq(a: Vector, b: Vector) -> u64 {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;
    ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as u64
}


fn size_three_largest(junctions: Vec<Vector>, max_steps: usize) -> u64 {
    let pairs = junctions.iter().combinations(2).map(|v| Pair{a: *v[0], b: *v[1]});
    let mut pairs = pairs.sorted();
    // println!("Pairs: \n{:?}\n", pairs);
    let mut circuits : Vec<BTreeMap<Vector, ()>> = Vec::with_capacity(junctions.len());
    for _ in 0..max_steps {
        let pair = pairs.next().unwrap();
        let (a, b) = (pair.a, pair.b);
        if a == b {
            continue;
        }
        // println!("Picking {:?}", pair);
        let pos_a = circuits.iter().position(|circuit| circuit.contains_key(&a));
        let pos_b = circuits.iter().position(|circuit| circuit.contains_key(&b));
        match (pos_a, pos_b) {
            (None, None) => {
                circuits.push(BTreeMap::new());
                circuits.last_mut().unwrap().insert(a, ());
                circuits.last_mut().unwrap().insert(b, ());
            }
            (None, Some(pos)) => {
                circuits[pos].insert(a, ());
            }
            (Some(pos), None) => {
                circuits[pos].insert(b, ());
            }
            (Some(pos_a), Some(pos_b)) => {
                if pos_a == pos_b {
                    continue;
                }
                let clone = circuits[pos_b].clone();
                for v in clone.keys() {
                    circuits[pos_a].insert(*v, ());
                }
                circuits.remove(pos_b);
            }
        }
    }
    // println!("Circuits: {:?}", circuits);
    assert!(circuits.len() >= 3);
    let ordered_circuits_lengths = circuits.iter().map(|circuit| circuit.len() as u64).sorted().rev().take(3).collect::<Vec<_>>();
    ordered_circuits_lengths[0] * ordered_circuits_lengths[1] * ordered_circuits_lengths[2]
}

fn distance_to_wall(junctions: Vec<Vector>) -> u64 {
    let pairs = junctions.iter().combinations(2).map(|v| Pair{a: *v[0], b: *v[1]});
    let mut pairs = pairs.sorted();
    // println!("Pairs: \n{:?}\n", pairs);
    let mut circuits : Vec<BTreeMap<Vector, ()>> = Vec::with_capacity(junctions.len());
    // fill the circuits with single junctions:
    for junction in junctions {
        circuits.push(BTreeMap::new());
        circuits.last_mut().unwrap().insert(junction, ());
    }
    let mut previous: Option<Pair> = None;
    while circuits.len() > 1 {
        let pair = pairs.next().unwrap();
        let (a, b) = (pair.a, pair.b);
        if a == b {
            continue;
        }
        // println!("Picking {:?}", pair);
        let pos_a = circuits.iter().position(|circuit| circuit.contains_key(&a));
        let pos_b = circuits.iter().position(|circuit| circuit.contains_key(&b));
        match (pos_a, pos_b) {
            (None, None) => {
                circuits.push(BTreeMap::new());
                circuits.last_mut().unwrap().insert(a, ());
                circuits.last_mut().unwrap().insert(b, ());
            }
            (None, Some(pos)) => {
                circuits[pos].insert(a, ());
            }
            (Some(pos), None) => {
                circuits[pos].insert(b, ());
            }
            (Some(pos_a), Some(pos_b)) => {
                if pos_a == pos_b {
                    continue;
                }
                let clone = circuits[pos_b].clone();
                for v in clone.keys() {
                    circuits[pos_a].insert(*v, ());
                }
                circuits.remove(pos_b);
            }
        }
        previous = Some(pair);
    }
    // println!("Circuits: {:?}", circuits);
    if let Some(pair) = previous {
        (pair.a.0 * pair.b.0) as u64
    } else {
        0
    }
}


pub fn part1(input: &str) -> anyhow::Result<()> {
    let junctions = parse_input(input);
    let result = size_three_largest(junctions, 1000);
    println!("Result: {result}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let junctions = parse_input(input);
    let result = distance_to_wall(junctions);
    println!("Result: {result}");
    Ok(())
}

#[test]
fn test_part1() {
    let input = include_str!("../../../data/test-day8.txt");
    let junctions = parse_input(input);
    let result = size_three_largest(junctions, 10);
    println!("Result: {result}");
    assert_eq!(result, 40);
}

#[test]
fn test_part2() {
    let input = include_str!("../../../data/test-day8.txt");
    let junctions = parse_input(input);
    let result = distance_to_wall(junctions);
    println!("Result: {result}");
    assert_eq!(result, 25272);
}
