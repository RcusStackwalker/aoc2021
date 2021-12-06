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

fn answer(state: SchoolState) -> usize {
    state.iter().sum()
}

#[test]
fn task1_example() {
    let mut values = read_file_into_state("src/day6/example.txt");
    (0..80).for_each(|_| {
        values = mutated_state(values);
    });
    let result = answer(values);
    println!("D6T1E {}", result);
    assert_eq!(result, 5934);
}

#[test]
fn task1_puzzle() {
    let mut values = read_file_into_state("src/day6/input.txt");
    (0..80).for_each(|_| {
        values = mutated_state(values);
    });
    let result = answer(values);
    println!("D6T1E {}", result);
    assert_eq!(result, 393019);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
