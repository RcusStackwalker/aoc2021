use crate::utils;
use itertools::Itertools;

static NEIGHBORS: [(i16, i16); 4] = [
    (0_i16, -1_i16),
    (0_i16, 1_i16),
    (-1_i16, 0_i16),
    (1_i16, 0_i16),
];

struct Grid {
    points: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

struct NeighborIterator<'a> {
    grid: &'a Grid,
    index: usize,
    x: i16,
    y: i16,
}

impl<'a> Iterator for NeighborIterator<'a> {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = NEIGHBORS.get(self.index) {
            self.index += 1;
            let x = self.x + n.0;
            let y = self.y + n.1;
            if x < 0 || y < 0 || x as usize >= self.grid.width || y as usize >= self.grid.height {
                self.next()
            } else {
                Some((x, y))
            }
        } else {
            None
        }
    }
}

impl Grid {
    fn point(&self, x: i16, y: i16) -> u8 {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            10
        } else {
            unsafe {
                self.points
                    .get_unchecked(y as usize)
                    .get_unchecked(x as usize)
                    .clone()
            }
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
        width,
        height,
    }
}

fn heat_value(grid: &Grid, x: i16, y: i16) -> usize {
    let p = grid.point(x, y);
    let n = NEIGHBORS
        .iter()
        .find(|&delta| grid.point(x + delta.0, y + delta.1) <= p);
    if n.is_none() {
        (p + 1) as usize
    } else {
        0_usize
    }
}

fn sum_heat(grid: Grid) -> usize {
    (0..grid.height)
        .cartesian_product(0..grid.width)
        .map(|(y, x)| heat_value(&grid, x as i16, y as i16))
        .sum()
}

fn basin_size(grid: &Grid, lowest_x: i16, lowest_y: i16) -> usize {
    todo!()
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
