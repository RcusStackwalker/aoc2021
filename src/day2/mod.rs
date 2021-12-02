use crate::utils;

enum Move {
    Forward(isize),
    Up(isize),
    Down(isize),
}

fn read_file_into_vector(path: &str) -> Vec<Move> {
    utils::read_file_into_vector(path, |l| {
        let (direction, x) = l.split_once(' ').expect("malformed line");
        let x = x.parse::<isize>().expect("x not a isize");
        match direction {
            "forward" => Move::Forward(x),
            "up" => Move::Up(x),
            "down" => Move::Down(x),
            _ => panic!("unexpected direction"),
        }
    })
}

fn move_sum(v: &Vec<Move>) -> (isize, isize) {
    let mut ret = (0, 0);
    v.iter().for_each(|m| match m {
        Move::Forward(x) => ret.0 += x,
        Move::Up(x) => ret.1 -= x,
        Move::Down(x) => ret.1 += x,
    });
    ret
}

fn move_sum2(v: &Vec<Move>) -> (isize, isize) {
    let mut ret = (0, 0);
    let mut aim = 0;
    v.iter().for_each(|m| match m {
        Move::Forward(x) => {
            ret.0 += x;
            ret.1 += x * aim;
        }
        Move::Up(x) => aim -= x,
        Move::Down(x) => aim += x,
    });
    ret
}

fn answer(v: (isize, isize)) -> isize {
    v.0 * v.1
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day2/example.txt");
    let result = answer(move_sum(&values));
    println!("D2T1E {}", result);
    assert_eq!(result, 150);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day2/input.txt");
    let result = answer(move_sum(&values));
    println!("D2T1P {}", result);
    assert_eq!(result, 1815044);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day2/example.txt");
    let result = answer(move_sum2(&values));
    println!("D2T2E {}", result);
    assert_eq!(result, 900);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day2/input.txt");
    let result = answer(move_sum2(&values));
    println!("D2T2P {}", result);
    assert_eq!(result, 1739283308);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
