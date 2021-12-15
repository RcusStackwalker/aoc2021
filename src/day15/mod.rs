use crate::utils;
use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Copy, Clone)]
struct Point {
    cost: u8,
}

impl Point {
    fn new(cost: u8) -> Point {
        Point { cost }
    }
}

struct Grid {
    points: Vec<Vec<Point>>,
    width: i16,
    height: i16,
}

impl Grid {
    fn point_unsafe(&self, x: i16, y: i16) -> &Point {
        unsafe {
            self.points
                .get_unchecked(y as usize)
                .get_unchecked(x as usize)
        }
    }

    fn neighbor_coordinates(&self, x: i16, y: i16) -> NeighborCoordinatesIterator {
        NeighborCoordinatesIterator {
            grid: self,
            index: 0,
            x,
            y,
        }
    }
}

static NEIGHBORS: [(i16, i16); 4] = [
    (0_i16, -1_i16),
    (0_i16, 1_i16),
    (-1_i16, 0_i16),
    (1_i16, 0_i16),
];

struct NeighborCoordinatesIterator<'a> {
    grid: &'a Grid,
    index: usize,
    x: i16,
    y: i16,
}

impl<'a> Iterator for NeighborCoordinatesIterator<'a> {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= NEIGHBORS.len() {
            return None;
        }
        let n = NEIGHBORS[self.index];
        self.index += 1;
        let x = self.x + n.0;
        let y = self.y + n.1;
        if x < 0 || y < 0 || x >= self.grid.width || y >= self.grid.height {
            self.next()
        } else {
            Some((x, y))
        }
    }
}

fn read_file(path: &str) -> Grid {
    let points = utils::read_file_into_vector(path, |l| {
        l.chars()
            .map(|x| Point::new(x.to_digit(10).unwrap() as u8))
            .collect_vec()
    });
    let width = points.get(0).unwrap().len() as i16;
    let height = points.len() as i16;
    Grid {
        points,
        width,
        height,
    }
}

use std::cmp::Reverse;
type UnvisitedQueue = PriorityQueue<(i16, i16), std::cmp::Reverse<usize>>;

struct VisitedGrid {
    flags: Vec<bool>,
    width: usize,
}

impl VisitedGrid {
    fn new(width: usize, height: usize) -> VisitedGrid {
        VisitedGrid {
            flags: vec![false; width * height],
            width,
        }
    }
    fn is_visited(&self, x: i16, y: i16) -> bool {
        self.flags[y as usize * self.width + x as usize]
    }
    fn set_visited(&mut self, x: i16, y: i16) -> () {
        self.flags[y as usize * self.width + x as usize] = true;
    }
}

fn find_path(grid: Grid) -> usize {
    let mut frontier = UnvisitedQueue::new();
    frontier.push((0, 0), Reverse(0));
    let mut visited = VisitedGrid::new(grid.width as usize, grid.height as usize);
    while !frontier.is_empty() {
        let (p, dist) = frontier.pop().unwrap();
        if p.0 == grid.width - 1 && p.1 == grid.height - 1 {
            return dist.0;
        }
        visited.set_visited(p.0, p.1);
        grid.neighbor_coordinates(p.0, p.1)
            .filter(|(x, y)| !visited.is_visited(*x, *y))
            .for_each(|(x, y)| {
                let alt = grid.point_unsafe(x, y).cost as usize + dist.0;
                frontier.push_increase((x, y), Reverse(alt));
            });
    }
    panic!("Path not found");
}

fn wrap(cost: u8) -> u8 {
    ((cost - 1) % 9) + 1
}

fn expand(grid: Grid) -> Grid {
    let width = grid.width;
    let height = grid.height;
    let expanded_right = grid
        .points
        .into_iter()
        .map(|l| {
            (0..5)
                .cartesian_product(l.into_iter())
                .map(|(it, point)| Point {
                    cost: wrap(point.cost + it),
                })
                .collect_vec()
        })
        .collect_vec();
    let points = (0..5)
        .cartesian_product(expanded_right.into_iter())
        .map(|(it, line)| {
            line.into_iter()
                .map(|p| Point {
                    cost: wrap(p.cost + it),
                })
                .collect_vec()
        })
        .collect_vec();
    Grid {
        points,
        width: width * 5,
        height: height * 5,
    }
}

#[test]
fn task1_example() {
    let grid = read_file("src/day15/example.txt");
    let result = find_path(grid);
    println!("D15T1E {}", result);
    assert_eq!(result, 40);
}

#[test]
fn task1_puzzle() {
    let grid = read_file("src/day15/input.txt");
    let result = find_path(grid);
    println!("D15T1P {}", result);
    assert_eq!(result, 656);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let grid = read_file("src/day15/example.txt");
    let grid = expand(grid);
    grid.points.iter().for_each(|l| {
        l.iter().for_each(|x| eprint!("{}", x.cost));
        eprintln!();
    });
    let result = find_path(grid);
    println!("D15T2E {}", result);
    assert_eq!(result, 315);
}

#[test]
fn task2_puzzle() {
    let grid = read_file("src/day15/input.txt");
    let result = find_path(expand(grid));
    println!("D15T2P {}", result);
    assert_eq!(result, 2979);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
