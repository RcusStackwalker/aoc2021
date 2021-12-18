use itertools::Itertools;

struct Area {
    x: (isize, isize),
    y: (isize, isize),
}

fn get_max_height(area: Area) -> usize {
    let dy = area.y.0.unsigned_abs();
    dy * (dy - 1) / 2
}

fn get_max_dx(area: &Area) -> isize {
    area.x.1
}

fn get_min_dx(area: &Area) -> isize {
    (area.x.0 as f64).sqrt().ceil() as isize
}

fn get_max_dy(area: &Area) -> isize {
    area.y.0.abs()
}

fn get_min_dy(area: &Area) -> isize {
    area.y.0
}

// for all x,y in the Area, if exists n so
// dx + (dx - 1) + (dx - 2) + ... (dx - n - 1) in [x0,x1]
// simple case - dx = n - 1
// (n - 1) + (n - 2) + ... 0 = (n - 1) * (n - 2) / 2 = 0.5*n^2 - 1.5n + 1
// a = 0.5, b = -1.5, c = (1-A)
// D = 1.5^2 + 2*A - 2
// n = (1.5 + (1.5^2 + 2*A - 2)) / 2
// dy + (dy - 1) + .. + (dy - n - 1) = n * dy - (n - 1) * (n - 2) / 2
// n * (dy + dy - n + 1) / 2 = A
// n * 2 * dy - n * (n - 1) = 2*A
// n ^ 2 - (2 * dy + 1) * n + 2 * A - 1 = 0
// D = (2*dy + 1)^2 - 4*(2*A-1)
// n = (-2*dy - 1 + sqrt((2*dy + 1) ^ 2 - 4*(2*A-1)))/ 2
fn number_of_y_steps_to_overshoot(y: isize, dy: isize) -> isize {
    let y = y.abs() as f64;
    let dy = dy.abs() as f64;
    ((((8.0 * y + (1.0 - 2.0 * dy).powf(2.0)) as f64).sqrt() - 2.0 * dy + 1.0) / 2.0).ceil()
        as isize
}
fn fits(area: &Area, dx: isize, dy: isize) -> bool {
    let nmax = dx
        .abs()
        .max(dy.abs() * 2 + 1 + number_of_y_steps_to_overshoot(area.y.1, dy));
    for n in 1..=nmax {
        let x = if dx < n - 1 {
            dx * (dx + 1) / 2
        } else {
            n * (2 * dx - n + 1) / 2
        };
        let y = n * (2 * dy - n + 1) / 2;
        if x >= area.x.0 && x <= area.x.1 && y >= area.y.0 && y <= area.y.1 {
            //eprintln!("{},{} fits", dx, dy);
            return true;
        }
    }
    //eprintln!("{},{} doesn't fit", dx, dy);
    false
}

fn get_solution_count(area: Area) -> usize {
    (get_min_dx(&area)..=get_max_dx(&area))
        .cartesian_product(get_min_dy(&area)..=get_max_dy(&area))
        .filter_map(|(dx, dy)| if fits(&area, dx, dy) { Some(()) } else { None })
        .count()
}

#[test]
fn task1_example() {
    let result = get_max_height(Area {
        x: (20, 30),
        y: (-10, -5),
    });
    println!("D17T1E {}", result);
    assert_eq!(result, 45);
}

#[test]
fn task1_puzzle() {
    let result = get_max_height(Area {
        x: (277, 318),
        y: (-92, -53),
    });
    println!("D17T1P {}", result);
    assert_eq!(result, 4186);
}

#[bench]
fn task1_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task1_puzzle();
    });
}

#[test]
fn task2_example() {
    let area = Area {
        x: (20, 30),
        y: (-10, -5),
    };
    assert_eq!(fits(&area, 23, -10), true);
    assert_eq!(fits(&area, 25, -9), true);
    assert_eq!(fits(&area, 27, -5), true);
    assert_eq!(fits(&area, 29, -6), true);
    assert_eq!(fits(&area, 22, -6), true);
    assert_eq!(fits(&area, 21, -7), true);
    assert_eq!(fits(&area, 9, 0), true);
    assert_eq!(fits(&area, 27, -7), true);
    assert_eq!(fits(&area, 24, -5), true);
    assert_eq!(fits(&area, 25, -7), true);
    assert_eq!(fits(&area, 26, -6), true);
    assert_eq!(fits(&area, 25, -5), true);
    assert_eq!(fits(&area, 6, 8), true);
    assert_eq!(fits(&area, 11, -2), true);
    assert_eq!(fits(&area, 20, -5), true);
    assert_eq!(fits(&area, 29, -10), true);
    assert_eq!(fits(&area, 6, 3), true);
    assert_eq!(fits(&area, 28, -7), true);
    assert_eq!(fits(&area, 8, 0), true);
    assert_eq!(fits(&area, 30, -6), true);
    assert_eq!(fits(&area, 29, -8), true);
    assert_eq!(fits(&area, 20, -10), true);
    assert_eq!(fits(&area, 6, 7), true);
    assert_eq!(fits(&area, 6, 4), true);
    assert_eq!(fits(&area, 6, 1), true);
    assert_eq!(fits(&area, 14, -4), true);
    assert_eq!(fits(&area, 21, -6), true);
    assert_eq!(fits(&area, 26, -10), true);
    assert_eq!(fits(&area, 7, -1), true);
    assert_eq!(fits(&area, 7, 7), true);
    assert_eq!(fits(&area, 8, -1), true);
    assert_eq!(fits(&area, 21, -9), true);
    assert_eq!(fits(&area, 6, 2), true);
    assert_eq!(fits(&area, 20, -7), true);
    assert_eq!(fits(&area, 30, -10), true);
    assert_eq!(fits(&area, 14, -3), true);
    assert_eq!(fits(&area, 20, -8), true);
    assert_eq!(fits(&area, 13, -2), true);
    assert_eq!(fits(&area, 7, 3), true);
    assert_eq!(fits(&area, 28, -8), true);
    assert_eq!(fits(&area, 29, -9), true);
    assert_eq!(fits(&area, 15, -3), true);
    assert_eq!(fits(&area, 22, -5), true);
    assert_eq!(fits(&area, 26, -8), true);
    assert_eq!(fits(&area, 25, -8), true);
    assert_eq!(fits(&area, 25, -6), true);
    assert_eq!(fits(&area, 15, -4), true);
    assert_eq!(fits(&area, 9, -2), true);
    assert_eq!(fits(&area, 15, -2), true);
    assert_eq!(fits(&area, 12, -2), true);
    assert_eq!(fits(&area, 28, -9), true);
    assert_eq!(fits(&area, 12, -3), true);
    assert_eq!(fits(&area, 24, -6), true);
    assert_eq!(fits(&area, 23, -7), true);
    assert_eq!(fits(&area, 25, -10), true);
    assert_eq!(fits(&area, 7, 8), true);
    assert_eq!(fits(&area, 11, -3), true);
    assert_eq!(fits(&area, 26, -7), true);
    assert_eq!(fits(&area, 7, 1), true);
    assert_eq!(fits(&area, 23, -9), true);
    assert_eq!(fits(&area, 6, 0), true);
    assert_eq!(fits(&area, 22, -10), true);
    assert_eq!(fits(&area, 27, -6), true);
    assert_eq!(fits(&area, 8, 1), true);
    assert_eq!(fits(&area, 22, -8), true);
    assert_eq!(fits(&area, 13, -4), true);
    assert_eq!(fits(&area, 7, 6), true);
    assert_eq!(fits(&area, 28, -6), true);
    assert_eq!(fits(&area, 11, -4), true);
    assert_eq!(fits(&area, 12, -4), true);
    assert_eq!(fits(&area, 26, -9), true);
    assert_eq!(fits(&area, 7, 4), true);
    assert_eq!(fits(&area, 24, -10), true);
    assert_eq!(fits(&area, 23, -8), true);
    assert_eq!(fits(&area, 30, -8), true);
    assert_eq!(fits(&area, 7, 0), true);
    assert_eq!(fits(&area, 9, -1), true);
    assert_eq!(fits(&area, 10, -1), true);
    assert_eq!(fits(&area, 26, -5), true);
    assert_eq!(fits(&area, 22, -9), true);
    assert_eq!(fits(&area, 6, 5), true);
    assert_eq!(fits(&area, 7, 5), true);
    assert_eq!(fits(&area, 23, -6), true);
    assert_eq!(fits(&area, 28, -10), true);
    assert_eq!(fits(&area, 10, -2), true);
    assert_eq!(fits(&area, 11, -1), true);
    assert_eq!(fits(&area, 20, -9), true);
    assert_eq!(fits(&area, 14, -2), true);
    assert_eq!(fits(&area, 29, -7), true);
    assert_eq!(fits(&area, 13, -3), true);
    assert_eq!(fits(&area, 23, -5), true);
    assert_eq!(fits(&area, 24, -8), true);
    assert_eq!(fits(&area, 27, -9), true);
    assert_eq!(fits(&area, 30, -7), true);
    assert_eq!(fits(&area, 28, -5), true);
    assert_eq!(fits(&area, 21, -10), true);
    assert_eq!(fits(&area, 7, 9), true);
    assert_eq!(fits(&area, 6, 6), true);
    assert_eq!(fits(&area, 21, -5), true);
    assert_eq!(fits(&area, 27, -10), true);
    assert_eq!(fits(&area, 7, 2), true);
    assert_eq!(fits(&area, 30, -9), true);
    assert_eq!(fits(&area, 21, -8), true);
    assert_eq!(fits(&area, 22, -7), true);
    assert_eq!(fits(&area, 24, -9), true);
    assert_eq!(fits(&area, 20, -6), true);
    assert_eq!(fits(&area, 6, 9), true);
    assert_eq!(fits(&area, 29, -5), true);
    assert_eq!(fits(&area, 8, -2), true);
    assert_eq!(fits(&area, 27, -8), true);
    assert_eq!(fits(&area, 30, -5), true);
    assert_eq!(fits(&area, 24, -7), true);
    let result = get_solution_count(area);
    println!("D17T2E {}", result);
    assert_eq!(result, 112);
}

#[test]
fn task2_puzzle() {
    let result = get_solution_count(Area {
        x: (277, 318),
        y: (-92, -53),
    });
    println!("D1721P {}", result);
    assert_eq!(result, 4186);
}

#[bench]
fn task2_puzzle_bench(b: &mut test::Bencher) {
    b.iter(|| {
        task2_puzzle();
    });
}
