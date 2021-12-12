use crate::utils;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Point {
    Start,
    End,
    Small(usize),
    Big(usize),
}
type Edge = (Point, Point);

fn classify(s: &str, map: &mut HashMap<String, usize>) -> Point {
    if s.chars().next().unwrap().is_uppercase() {
        let len = map.len();
        Point::Big(*map.entry(s.to_owned()).or_insert(len))
    } else {
        match s {
            "start" => Point::Start,
            "end" => Point::End,
            _ => {
                let len = map.len();
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

#[derive(Clone)]
struct Path {
    points: Vec<Point>,
    map: HashMap<Point, usize>,
}

impl Path {
    fn new() -> Path {
        let mut ret = Path {
            points: vec![],
            map: HashMap::new(),
        };
        ret.push(Point::Start);
        ret
    }

    fn push(&mut self, p: Point) -> () {
        *self.map.entry(p.clone()).or_default() += 1;
        self.points.push(p);
    }

    fn count(&self, p: &Point) -> usize {
        self.map.get(p).unwrap_or(&0_usize).clone()
    }

    fn last(&self) -> Option<&Point> {
        self.points.last()
    }
}

fn small_once(path: &Path, point: &Point) -> bool {
    path.count(point) == 0
}

fn small_twice(path: &Path, point: &Point) -> bool {
    if path.count(point) == 0 {
        return true;
    }
    path.map.iter().all(|(point, count)| match point {
        Point::Small(_) => *count < 2,
        _ => true,
    })
}

fn build_path<F>(edges: &HashMap<Point, Vec<Point>>, path: &Path, small_rule: &F) -> Vec<Path>
where
    F: Fn(&Path, &Point) -> bool,
{
    assert_ne!(path.last(), Some(&Point::End));
    edges
        .get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter_map(|dst| match dst {
            Point::Start => None,
            Point::End | Point::Big(_) => Some(dst.clone()),
            Point::Small(_) => {
                if small_rule(path, dst) {
                    Some(dst.clone())
                } else {
                    None
                }
            }
        })
        .map(|p| {
            let mut path = path.clone();
            path.push(p);
            path
        })
        .collect()
}

fn path_count<F>(v: Vec<Edge>, small_rule: F) -> usize
where
    F: Fn(&Path, &Point) -> bool,
{
    let mut edges = HashMap::new();
    v.into_iter().for_each(|(p1, p2)| {
        edges.entry(p1.clone()).or_insert(vec![]).push(p2.clone());
        edges.entry(p2.clone()).or_insert(vec![]).push(p1.clone());
    });
    let mut paths = vec![Path::new()];
    let mut finished_paths = vec![];
    loop {
        let mut new_paths = paths
            .iter()
            .flat_map(|path| build_path(&edges, path, &small_rule).into_iter())
            .collect_vec();

        new_paths
            .drain_filter(|p| p.last().unwrap() == &Point::End)
            .for_each(|p| finished_paths.push(p));
        if new_paths.len() == 0 {
            break;
        }
        paths = new_paths;
    }
    finished_paths.len()
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
