use crate::utils;

struct Line {
    bits: Vec<u8>
}

fn read_file_into_vector(path: &str) -> Vec<Line> {
    utils::read_file_into_vector(path, |l| {
        Line { bits: l.as_bytes().to_owned() }
    })
}

fn gamma_epsilon(v: &Vec<Line>) -> (usize,usize) {
    let mut bits = vec![0; v[0].bits.len()];
    v.iter().for_each(|l| for x in 0..l.bits.len() {
        bits[x] += if l.bits[x] == b'1' { 1 } else { 0 }
    });
    let mut ret = 0;
    for x in 0..bits.len() {
        ret += (if bits[x] > v.len() / 2 { 1 } else { 0 }) << bits.len() - x - 1;
    }
    (ret, ((1 << bits.len()) - 1) ^ ret)
}

fn answer(v: (usize, usize)) -> usize {
    v.0 * v.1
}

#[test]
fn task1_example() {
    let values = read_file_into_vector("src/day3/example.txt");
    let result = answer(gamma_epsilon(&values));
    println!("D3T1E {}", result);
    assert_eq!(result, 198);
}

#[test]
fn task1_puzzle() {
    let values = read_file_into_vector("src/day3/input.txt");
    let result = answer(gamma_epsilon(&values));
    println!("D3T1P {}", result);
    assert_eq!(result, 2954600);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

// #[test]
// fn task2_example() {
//     let values = read_file_into_vector("src/day2/example.txt");
//     let result = answer(move_sum2(&values));
//     println!("D2T2E {}", result);
//     assert_eq!(result, 900);
// }
//
// #[test]
// fn task2_puzzle() {
//     let values = read_file_into_vector("src/day2/input.txt");
//     let result = answer(move_sum2(&values));
//     println!("D2T2P {}", result);
//     assert_eq!(result, 1739283308);
// }
//
// #[bench]
// fn task2_puzzle_bench(b: &mut test::Bencher) {
//     b.iter(|| {
//         task2_puzzle();
//     });
// }
