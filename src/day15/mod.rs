use crate::utils;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::HashSet;

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

fn find_path(grid: Grid) -> usize {
    let mut frontier = UnvisitedQueue::new();
    frontier.push((0, 0), Reverse(0));
    let mut visited = HashSet::new();
    while !frontier.is_empty() {
        let (p, dist) = frontier.pop().unwrap();
        if p.0 == grid.width - 1 && p.1 == grid.height - 1 {
            return dist.0;
        }
        visited.insert(p.clone());
        grid.neighbor_coordinates(p.0, p.1)
            .filter(|(x, y)| !visited.contains(&(*x, *y)))
            .for_each(|(x, y)| {
                let alt = grid.point_unsafe(x, y).cost as usize + dist.0;
                frontier.push_increase((x, y), Reverse(alt));
            });
    }
    panic!("Path not found");
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
    assert_eq!(result, 4517);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}
