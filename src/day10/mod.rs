use crate::utils;

static POINTS: [(char, usize); 4] = [
    (')', 3_usize),
    (']', 57_usize),
    ('}', 1197_usize),
    ('>', 25137_usize),
];

fn read_file_into_vector(path: &str) -> Vec<String> {
    utils::read_file_into_vector(path, |s| s.to_owned())
}

fn first_illegal_character(input: &String) -> Option<char> {
    let mut stack = Vec::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.last() != Some(&'(') {
                    return Some(c);
                } else {
                    stack.pop();
                }
            }
            ']' => {
                if stack.last() != Some(&'[') {
                    return Some(c);
                } else {
                    stack.pop();
                }
            }
            '}' => {
                if stack.last() != Some(&'{') {
                    return Some(c);
                } else {
                    stack.pop();
                }
            }
            '>' => {
                if stack.last() != Some(&'<') {
                    return Some(c);
                } else {
                    stack.pop();
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    None
}

fn score_illegal_character(input: char) -> usize {
    POINTS
        .iter()
        .find_map(|&(c, s)| if c == input { Some(s) } else { None })
        .unwrap_or_default()
}

fn syntax_error_score(input: Vec<String>) -> usize {
    input
        .iter()
        .filter_map(first_illegal_character)
        .map(score_illegal_character)
        .sum()
}

#[test]
fn test_scoring() {
    assert_eq!(score_illegal_character(')'), 3_usize);
    assert_eq!(score_illegal_character(']'), 57_usize);
    assert_eq!(score_illegal_character('}'), 1197_usize);
    assert_eq!(score_illegal_character('>'), 25137_usize);
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
