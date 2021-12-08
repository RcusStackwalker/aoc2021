use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn read_file_into_vectors(path: &str) -> Vec<(HashSet<String>, Vec<String>)> {
    let data = fs::read_to_string(path).expect("Input data mising");
    let lines = data.lines();
    lines
        .map(|l| {
            let mut it = l.split('|');
            let v0 = it
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .sorted_by(|l, r| l.len().cmp(&r.len()))
                .collect();
            let v1 = it
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .collect();
            (v0, v1)
        })
        .collect()
}

fn contains_pattern(s: &str, pat: &str) -> bool {
    pat.chars().all(|c| s.contains(c))
}

fn map_values(mut values: HashSet<String>) -> Option<Vec<String>> {
    //regardless of contents
    // len=2 - 1,
    // len=3 - 7,
    // len=4 - 4,
    // len=7 - 8
    // len=5 - 2,3,5, only 3 contains 1
    // len=6 - 0,6,9 - only 6 doesn't contain full 1
    // out of 0,9 only 9 contains 3, 0 is the other
    // 5 is contained in 6, 2 is the other
    let mut map = vec![String::new(); 10];
    map[1] = values.drain_filter(|s| s.len() == 2).next()?;
    map[7] = values.drain_filter(|s| s.len() == 3).next()?;
    map[4] = values.drain_filter(|s| s.len() == 4).next()?;
    map[8] = values.drain_filter(|s| s.len() == 7).next()?;
    let v1 = map[1].clone();
    map[3] = values
        .drain_filter(|s| s.len() == 5 && contains_pattern(s, v1.as_str()))
        .next()?;
    let v3 = map[3].clone();
    map[6] = values
        .drain_filter(|s| s.len() == 6 && !contains_pattern(s, v1.as_str()))
        .next()?;
    let v6 = map[6].clone();
    map[9] = values
        .drain_filter(|s| s.len() == 6 && contains_pattern(s, v3.as_str()))
        .next()?;
    map[0] = values.drain_filter(|s| s.len() == 6).next()?;
    map[5] = values
        .drain_filter(|s| contains_pattern(v6.as_str(), s.as_str()))
        .next()?;
    map[2] = values.drain().next().unwrap();
    assert!(values.is_empty());
    Some(map)
}

fn count_digits<P>(line: (HashSet<String>, Vec<String>), predicate: P) -> usize
where
    P: Fn(&usize) -> bool,
{
    let map = map_values(line.0).expect("Couldn't resolve mapping");
    let mut hash = HashMap::new();
    map.into_iter().enumerate().for_each(|(i, s)| {
        hash.insert(s, i);
    });
    line.1
        .iter()
        .map(|s| hash.get(s.as_str()).expect("Unknown pattern"))
        .filter(|&x| predicate(x))
        .count()
}

fn get_value(line: (HashSet<String>, Vec<String>)) -> usize {
    let map = map_values(line.0).expect("Couldn't resolve mapping");
    let mut hash = HashMap::new();
    map.into_iter().enumerate().for_each(|(i, s)| {
        hash.insert(s, i);
    });
    line.1
        .iter()
        .rev().enumerate()
        .map(|(i,s)| 10_usize.pow(i as u32) * hash.get(s.as_str()).expect("Unknown pattern"))
        .sum()
}


fn simple_digit(d: &usize) -> bool {
    match d {
        1 | 4 | 7 | 8 => true,
        _ => false,
    }
}

#[test]
fn task1_example() {
    let lines = read_file_into_vectors("src/day8/example.txt");
    let result: usize = lines
        .into_iter()
        .map(|l| count_digits(l, simple_digit))
        .sum();
    println!("D8T1E {}", result);
    assert_eq!(result, 26);
}

#[test]
fn task1_puzzle() {
    let lines = read_file_into_vectors("src/day8/input.txt");
    let result: usize = lines
        .into_iter()
        .map(|l| count_digits(l, simple_digit))
        .sum();
    println!("D8T1P {}", result);
    assert_eq!(result, 367);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let lines = read_file_into_vectors("src/day8/example.txt");
    let result: usize = lines
        .into_iter()
        .map(|l| get_value(l))
        .sum();
    println!("D8T2E {}", result);
    assert_eq!(result, 61229);
}

#[test]
fn task2_puzzle() {
    let lines = read_file_into_vectors("src/day8/input.txt");
    let result: usize = lines
        .into_iter()
        .map(|l| get_value(l))
        .sum();
    println!("D8T1P {}", result);
    assert_eq!(result, 974512);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}