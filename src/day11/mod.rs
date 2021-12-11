use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
enum Cell {
    Energy(u8),
    Flash,
}

static NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const GRID_SIZE: usize = 10;

type Grid = [[Cell; GRID_SIZE]; GRID_SIZE];

fn read_file_into_grid(path: &str) -> Grid {
    std::fs::read_to_string(path)
        .expect("Input data missing")
        .lines()
        .map(|x| {
            let line: [Cell; 10] = x
                .chars()
                .map(|c| Cell::Energy(c.to_digit(10).unwrap() as u8))
                .collect_vec()
                .as_slice()
                .try_into()
                .unwrap();
            line
        })
        .collect_vec()
        .as_slice()
        .try_into()
        .unwrap()
}

fn mutate(grid: &mut Grid) -> usize {
    let mut flashes = HashSet::new();
    (0..GRID_SIZE)
        .cartesian_product(0..GRID_SIZE)
        .for_each(|(y, x)| {
            let c = &mut grid[y][x];
            grid[y][x] = match c {
                Cell::Energy(e) => {
                    if *e < 9_u8 {
                        Cell::Energy(*e + 1)
                    } else {
                        flashes.insert((x, y));
                        Cell::Flash
                    }
                }
                Cell::Flash => panic!("Flashes should have been cleared"),
            }
        });
    while flashes.len() > 0 {
        let mut new_flashes = HashSet::new();
        flashes
            .iter()
            .flat_map(|&(x, y)| {
                NEIGHBORS.iter().filter_map(move |&(dx, dy)| {
                    if x == 0 && dx < 0
                        || y == 0 && dy < 0
                        || x == GRID_SIZE - 1 && dx > 0
                        || y == GRID_SIZE - 1 && dy > 0
                    {
                        None
                    } else {
                        Some(((x as isize + dx) as usize, (y as isize + dy) as usize))
                    }
                })
            })
            .for_each(|(x, y)| {
                if let Cell::Energy(e) = grid[y][x] {
                    grid[y][x] = if e < 9 {
                        Cell::Energy(e + 1)
                    } else {
                        new_flashes.insert((x, y));
                        Cell::Flash
                    }
                }
            });
        flashes = new_flashes;
    }
    //last
    let mut ret = 0;
    (0..GRID_SIZE)
        .cartesian_product(0..GRID_SIZE)
        .for_each(|(y, x)| {
            if let Cell::Flash = grid[y][x] {
                grid[y][x] = Cell::Energy(0);
                ret += 1;
            }
        });
    ret
}

fn step(mut grid: Grid, steps: usize) -> usize {
    (0..steps).map(|_| mutate(&mut grid)).sum()
}

fn step_to_flash_all(mut grid: Grid) -> usize {
    let mut ret = 0;
    loop {
        ret += 1;
        if mutate(&mut grid) == 100 {
            break;
        }
    }
    ret
}

#[test]
fn task1_example() {
    let grid = read_file_into_grid("src/day11/example.txt");
    let result = step(grid, 100);
    println!("D11T1E {}", result);
    assert_eq!(result, 1656);
}

#[test]
fn task1_puzzle() {
    let grid = read_file_into_grid("src/day11/input.txt");
    let result = step(grid, 100);
    println!("D11T1P {}", result);
    assert_eq!(result, 1686);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let grid = read_file_into_grid("src/day11/example.txt");
    let result = step_to_flash_all(grid);
    println!("D11T2E {}", result);
    assert_eq!(result, 195);
}

#[test]
fn task2_puzzle() {
    let grid = read_file_into_grid("src/day11/input.txt");
    let result = step_to_flash_all(grid);
    println!("D11T2P {}", result);
    assert_eq!(result, 360);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
