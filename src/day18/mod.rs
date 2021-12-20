use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Clone)]
struct List {
    l: Node,
    r: Node,
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:?},{:?}]", self.l, self.r))
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Node {
    Regular(u8),
    List(Box<List>),
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Regular(v) => f.write_fmt(format_args!("{}", v)),
            Node::List(l) => f.write_fmt(format_args!("{:?}", l.as_ref())),
        }
    }
}

fn node_magnitude(n: &Node) -> usize {
    match n {
        Node::Regular(v) => *v as usize,
        Node::List(l) => magnitude(l),
    }
}

fn magnitude(n: &List) -> usize {
    3 * node_magnitude(&n.l) + 2 * node_magnitude(&n.r)
}

fn dfs_traverse_node_in_order<F>(n: &mut Node, level: u8, f: &mut F) -> ()
where
    F: FnMut(&mut Node, u8) -> (),
{
    match n {
        Node::List(l) => {
            if level >= 4 {
                f(n, level);
            } else {
                dfs_traverse_list_in_order(l, level, f);
            }
        }
        Node::Regular(_) => f(n, level),
    }
}

fn dfs_traverse_list_in_order<F>(n: &mut List, level: u8, f: &mut F) -> ()
where
    F: FnMut(&mut Node, u8) -> (),
{
    dfs_traverse_node_in_order(&mut n.l, level + 1, f);
    dfs_traverse_node_in_order(&mut n.r, level + 1, f);
}

fn add_regular_checked(n: *mut Node, value: u8) -> () {
    if n.is_null() {
        return;
    }
    unsafe {
        if let Node::Regular(v) = n.as_ref().unwrap() {
            *n = Node::Regular(*v + value);
        } else {
            panic!("Invalid node type");
        }
    }
}

fn explosion_traversal(n: &mut List) -> bool {
    let mut l: *mut Node = std::ptr::null_mut();
    let mut r: *mut Node = std::ptr::null_mut();
    let mut exp: *mut Node = std::ptr::null_mut();
    dfs_traverse_list_in_order(n, 0, &mut |n, level| {
        //eprintln!("{:?}", n);
        match n {
            Node::Regular(_) => {
                if exp.is_null() {
                    l = n;
                } else if r.is_null() {
                    r = n
                } else {
                    //right already assigned
                }
            }
            Node::List(l) => {
                if level != 4 {
                    return;
                }
                if exp.is_null() {
                    exp = n;
                } else if r.is_null() {
                    r = &mut l.l;
                }
            }
        }
    });
    if !exp.is_null() {
        unsafe {
            let expr = exp.as_ref().unwrap();
            let (lv, rv) = match expr {
                Node::List(list) => (&list.l, &list.r),
                _ => unreachable!(),
            };
            let (lv, rv) = match (lv, rv) {
                (Node::Regular(l), Node::Regular(r)) => (*l, *r),
                _ => unreachable!(),
            };
            add_regular_checked(l, lv);
            add_regular_checked(r, rv);
            *exp = Node::Regular(0)
        }

        true
    } else {
        false
    }
}

fn split_traversal(n: &mut List) -> bool {
    let mut done = false;
    dfs_traverse_list_in_order(n, 0, &mut |n, _| {
        //eprintln!("{:?}", n);
        if done {
            return;
        }
        if let Node::Regular(v) = n {
            let v = *v;
            if v <= 9 {
                return;
            }
            let l = (v as f64 / 2.0).floor() as u8;
            let r = (v as f64 / 2.0).ceil() as u8;
            let x = Node::List(Box::new(List {
                l: Node::Regular(l),
                r: Node::Regular(r),
            }));
            let _ = std::mem::replace(n, x);
            done = true;
        }
    });
    done
}

fn reduce(n: &mut List) -> () {
    //eprintln!("Reducing {:?}", n);
    let done = explosion_traversal(n);
    if done {
        //eprintln!("After explosion {:?}", n);
        return reduce(n);
    }
    let done = split_traversal(n);
    if done {
        //eprintln!("After split {:?}", n);
        return reduce(n);
    }
}

fn add(l: List, r: List) -> List {
    let mut ret = List {
        l: Node::List(Box::new(l)),
        r: Node::List(Box::new(r)),
    };
    reduce(&mut ret);
    ret
}

fn parse_node_from_iterator<I>(it: &mut I) -> Node
where
    I: Iterator<Item = u8>,
{
    match it.next().unwrap() {
        b'[' => {
            let l = parse_node_from_iterator(it);
            let c = it.next().unwrap();
            assert_eq!(b',', c);
            let r = parse_node_from_iterator(it);
            let c = it.next().unwrap();
            assert_eq!(b']', c);
            Node::List(Box::new(List { l, r }))
        }
        b => Node::Regular(b - b'0'),
    }
}

fn parse_str(s: &str) -> List {
    let mut it = s.bytes();
    let n = parse_node_from_iterator(&mut it);
    if let Node::List(l) = n {
        List { l: l.l, r: l.r }
    } else {
        panic!("Root node not-list");
    }
}

#[test]
fn explode_test() {
    let mut n = parse_str("[[[[[9,8],1],2],3],4]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[[[0,9],2],3],4]"));

    let mut n = parse_str("[7,[6,[5,[4,[3,2]]]]]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[7,[6,[5,[7,0]]]]"));

    let mut n = parse_str("[[6,[5,[4,[3,2]]]],1]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[6,[5,[7,0]]],3]"));

    let mut n = parse_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

    let mut n = parse_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}

#[test]
fn reduce_test() {
    let mut n = parse_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));

    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    let sp = split_traversal(&mut n);
    assert_eq!(sp, true);

    let sp = split_traversal(&mut n);
    assert_eq!(sp, true);
    assert_eq!(n, parse_str("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));

    let ex = explosion_traversal(&mut n);
    assert_eq!(ex, true);
    assert_eq!(n, parse_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

    let mut n = parse_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    reduce(&mut n);
    assert_eq!(n, parse_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn add_test() {
    let l = parse_str("[1,1]");
    let l = add(l, parse_str("[2,2]"));
    let l = add(l, parse_str("[3,3]"));
    let l = add(l, parse_str("[4,4]"));
    assert_eq!(l, parse_str("[[[[1,1],[2,2]],[3,3]],[4,4]]"));

    let l = parse_str("[1,1]");
    let l = add(l, parse_str("[2,2]"));
    let l = add(l, parse_str("[3,3]"));
    let l = add(l, parse_str("[4,4]"));
    let l = add(l, parse_str("[5,5]"));
    assert_eq!(l, parse_str("[[[[3,0],[5,3]],[4,4]],[5,5]]"));

    let l = parse_str("[1,1]");
    let l = add(l, parse_str("[2,2]"));
    let l = add(l, parse_str("[3,3]"));
    let l = add(l, parse_str("[4,4]"));
    let l = add(l, parse_str("[5,5]"));
    let l = add(l, parse_str("[6,6]"));
    assert_eq!(l, parse_str("[[[[5,0],[7,4]],[5,5]],[6,6]]"));

    let l = parse_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let l = add(l, parse_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"));
    let l = add(l, parse_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"));
    let l = add(
        l,
        parse_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
    );
    let l = add(l, parse_str("[7,[5,[[3,8],[1,4]]]]"));
    let l = add(l, parse_str("[[2,[2,2]],[8,[8,1]]]"));
    let l = add(l, parse_str("[2,9]"));
    let l = add(l, parse_str("[1,[[[9,3],9],[[9,0],[0,7]]]]"));
    let l = add(l, parse_str("[[[5,[7,4]],7],1]"));
    let l = add(l, parse_str("[[[[4,2],2],6],[8,7]]"));
    assert_eq!(
        l,
        parse_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn magnitude_test() {
    assert_eq!(magnitude(&parse_str("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(
        magnitude(&parse_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
        1384
    );
    assert_eq!(magnitude(&parse_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
    assert_eq!(magnitude(&parse_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
    assert_eq!(magnitude(&parse_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
    assert_eq!(
        magnitude(&parse_str(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )),
        3488
    );
}

#[test]
fn task1_example() {
    let v = super::utils::read_file_into_vector("src/day18/example.txt", parse_str);
    let mut it = v.into_iter();
    let init = it.next().unwrap();
    let result = it.fold(init, add);
    assert_eq!(
        result,
        parse_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
    );
    assert_eq!(magnitude(&result), 4140);
}

#[test]
fn task1_puzzle() {
    let v = super::utils::read_file_into_vector("src/day18/input.txt", parse_str);
    let mut it = v.into_iter();
    let init = it.next().unwrap();
    let result = it.fold(init, add);
    assert_eq!(magnitude(&result), 3665);
}
