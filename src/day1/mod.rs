use itertools::Itertools;
use std::fs;

fn number_of_inc<T: Ord>(data: &Vec<T>) -> usize {
    data.iter().tuple_windows().filter(|(l, r)| l < r).count()
}

#[test]
fn task1_example_data() {
    let data = fs::read_to_string("src/day1/task1_example_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc(&values);
    println!("{}", result);
    assert_eq!(result, 7);
}

#[test]
fn task1_puzzle() {
    let data = fs::read_to_string("src/day1/task1_puzzle_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc(&values);
    println!("{}", result);
    assert_eq!(result, 1475);
}
