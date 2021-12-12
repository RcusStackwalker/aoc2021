use crate::utils;
use itertools::Itertools;
use std::collections::HashMap;

type NodeIndex = u8;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Point {
    Start,
    End,
    Small(NodeIndex),
    Big(NodeIndex),
}
type Edge = (Point, Point);

fn classify(s: &str, map: &mut HashMap<String, NodeIndex>) -> Point {
    if s.chars().next().unwrap().is_uppercase() {
        let len = map.len() as NodeIndex;
        Point::Big(*map.entry(s.to_owned()).or_insert(len))
    } else {
        match s {
            "start" => Point::Start,
            "end" => Point::End,
            _ => {
                let len = map.len() as NodeIndex;
                Point::Small(*map.entry(s.to_owned()).or_insert(len))
            }
        }
    }
}

fn read_file_into_vector(path: &str) -> Vec<Edge> {
    let mut map = HashMap::new();
    utils::read_file_into_vector(path, |l| {
        let p = l.split_once('-').expect("Malformed string");
        let p1 = classify(p.0, &mut map);
        let p2 = classify(p.1, &mut map);
        (p1, p2)
    })
}

#[derive(Copy, Clone)]
struct Path {
    last: Point,
    small_map: u32,
    second_slot_available: bool,
}

impl Path {
    fn new() -> Path {
        Path {
            last: Point::Start,
            small_map: 0,
            second_slot_available: true,
        }
    }

    fn push(&self, p: Point) -> Path {
        let (small_map, second_slot_available) = if let Point::Small(index) = p {
            let mask = 1 << index;
            let second_slot_available = if self.small_map & mask != 0 {
                assert_eq!(self.second_slot_available, true);
                false
            } else {
                self.second_slot_available
            };
            (self.small_map | mask, second_slot_available)
        } else {
            (self.small_map, self.second_slot_available)
        };
        Path {
            last: p,
            small_map,
            second_slot_available,
        }
    }

    fn contains_small(&self, p: NodeIndex) -> bool {
        (self.small_map & (1 << p)) != 0
    }

    fn last(&self) -> Point {
        self.last
    }

    fn finished(&self) -> bool {
        self.last == Point::End
    }
}

fn small_once(path: &Path, point: NodeIndex) -> bool {
    !path.contains_small(point)
}

fn small_twice(path: &Path, point: NodeIndex) -> bool {
    !path.contains_small(point) || path.second_slot_available
}

fn build_path<'a, F>(
    edges: &'a HashMap<Point, Vec<Point>>,
    path: &'a Path,
    small_rule: &'a F,
) -> impl Iterator<Item = Path> + 'a
where
    F: Fn(&Path, NodeIndex) -> bool,
{
    edges
        .get(&path.last())
        .unwrap()
        .iter()
        .filter_map(|&dst| match &dst {
            Point::Start => None,
            Point::End | Point::Big(_) => Some(dst),
            Point::Small(index) => {
                if small_rule(path, *index) {
                    Some(dst)
                } else {
                    None
                }
            }
        })
        .map(|p| path.push(p))
}

fn path_count<F>(v: Vec<Edge>, small_rule: F) -> usize
where
    F: Fn(&Path, NodeIndex) -> bool,
{
    let mut edges = HashMap::new();
    v.into_iter().for_each(|(p1, p2)| {
        if p1 != Point::End && p2 != Point::Start {
            edges.entry(p1.clone()).or_insert(vec![]).push(p2.clone());
        }
        if p2 != Point::End && p1 != Point::Start {
            edges.entry(p2.clone()).or_insert(vec![]).push(p1.clone());
        }
    });
    let mut paths = vec![Path::new()];
    let mut finished_paths = 0;
    while paths.len() != 0 {
        paths = paths
            .iter()
            .flat_map(|path| build_path(&edges, path, &small_rule))
            .filter(|path| {
                if path.finished() {
                    finished_paths += 1;
                    false
                } else {
                    true
                }
            })
            .collect_vec();
    }
    finished_paths
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day12/example.txt");
    let result = path_count(values, small_once);
    println!("D12T1E {}", result);
    assert_eq!(result, 226);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day12/input.txt");
    let result = path_count(values, small_once);
    println!("D12T1P {}", result);
    assert_eq!(result, 5252);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day12/example.txt");
    let result = path_count(values, small_twice);
    println!("D12T2E {}", result);
    assert_eq!(result, 3509);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day12/input.txt");
    let result = path_count(values, small_twice);
    println!("D12T2P {}", result);
    assert_eq!(result, 147784);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
