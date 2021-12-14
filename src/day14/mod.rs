use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

type Instructions = HashMap<(u8, u8), u8>;
struct Values {
    pairs: HashMap<(u8, u8), usize>,
    counts: HashMap<u8, usize>,
}

impl Values {
    fn least_most_common(&self) -> (usize, usize) {
        let minmax = self.counts.iter().minmax_by_key(|x| x.1);
        match minmax {
            MinMaxResult::MinMax(min, max) => (*min.1, *max.1),
            _ => unreachable!(),
        }
    }

    fn from_vector(values: Vec<u8>) -> Values {
        let mut pairs = HashMap::new();
        values.iter().tuple_windows().for_each(|(p0, p1)| {
            *pairs.entry((*p0, *p1)).or_insert(0) += 1;
        });
        let mut counts = HashMap::new();
        values.into_iter().for_each(|x| {
            *counts.entry(x).or_insert(0) += 1;
        });
        Values { pairs, counts }
    }
}

fn step(values: Values, instructions: &Instructions) -> Values {
    let mut pairs = HashMap::new();
    let mut counts = values.counts.clone();
    values.pairs.iter().for_each(|(p, count)| {
        if let Some(v) = instructions.get(p) {
            *pairs.entry((p.0, *v)).or_default() += count;
            *pairs.entry((*v, p.1)).or_default() += count;
            *counts.entry(*v).or_default() += count;
        } else {
            *pairs.entry(*p).or_default() += count;
        }
    });
    Values { pairs, counts }
}

fn steps(values: Vec<u8>, instructions: Instructions, n: usize) -> Values {
    let mut values = Values::from_vector(values);
    for _ in 0..n {
        values = step(values, &instructions);
    }
    values
}

fn answer(values: Values) -> usize {
    let minmax = values.least_most_common();
    minmax.1 - minmax.0
}

fn read_file(path: &str) -> (Vec<u8>, Instructions) {
    let input = std::fs::read_to_string(path).expect("Missing input data");
    let mut lines = input.lines();
    let values = lines.next().unwrap().bytes().collect();
    lines.next();
    let instructions = lines
        .map(|l| {
            let l = l.split_once(" -> ").unwrap();
            let mut p = l.0.bytes();
            let p0 = p.next().unwrap();
            let p1 = p.next().unwrap();
            ((p0, p1), l.1.bytes().next().unwrap())
        })
        .collect();
    (values, instructions)
}

#[test]
fn task1_example() {
    let (values, instructions) = read_file("src/day14/example.txt");
    let values = steps(values, instructions, 10);
    let result = answer(values);
    println!("D14T1E {}", result);
    assert_eq!(result, 1588);
}

#[test]
fn task1_puzzle() {
    let (values, instructions) = read_file("src/day14/input.txt");
    let result = answer(steps(values, instructions, 10));
    println!("D14T1P {}", result);
    assert_eq!(result, 4517);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let (values, instructions) = read_file("src/day14/example.txt");
    let values = steps(values, instructions, 40);
    let result = answer(values);
    println!("D14T2E {}", result);
    assert_eq!(result, 2188189693529);
}

#[test]
fn task2_puzzle() {
    let (values, instructions) = read_file("src/day14/input.txt");
    let result = answer(steps(values, instructions, 40));
    println!("D14T2P {}", result);
    assert_eq!(result, 4704817645083);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
