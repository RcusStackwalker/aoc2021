use itertools::Itertools;
use crate::utils;

#[derive(Debug,Copy,Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug,Copy,Clone)]
struct Line {
    p1: Point,
    p2: Point
}

type PointMap = std::collections::HashMap<Point, usize>;

fn read_file_into_vector(path: &str) -> Vec<Line> {
    utils::read_file_into_vector(path, |l| {
        let mut points = l.split(" -> ").map(|s| {
            let mut pair = s.split(',').map(|x| { x.parse::<usize>().unwrap()});
            Point {
                x: pair.next().unwrap(),
                y: pair.next().unwrap(),
            }});
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
        if l.p1.x == l.p2.x {
            if l.p1.y < l.p2.y {
                l.p1.y..=l.p2.y
            } else {
                l.p2.y..=l.p1.y
            }.for_each(|y| {
                add_point(&mut map, Point {
                    x: l.p1.x,
                    y
                })
            });
        } else if l.p1.y == l.p2.y {
            if l.p1.x < l.p2.x {
                l.p1.x..=l.p2.x
            } else {
                l.p2.x..=l.p1.x
            }.for_each(|x| {
                add_point(&mut map, Point {
                    x,
                    y: l.p1.y
                })
            });
        }
    });
    let result = map.iter().filter(|(_, &count)| {
        count > 1
    }).count();
    result
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day5/example.txt").iter().filter(|&l| {
        l.p1.x == l.p2.x || l.p1.y == l.p2.y
    }).map(|l| *l).collect_vec();
    let result = analyze(values);
    println!("D5T1E {}", result);
    assert_eq!(result, 5);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day5/input.txt").iter().filter(|&l| {
        l.p1.x == l.p2.x || l.p1.y == l.p2.y
    }).map(|l| *l).collect_vec();
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
