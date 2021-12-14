use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

type Instructions = HashMap<(u8, u8), u8>;

fn step(values: Vec<u8>, instructions: &Instructions) -> Vec<u8> {
    let mut ret = Vec::with_capacity(values.len() * 2);
    values.iter().tuple_windows().for_each(|(p0, p1)| {
        ret.push(*p0);
        if let Some(v) = instructions.get(&(*p0, *p1)) {
            ret.push(*v);
        }
    });
    ret.push(*values.last().unwrap());
    ret
}

fn steps(mut values: Vec<u8>, instructions: Instructions, n: usize) -> Vec<u8> {
    for _ in 0..n {
        values = step(values, &instructions);
    }
    values
}

fn answer(values: Vec<u8>) -> usize {
    let mut map = HashMap::new();
    values.iter().for_each(|x| {
        *map.entry(*x).or_insert(0) += 1;
    });
    let minmax = map.iter().minmax_by_key(|x| x.1);
    match minmax {
        MinMaxResult::MinMax(min, max) => max.1 - min.1,
        _ => unreachable!(),
    }
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
    assert_eq!(values.len(), 3073);
    let result = answer(values);
    println!("D14T1E {}", result);
    assert_eq!(result, 1588);
}

#[test]
fn task1_puzzle() {
    let (values, instructions) = read_file("src/day14/input.txt");
    let result = answer(steps(values, instructions, 10));
    println!("D13T1P {}", result);
    assert_eq!(result, 4517);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
