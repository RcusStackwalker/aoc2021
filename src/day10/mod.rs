use crate::utils;
use itertools::Itertools;

enum ParseResult {
    IllegalCharacter(char),
    Incomplete(Vec<char>),
}

fn read_file_into_vector(path: &str) -> Vec<String> {
    utils::read_file_into_vector(path, |s| s.to_owned())
}

fn parse(input: &String) -> ParseResult {
    let mut stack = Vec::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.last() != Some(&'(') {
                    return ParseResult::IllegalCharacter(c);
                } else {
                    stack.pop();
                }
            }
            ']' => {
                if stack.last() != Some(&'[') {
                    return ParseResult::IllegalCharacter(c);
                } else {
                    stack.pop();
                }
            }
            '}' => {
                if stack.last() != Some(&'{') {
                    return ParseResult::IllegalCharacter(c);
                } else {
                    stack.pop();
                }
            }
            '>' => {
                if stack.last() != Some(&'<') {
                    return ParseResult::IllegalCharacter(c);
                } else {
                    stack.pop();
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    ParseResult::Incomplete(stack)
}

fn score_illegal_character(input: char) -> usize {
    static POINTS: [(char, usize); 4] = [
        (')', 3_usize),
        (']', 57_usize),
        ('}', 1197_usize),
        ('>', 25137_usize),
    ];

    POINTS
        .iter()
        .find_map(|&(c, s)| if c == input { Some(s) } else { None })
        .unwrap_or_default()
}

fn score_incomplete(prev: usize, input: &char) -> usize {
    static POINTS: [(char, usize); 4] = [
        ('(', 1_usize),
        ('[', 2_usize),
        ('{', 3_usize),
        ('<', 4_usize),
    ];
    prev * 5
        + POINTS
            .iter()
            .find_map(|&(c, s)| if &c == input { Some(s) } else { None })
            .unwrap_or_default()
}

fn syntax_error_score(input: Vec<String>) -> usize {
    input
        .iter()
        .filter_map(|s| {
            if let ParseResult::IllegalCharacter(c) = parse(s) {
                Some(c)
            } else {
                None
            }
        })
        .map(score_illegal_character)
        .sum()
}

fn completion_score(input: Vec<String>) -> usize {
    let results = input
        .iter()
        .filter_map(|s| {
            if let ParseResult::Incomplete(stack) = parse(s) {
                Some(stack)
            } else {
                None
            }
        })
        .map(|stack| stack.iter().rev().fold(0_usize, score_incomplete))
        .sorted()
        .collect_vec();
    results[results.len() / 2]
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day10/example.txt");
    let result = syntax_error_score(values);
    println!("D10T1E {}", result);
    assert_eq!(result, 26397);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day10/input.txt");
    let result = syntax_error_score(values);
    println!("D10T1P {}", result);
    assert_eq!(result, 278475);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day10/example.txt");
    let result = completion_score(values);
    println!("D10T1E {}", result);
    assert_eq!(result, 288957);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day10/input.txt");
    let result = completion_score(values);
    println!("D10T1P {}", result);
    assert_eq!(result, 3015539998);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
