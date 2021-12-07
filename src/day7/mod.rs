use itertools::{Itertools, MinMaxResult};
use std::fs;

fn read_file_into_vector(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .expect("Missing input file")
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn fuel_needed<F>(values: &Vec<isize>, pos: &isize, burn_fn: &F) -> isize
where
    F: Fn(isize) -> isize,
{
    values
        .iter()
        .map(|x| if x < pos { pos - x } else { x - pos })
        .map(burn_fn)
        .sum()
}

fn least_fuel<F>(values: Vec<isize>, burn_fn: F) -> isize
where
    F: Fn(isize) -> isize,
{
    let minmax = values.iter().minmax();
    match minmax {
        MinMaxResult::MinMax(min, max) => (min.clone()..=max.clone())
            .map(|x| fuel_needed(&values, &x, &burn_fn))
            .min()
            .unwrap(),
        _ => 0,
    }
}

fn linear_burn(delta: isize) -> isize {
    delta
}

fn progressive_burn(delta: isize) -> isize {
    delta * (delta + 1) / 2
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day7/example.txt");
    let result = least_fuel(values, linear_burn);
    println!("D7T1E {}", result);
    assert_eq!(result, 37);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day7/input.txt");
    let result = least_fuel(values, linear_burn);
    println!("D7T1P {}", result);
    assert_eq!(result, 333755);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day7/example.txt");
    let result = least_fuel(values, progressive_burn);
    println!("D7T2E {}", result);
    assert_eq!(result, 168);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day7/input.txt");
    let result = least_fuel(values, progressive_burn);
    println!("D7T2P {}", result);
    assert_eq!(result, 94017638);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
