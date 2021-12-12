use crate::utils;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Point {
    Start,
    End,
    Small(String),
    Big(String),
}
type Edge = (Point, Point);

fn classify(s: &str) -> Point {
    if s.chars().next().unwrap().is_uppercase() {
        Point::Big(s.to_owned())
    } else {
        match s {
            "start" => Point::Start,
            "end" => Point::End,
            _ => Point::Small(s.to_owned()),
        }
    }
}

fn read_file_into_vector(path: &str) -> Vec<Edge> {
    utils::read_file_into_vector(path, |l| {
        let p = l.split_once('-').expect("Malformed string");
        (classify(p.0), classify(p.1))
    })
}

#[derive(Clone)]
struct Path(Vec<Point>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|p| match p {
                Point::Start => "start",
                Point::End => "end",
                Point::Big(s) => s.as_str(),
                Point::Small(s) => s.as_str(),
            })
            .join("-")
            .fmt(f)
    }
}

fn small_once(path: &Path, point: &Point) -> bool {
    !path.0.contains(point)
}

fn small_twice(path: &Path, point: &Point) -> bool {
    let c = !path.0.contains(point);
    if c {
        //eprintln!("accepting {:?} for {}, !contains", point, path);
        return true;
    }
    let nodoubles = path
        .0
        .iter()
        .filter(|&x| match x {
            Point::Small(_) => true,
            _ => false,
        })
        .all(|x| path.0.iter().filter(|&p| p == x).count() < 2);
    if nodoubles {
        //eprintln!("accepting {:?} for {}, !nodoubles", point, path);
    }
    nodoubles
}

fn build_path<F>(edges: &HashMap<Point, Vec<Point>>, path: &Path, small_rule: &F) -> Vec<Path>
where
    F: Fn(&Path, &Point) -> bool,
{
    if let Point::End = path.0.last().unwrap() {
        return vec![path.clone()];
    }
    if path.0.len() > 100 {
        return vec![];
    }
    edges
        .get(path.0.last().unwrap())
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
            //eprintln!("Growing {} by {:?}", path, p);
            path.0.push(p);
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
    let mut paths = vec![Path(vec![Point::Start])];
    let mut finished_paths = vec![];
    loop {
        let mut new_paths = paths
            .iter()
            .flat_map(|path| build_path(&edges, path, &small_rule).into_iter())
            .collect_vec();

        new_paths
            .drain_filter(|p| p.0.last().unwrap() == &Point::End)
            .for_each(|p| finished_paths.push(p));
        if new_paths.len() == 0 {
            break;
        }
        paths = new_paths;
    }
    // finished_paths
    //     .iter()
    //     .filter(|&p| p.0.last().unwrap() == &Point::End)
    //     .sorted_by_key(|&x| x.0.len())
    //     .for_each(|x| eprintln!("{}", x));
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
