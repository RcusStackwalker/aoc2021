use itertools::Itertools;
use std::fs;
use std::ops::Add;

fn number_of_inc_from_iter<I>(it: I) -> usize
where
    I: Sized + Iterator,
    <I as Iterator>::Item: Clone + PartialOrd,
{
    it.tuple_windows().filter(|(l, r)| l < r).count()
}

fn number_of_inc<T: Ord>(data: &Vec<T>) -> usize {
    number_of_inc_from_iter(data.iter())
}

fn number_of_inc_sliding_window3<T>(data: &Vec<T>) -> usize
where
    T: Ord + Add<Output = T> + Clone,
{
    number_of_inc_from_iter(
        data.iter()
            .tuple_windows()
            .map(|(a, b, c)| a.clone() + b.clone() + c.clone()),
    )
}

#[test]
fn task1_example_data() {
    let data = fs::read_to_string("src/day1/task1_example_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc(&values);
    println!("D1T1E {}", result);
    assert_eq!(result, 7);
}

#[test]
fn task1_puzzle() {
    let data = fs::read_to_string("src/day1/task1_puzzle_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc(&values);
    println!("D1T1P {}", result);
    assert_eq!(result, 1475);
}

#[test]
fn task2_example_data() {
    let data = fs::read_to_string("src/day1/task1_example_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc_sliding_window3(&values);
    println!("D1T2E {}", result);
    assert_eq!(result, 5);
}

#[test]
fn task2_puzzle_data() {
    let data = fs::read_to_string("src/day1/task1_puzzle_data.txt").expect("input data missing");
    let values = Vec::from_iter(data.lines().map(|l| l.parse::<usize>().unwrap()));
    let result = number_of_inc_sliding_window3(&values);
    println!("D1T2P {}", result);
    assert_eq!(result, 1516);
}
