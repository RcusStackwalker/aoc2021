use crate::utils;
use std::fs;

const MAX_PERIOD: usize = 8;
const STATE_SIZE: usize = MAX_PERIOD + 1;
pub type SchoolState = [usize; STATE_SIZE];

fn read_file_into_state(path: &str) -> SchoolState {
    let mut ret = [0; STATE_SIZE];
    fs::read_to_string(path)
        .expect("Missing input data")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|x| {
            ret[x] += 1;
        });
    ret
}

fn mutated_state(state: SchoolState) -> SchoolState {
    let mut ret = [0; STATE_SIZE];
    ret[MAX_PERIOD] = state[0];
    for x in 0..MAX_PERIOD {
        ret[x] = state[x + 1];
    }
    ret[6] += state[0];
    ret
}

fn answer(mut state: SchoolState, days: usize) -> usize {
    (0..days).for_each(|_| {
        state = mutated_state(state);
    });
    state.iter().sum()
}

#[test]
fn task1_example() {
    let mut values = read_file_into_state("src/day6/example.txt");
    let result = answer(values, 80);
    println!("D6T1E {}", result);
    assert_eq!(result, 5934);
}

#[test]
fn task1_puzzle() {
    let mut values = read_file_into_state("src/day6/input.txt");
    let result = answer(values, 80);
    println!("D6T1E {}", result);
    assert_eq!(result, 393019);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let mut values = read_file_into_state("src/day6/example.txt");
    let result = answer(values, 256);
    println!("D6T2E {}", result);
    assert_eq!(result, 26984457539);
}

#[test]
fn task2_puzzle() {
    let mut values = read_file_into_state("src/day6/input.txt");
    let result = answer(values, 256);
    println!("D6T2E {}", result);
    assert_eq!(result, 1757714216975);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
