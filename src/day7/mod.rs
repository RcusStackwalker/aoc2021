use itertools::{Itertools, MinMaxResult};
use std::fs;

fn read_file_into_vector(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .expect("Missing input file")
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn fuel_needed(values: &Vec<isize>, pos: &isize) -> isize {
    values
        .iter()
        .map(|x| if x < pos { pos - x } else { x - pos })
        .sum()
}

fn least_fuel(values: Vec<isize>) -> isize {
    let minmax = values.iter().minmax();
    match minmax {
        MinMaxResult::MinMax(min, max) => (min.clone()..=max.clone())
            .map(|x| fuel_needed(&values, &x))
            .min()
            .unwrap(),
        _ => 0,
    }
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day7/example.txt");
    let result = least_fuel(values);
    println!("D7T1E {}", result);
    assert_eq!(result, 37);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day7/input.txt");
    let result = least_fuel(values);
    println!("D7T1P {}", result);
    assert_eq!(result, 333755);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
