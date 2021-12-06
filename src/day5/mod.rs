use crate::utils;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

type PointMap = std::collections::HashMap<Point, usize>;

fn read_file_into_vector(path: &str) -> Vec<Line> {
    utils::read_file_into_vector(path, |l| {
        let mut points = l.split(" -> ").map(|s| {
            let mut pair = s.split(',').map(|x| x.parse::<isize>().unwrap());
            Point {
                x: pair.next().unwrap(),
                y: pair.next().unwrap(),
            }
        });
        Line {
            p1: points.next().unwrap(),
            p2: points.next().unwrap(),
        }
    })
}

fn add_point(map: &mut PointMap, p: Point) {
    *map.entry(p).or_insert(0) += 1;
}

fn analyze(values: Vec<Line>) -> usize {
    let mut map = PointMap::new();
    values.iter().for_each(|l| {
        let dx = match l.p1.x.cmp(&l.p2.x) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let dy = match l.p1.y.cmp(&l.p2.y) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let mut p = l.p1;
        while p != l.p2 {
            add_point(&mut map, p);
            p.x += dx;
            p.y += dy;
        }
        add_point(&mut map, l.p2);
    });
    let result = map.iter().filter(|(_, &count)| count > 1).count();
    result
}

fn read_vertical_horizontal_lines_into_vector(path: &str) -> Vec<Line> {
    read_file_into_vector(path)
        .iter()
        .filter(|&l| l.p1.x == l.p2.x || l.p1.y == l.p2.y)
        .map(|l| *l)
        .collect()
}

#[test]
fn task1_example() {
    let values = read_vertical_horizontal_lines_into_vector("src/day5/example.txt");
    let result = analyze(values);
    println!("D5T1E {}", result);
    assert_eq!(result, 5);
}

#[test]
fn task1_puzzle() {
    let values = read_vertical_horizontal_lines_into_vector("src/day5/input.txt");
    let result = analyze(values);
    println!("D5T1P {}", result);
    assert_eq!(result, 6841);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day5/example.txt");
    let result = analyze(values);
    println!("D5T2E {}", result);
    assert_eq!(result, 12);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day5/input.txt");
    let result = analyze(values);
    println!("D5T2P {}", result);
    assert_eq!(result, 19258);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
