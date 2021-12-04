use itertools::Itertools;
use std::fs;

const BOARD_SIZE: usize = 5;

type Draws = Vec<usize>;
type IndexMap = std::collections::HashMap<usize, Vec<(usize, usize, usize)>>;

struct Board {
    row_hits: [usize; BOARD_SIZE],
    column_hits: [usize; BOARD_SIZE],
    rows: [[(usize, bool); BOARD_SIZE]; BOARD_SIZE],
    won: bool
}

struct Game {
    index: IndexMap,
    draws: Draws,
    boards: Vec<Board>,
}

impl<'a> Board {
    pub fn new<I>(mut it: I) -> Board
    where
        I: Iterator<Item = &'a str>,
    {
        let mut rows = [[(0, false); BOARD_SIZE]; BOARD_SIZE];
        let mut y = 0;
        loop {
            match it.next() {
                None => break,
                Some(s) => s.split_ascii_whitespace().enumerate().for_each(|(x, s)| {
                    rows[y][x].0 = s.parse::<usize>().unwrap();
                }),
            }
            y += 1;
        }
        Board {
            row_hits: [0; BOARD_SIZE],
            column_hits: [0; BOARD_SIZE],
            rows,
            won: false
        }
    }

    pub fn apply(&mut self, x: usize, y: usize) -> Option<usize> {
        if self.won {
            return None;
        }
        self.row_hits[y] += 1;
        self.column_hits[x] += 1;
        self.rows[y][x].1 = true;
        if self.row_hits[y] == BOARD_SIZE || self.column_hits[x] == BOARD_SIZE {
            let unmarked: usize = self
                .rows
                .iter()
                .map(|row| {
                    row.iter()
                        .filter_map(|value| if !value.1 { Some(value.0) } else { None })
                        .sum::<usize>()
                })
                .sum();
            self.won = true;
            Some(unmarked * self.rows[y][x].0)
        } else {
            None
        }
    }

    pub fn won(&self) -> bool { self.won }
}

impl Game {
    pub fn new(draws: Draws, boards: Vec<Board>) -> Game {
        let mut index = IndexMap::with_capacity(draws.len());
        boards.iter().enumerate().for_each(|(board_index, b)| {
            (0..BOARD_SIZE)
                .cartesian_product(0..BOARD_SIZE)
                .for_each(|(y, x)| {
                    index
                        .entry(b.rows[y][x].0)
                        .or_insert(Vec::new())
                        .push((board_index, x, y));
                });
        });
        Game {
            index,
            draws,
            boards,
        }
    }
    pub fn play(&mut self) -> usize {
        for d in &self.draws {
            //eprintln!("Drawing {}", d);
            match self.index.get(&d) {
                None => continue,
                Some(cells) => {
                    for c in cells {
                        //eprintln!("Applying to board {} ({}, {})", c.0, c.1, c.2);
                        if let Some(result) = self.boards[c.0].apply(c.1, c.2) {
                            return result;
                        }
                    }
                }
            }
        }
        panic!("No winner");
    }
    pub fn play2(&mut self) -> usize {
        let mut winner = 0;
        for d in &self.draws {
            //eprintln!("Drawing {}", d);
            match self.index.get(&d) {
                None => continue,
                Some(cells) => {
                    for c in cells {
                        //eprintln!("Applying to board {} ({}, {})", c.0, c.1, c.2);
                        if let Some(result) = self.boards[c.0].apply(c.1, c.2) {
                            winner += 1;
                            if winner == self.boards.len() {
                                return result;
                            }
                        }
                    }
                }
            }
        }
        panic!("No winner");
    }
}

fn read_file_into_game(path: &str) -> Game {
    let input = fs::read_to_string(path).expect("Missing input data");
    let mut it = input.split("\r\n\r\n");
    let draws_string = it.next().expect("Missing draws");
    //eprintln!("{:?}", draws_string);
    let draws = draws_string
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let boards = it
        .map(|board_input| Board::new(board_input.lines()))
        .collect_vec();
    Game::new(draws, boards)
}

#[test]
fn task1_example() {
    let mut game = read_file_into_game("src/day4/example.txt");
    let result = game.play();
    println!("D4T1E {}", result);
    assert_eq!(result, 4512);
}

#[test]
fn task1_puzzle() {
    let mut game = read_file_into_game("src/day4/input.txt");
    let result = game.play();
    println!("D4T1P {}", result);
    assert_eq!(result, 35670);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let mut game = read_file_into_game("src/day4/example.txt");
    let result = game.play2();
    println!("D4T2E {}", result);
    assert_eq!(result, 1924);
}

#[test]
fn task2_puzzle() {
    let mut game = read_file_into_game("src/day4/input.txt");
    let result = game.play2();
    println!("D4T2P {}", result);
    assert_eq!(result, 22704);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
