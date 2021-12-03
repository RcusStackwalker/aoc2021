use crate::utils;

#[derive(Debug,Clone)]
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

fn most_common_bit(v: &Vec<Line>, pos: usize) -> u8 {
    let mut count = 0;
    v.iter().for_each(|l| {
        if l.bits[pos] == b'1' {
            count += 1;
        }
    });
    if count*2 >= v.len() { b'1' } else { b'0' }
}

fn least_common_bit(v: &Vec<Line>, pos: usize) -> u8 {
    let mut count = 0;
    v.iter().for_each(|l| {
        if l.bits[pos] == b'1' {
            count += 1;
        }
    });
    if count*2 < v.len() { b'1' } else { b'0' }
}

fn filter_by_position(v: &Vec<Line>, pos: usize, byte: u8) -> Vec<Line> {
    v.iter().filter(|l| {
        l.bits[pos] == byte
    }).map(|x| x.clone()).collect()
}

fn bits_to_decimal(v: &Line) -> usize {
    let mut ret = 0;
    for x in 0..v.bits.len() {
        ret += (if v.bits[x] == b'1' { 1 } else { 0 }) << (v.bits.len() - 1 - x);
    }
    ret
}

fn oxygen_co2(v: &Vec<Line>) -> (usize, usize) {
    let mut values = v.clone();
    eprintln!("{:?}", values);
    for pos in 0..v[0].bits.len() {
        let mcb = most_common_bit(&values, pos);
        eprintln!("mcb at pos {} {}", pos, mcb);
        values = filter_by_position(&values, pos, mcb);
        eprintln!("{:?}", values);
        if values.len() == 1 {
            break;
        }
    }
    let oxygen = bits_to_decimal(&values[0]);
    values = v.clone();
    for pos in 0..v[0].bits.len() {
        values = filter_by_position(&values, pos, least_common_bit(&values, pos));
        if values.len() == 1 {
            break;
        }
    }
    let co2 = bits_to_decimal(&values[0]);
    (oxygen, co2)
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

#[test]
fn task2_example() {
    let values = read_file_into_vector("src/day3/example.txt");
    let result = answer(oxygen_co2(&values));
    println!("D3T2E {}", result);
    assert_eq!(result, 230);
}

#[test]
fn task2_puzzle() {
    let values = read_file_into_vector("src/day3/input.txt");
    let result = answer(oxygen_co2(&values));
    println!("D3T2P {}", result);
    assert_eq!(result, 1662846);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}

