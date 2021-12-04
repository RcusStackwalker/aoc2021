use crate::utils;

#[derive(Debug, Copy, Clone)]
struct Line {
    bits: usize,
    len: usize,
}

impl Line {
    pub fn new(str: &str) -> Line {
        Line {
            bits: usize::from_str_radix(str, 2).expect("Malformed string"),
            len: str.len(),
        }
    }
    pub fn bool_at_pos(&self, pos: usize) -> bool {
        (self.bits & (1 << (self.len - 1 - pos))) != 0
    }

    pub fn value_at_pos(&self, pos: usize) -> usize {
        if self.bool_at_pos(pos) {
            1
        } else {
            0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn read_file_into_vector(path: &str) -> Vec<Line> {
    utils::read_file_into_vector(path, Line::new)
}

fn gamma_epsilon(v: &Vec<Line>) -> (usize, usize) {
    let mut ret = 0;
    let l = v[0].len();
    for x in 0..l {
        ret += (if most_common_bit(v, x) { 1 } else { 0 }) << l - x - 1;
    }
    (ret, ((1 << l) - 1) ^ ret)
}

fn most_common_bit(v: &Vec<Line>, pos: usize) -> bool {
    let mut count = 0;
    v.iter().for_each(|l| {
        if l.bool_at_pos(pos) {
            count += 1;
        }
    });
    count * 2 >= v.len()
}

fn least_common_bit(v: &Vec<Line>, pos: usize) -> bool {
    !most_common_bit(v, pos)
}

fn filter_by_position(v: Vec<Line>, pos: usize, byte: bool) -> Vec<Line> {
    v.into_iter()
        .filter(|l| l.bool_at_pos(pos) == byte)
        .collect()
}

fn bits_to_decimal(v: &Line) -> usize {
    let mut ret = 0;
    for x in 0..v.len() {
        ret += v.value_at_pos(x) << (v.len() - 1 - x);
    }
    ret
}

fn filter_by_criterion<F>(v: Vec<Line>, fun: F) -> usize
where
    F: Fn(&Vec<Line>, usize) -> bool,
{
    let linelen = v[0].len();
    let mut values = v;
    for pos in 0..linelen {
        let byte = fun(&values, pos);
        values = filter_by_position(values, pos, byte);
        if values.len() == 1 {
            break;
        }
    }
    bits_to_decimal(&values[0])
}

fn oxygen_co2(v: &Vec<Line>) -> (usize, usize) {
    let oxygen = filter_by_criterion(v.clone(), most_common_bit);
    let co2 = filter_by_criterion(v.clone(), least_common_bit);
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
