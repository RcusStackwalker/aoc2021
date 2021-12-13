use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
    X(usize),
    Y(usize),
}

type Instructions = Vec<Instruction>;

type Point = (usize, usize);

type Grid = HashSet<Point>;

fn read_file(path: &str) -> (Grid, Instructions) {
    let input = std::fs::read_to_string(path).expect("Missing input data");
    let lines = input.lines().collect_vec();
    let mut blocks = lines.split(|&s| s.is_empty()).into_iter();
    let grid: Grid = blocks
        .next()
        .expect("Missing points")
        .iter()
        .map(|&l| {
            let l = l.split_once(',').unwrap();
            (l.0.parse::<usize>().unwrap(), l.1.parse::<usize>().unwrap())
        })
        .collect();
    let instructions = blocks
        .next()
        .expect("Missing instructions")
        .iter()
        .map(|&l| {
            let l = l.split_once('=').unwrap();
            let coord = l.1.parse::<usize>().unwrap();
            match l.0 {
                "fold along x" => Instruction::X(coord),
                "fold along y" => Instruction::Y(coord),
                _ => panic!("Unexpected fold"),
            }
        })
        .collect();
    (grid, instructions)
}

fn step_once(grid: Grid, instruction: Instruction) -> Grid {
    grid.into_iter()
        .map(|(x, y)| match instruction {
            Instruction::X(c) => {
                if x < c {
                    (x, y)
                } else {
                    (2 * c - x, y)
                }
            }
            Instruction::Y(c) => {
                if y < c {
                    (x, y)
                } else {
                    (x, 2 * c - y)
                }
            }
        })
        .collect()
}

fn result_once(grid: Grid, instructions: Instructions) -> usize {
    assert!(instructions.len() > 0);
    step_once(grid, instructions[0]).len()
}

#[test]
fn task1_example() {
    let (values, instructions) = read_file("src/day13/example.txt");
    let result = result_once(values, instructions);
    println!("D13T1E {}", result);
    assert_eq!(result, 17);
}

#[test]
fn task1_puzzle() {
    let (values, instructions) = read_file("src/day13/input.txt");
    let result = result_once(values, instructions);
    println!("D13T1P {}", result);
    assert_eq!(result, 693);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
