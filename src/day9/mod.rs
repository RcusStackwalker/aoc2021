use crate::utils;
use itertools::Itertools;
use std::collections::HashSet;

static NEIGHBORS: [(i16, i16); 4] = [
    (0_i16, -1_i16),
    (0_i16, 1_i16),
    (-1_i16, 0_i16),
    (1_i16, 0_i16),
];

struct Grid {
    points: Vec<Vec<u8>>,
    width: i16,
    height: i16,
}

struct NeighborIterator<'a> {
    grid: &'a Grid,
    index: usize,
    x: i16,
    y: i16,
}

impl<'a> Iterator for NeighborIterator<'a> {
    type Item = u8;

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
            let p = self.grid.point_unsafe(x, y);
            Some(p)
        }
    }
}

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

impl Grid {
    fn point_unsafe(&self, x: i16, y: i16) -> u8 {
        unsafe {
            self.points
                .get_unchecked(y as usize)
                .get_unchecked(x as usize)
                .clone()
        }
    }

    fn neighbors(&self, x: i16, y: i16) -> NeighborIterator {
        NeighborIterator {
            grid: self,
            index: 0,
            x,
            y,
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

fn read_file_into_grid(path: &str) -> Grid {
    let points = utils::read_file_into_vector(path, |l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|x| x as u8)
            .collect_vec()
    });
    let width = points[0].len();
    let height = points.len();
    Grid {
        points,
        width: width as i16,
        height: height as i16,
    }
}

fn is_lowest(grid: &Grid, x: i16, y: i16) -> bool {
    let p = grid.point_unsafe(x, y);
    grid.neighbors(x, y).find(|&vn| vn <= p).is_none()
}

fn heat_value(grid: &Grid, x: i16, y: i16) -> Option<usize> {
    let p = grid.point_unsafe(x, y);
    let n = grid.neighbors(x, y).find(|&vn| vn <= p);
    if n.is_none() {
        Some((p + 1) as usize)
    } else {
        None
    }
}

fn sum_heat(grid: Grid) -> usize {
    (0..grid.height)
        .cartesian_product(0..grid.width)
        .filter_map(|(y, x)| heat_value(&grid, x as i16, y as i16))
        .sum()
}

fn top3_basin_sizes(grid: Grid) -> usize {
    let basins = (0..grid.height)
        .cartesian_product(0..grid.width)
        .filter(|&(y, x)| is_lowest(&grid, x, y))
        .map(|(y, x)| {
            let mut set: HashSet<(i16, i16)> = HashSet::new();
            set.insert((x, y));
            loop {
                let mut new_points = HashSet::new();
                set.iter()
                    .flat_map(|&(x, y)| grid.neighbor_coordinates(x, y))
                    .for_each(|(x, y)| {
                        if grid.point_unsafe(x, y) < 9 {
                            new_points.insert((x, y));
                        }
                    });
                if set.is_superset(&new_points) {
                    break;
                }
                set = set.union(&new_points).map(|x| *x).collect();
            }
            set.len()
        })
        .sorted()
        .rev()
        .collect_vec();
    basins[0] * basins[1] * basins[2]
}

#[test]
fn task1_example() {
    let values = read_file_into_grid("src/day9/example.txt");
    let result = sum_heat(values);
    println!("D9T1E {}", result);
    assert_eq!(result, 15);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_grid("src/day9/input.txt");
    let result = sum_heat(values);
    println!("D9T1P {}", result);
    assert_eq!(result, 486);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_grid("src/day9/example.txt");
    let result = top3_basin_sizes(values);
    println!("D9T2E {}", result);
    assert_eq!(result, 1134);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_grid("src/day9/input.txt");
    let result = top3_basin_sizes(values);
    println!("D9T2P {}", result);
    assert_eq!(result, 1059300);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
