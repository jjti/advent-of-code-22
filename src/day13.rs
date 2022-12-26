use std::cmp::Ordering;

pub fn part_1() -> String {
    format!("{}", solve_part_1(include_str!("input/day13.txt")))
}

pub fn part_2() -> String {
    format!("{}", solve_part_2(include_str!("input/day13.txt")))
}

fn solve_part_1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    (0..lines.len())
        .step_by(3)
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, index)| {
            if sorted_str(lines[index], lines[index + 1]) {
                assert!(
                    !sorted_str(lines[index + 1], lines[index]),
                    "{:?} {:?}",
                    lines[index + 1],
                    lines[index]
                );
                acc + (i + 1)
            } else {
                assert!(
                    sorted_str(lines[index + 1], lines[index]),
                    "{:?} {:?}",
                    lines[index + 1],
                    lines[index]
                );
                acc
            }
        })
}

fn solve_part_2(input: &str) -> i32 {
    let mut vals: Vec<Vec<Val>> = input
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(parse)
        .collect();
    vals.push(vec![Val::List(vec![Val::Int(6)])]);
    vals.push(vec![Val::List(vec![Val::Int(2)])]);

    vals.sort_by(|a, b| sorted(a, b));

    let mut i = 0;
    let mut j = 0;
    for (k, v) in vals.into_iter().enumerate() {
        if v.eq(&vec![Val::List(vec![Val::Int(2)])]) {
            i = k + 1;
        }
        if v.eq(&vec![Val::List(vec![Val::Int(6)])]) {
            j = k + 1;
        }
    }

    (i * j) as i32
}

fn sorted_str(l: &str, r: &str) -> bool {
    let left = &mut parse(l);
    let right = &mut parse(r);

    sorted(left, right) == Ordering::Less
}

#[derive(Debug, PartialEq)]
enum Val {
    List(Vec<Val>),
    Int(u8),
}

impl Val {
    fn push(&mut self, val: Val) {
        match self {
            Val::List(l) => l.push(val),
            _ => panic!("Cannot push to non-list"),
        }
    }

    fn new_list() -> Val {
        Val::List(vec![])
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        match self {
            Val::Int(v) => Val::Int(*v),
            Val::List(vals) => Val::List(vals.clone()),
        }
    }
}

fn parse(input_str: &str) -> Vec<Val> {
    let mut stack: Vec<Val> = vec![Val::new_list()];

    let input: Vec<char> = input_str.chars().into_iter().collect();
    let mut skip = 0;
    for i in 1..input.len() - 1 {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        let c = input[i];
        let stack_top = stack.len() - 1;
        match c {
            '[' => {
                let inner_list = Val::new_list();
                stack.push(inner_list);
            }
            ']' => {
                let inner_list = stack.pop().unwrap();
                stack[stack_top - 1].push(inner_list);
            }
            ',' => {
                continue;
            }
            c if c.is_ascii_digit() => {
                // Parse the integer
                let mut int: Vec<char> = vec![];
                int.push(c);
                let mut j = i + 1;
                while input[j].is_ascii_digit() {
                    int.push(input[j]);
                    j += 1;
                    skip += 1;
                }

                stack[stack_top].push(Val::Int(
                    int.into_iter().collect::<String>().parse().unwrap(),
                ));
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    assert_eq!(1, stack.len(), "wat {}", input_str);

    match stack.pop().unwrap() {
        Val::List(l) => l,
        _ => panic!("Expected list"),
    }
}

fn sorted(left_input: &[Val], right_input: &[Val]) -> Ordering {
    let left: &mut Vec<Val> = &mut left_input.to_vec();
    let right: &mut Vec<Val> = &mut right_input.to_vec();

    if left.is_empty() {
        if right.is_empty() {
            return Ordering::Equal;
        }
        return Ordering::Less;
    }

    while !left.is_empty() {
        if right.is_empty() {
            // If the right list runs out of items first, the inputs are not in the right order.
            return Ordering::Greater;
        }

        let lval = left.remove(0);
        let rval = right.remove(0);

        match (lval, rval) {
            (Val::Int(l), Val::Int(r)) => {
                if l > r {
                    // If the left integer is higher than the right integer, the inputs are not in the right order.
                    return Ordering::Greater;
                } else if l < r {
                    // If the left integer is lower than the right integer, the inputs are in the right order.
                    return Ordering::Less;
                }
                // Otherwise, the inputs are the same integer; continue checking the next part of the input.
                continue;
            }
            (Val::List(l), Val::List(r)) => {
                // If both values are lists, compare the first value of each list, then the second value, and so on
                match sorted(&l, &r) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => (), // continue
                }
            }
            (Val::Int(l), Val::List(r)) => {
                // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
                match sorted(&[Val::Int(l)], &r) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => (), // continue
                }
            }
            (Val::List(l), Val::Int(r)) => {
                // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
                match sorted(&l, &[Val::Int(r)]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => (), // continue
                }
            }
        }
    }

    // println!("left: {:?}, right: {:?}", left, right);
    if right.is_empty() {
        return Ordering::Equal;
    }
    Ordering::Less
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        for (input, expected) in vec![
            // ("[[[[],[0],[1,8,10,6]],7,2,[[]]],[6,9],[[[3],[9,7,8],4,[8,1,5],10],2],[1,[[8,10,10,4,1],9,1],8,[[5,1,2],2,0,7,[0,1,7]]]]", vec![List([List([List([]), List([Int(0)]), List([Int(1), Int(8), Int(10), Int(6)])]), Int(7), Int(2), List([List([])])]), List([Int(6), Int(9)]), List([List([List([Int(3)]), List([Int(9), Int(7), Int(8)]), Int(4), List([Int(8), Int(1), Int(5)]), Int(10)]), Int(2)]), List([Int(1), List([List([Int(8), Int(10), Int(10), Int(4), Int(1)]), Int(9), Int(1)]), Int(8), List([List([Int(5), Int(1), Int(2)]), Int(2), Int(0), Int(7), List([Int(0), Int(1), Int(7)])])])]),
            (
                "[1,1,3,1,1]",
                vec![
                    Val::Int(1),
                    Val::Int(1),
                    Val::Int(3),
                    Val::Int(1),
                    Val::Int(1),
                ],
            ),
            ("[10]", vec![Val::Int(10)]),
            (
                "[[1],[2,3,4]]",
                vec![
                    Val::List(vec![Val::Int(1)]),
                    Val::List(vec![Val::Int(2), Val::Int(3), Val::Int(4)]),
                ],
            ),
            ("[[1],4]", vec![Val::List(vec![Val::Int(1)]), Val::Int(4)]),
            (
                "[[8,7,6]]",
                vec![Val::List(vec![Val::Int(8), Val::Int(7), Val::Int(6)])],
            ),
            (
                "[[4,4],4,4]",
                vec![
                    Val::List(vec![Val::Int(4), Val::Int(4)]),
                    Val::Int(4),
                    Val::Int(4),
                ],
            ),
            (
                "[[4,4],4,4,4]",
                vec![
                    Val::List(vec![Val::Int(4), Val::Int(4)]),
                    Val::Int(4),
                    Val::Int(4),
                    Val::Int(4),
                ],
            ),
            ("[[[]]]", vec![Val::List(vec![Val::List(vec![])])]),
        ] {
            assert_eq!(expected, parse(input));
        }
    }

    #[test]
    fn test_sorted() {
        assert!(sorted_str(
            "[[],[[[5,6,3],6,[6,5,3,3]],8,3],[],[4]]",
            "[[[1,6,[1,9,0,9],6]]]"
        ));
        assert!(!sorted_str(
            "[[[1,6,[1,9,0,9],6]]]",
            "[[],[[[5,6,3],6,[6,5,3,3]],8,3],[],[4]]",
        ));
        assert!(!sorted_str("[[10]]", "[[3,[],[7,4,8,[]],1]]"));
        assert!(sorted_str("[1,1,3,1,1]", "[1,1,5,1,1]"));
        assert!(!sorted_str("[1,1,5,1,1]", "[1,1,3,1,1]"));
        assert!(sorted_str("[[1],[2,3,4]]", "[[1],4]"));
        assert!(!sorted_str("[[1],4]", "[[1],[2,3,4]]"));
        assert!(!sorted_str("[9]", "[[8,7,6]]"));
        assert!(sorted_str("[[8,7,6]]", "[9]"));
        assert!(sorted_str("[[4,4],4,4]", "[[4,4],4,4,4]"));
        assert!(!sorted_str("[[4,4],4,4,4]", "[[4,4],4,4]"));
        assert!(!sorted_str("[7,7,7,7]", "[7,7,7]"));
        assert!(sorted_str("[7,7,7]", "[7,7,7,7]"));
        assert!(sorted_str("[]", "[3]"));
        assert!(!sorted_str("[3]", "[]"));
        assert!(!sorted_str("[[[]]]", "[[]]"));
        assert!(sorted_str("[[]]", "[[[]]]"));
        assert!(!sorted_str(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
        assert!(sorted_str(
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]"
        ));
        assert!(!sorted_str(
            "[[[1,6,[1,9,0,9],6]]]",
            "[[],[[[5,6,3],6,[6,5,3,3]],8,3],[],[4]]"
        ));
        assert!(!sorted_str(
            "[[1,9,2]]",
            "[[[[],[0],[1,8,10,6]],7,2,[[]]],[6,9],[[[3],[9,7,8],4,[8,1,5],10],2],[1,[[8,10,10,4,1],9,1],8,[[5,1,2],2,0,7,[0,1,7]]]]",
        ));
        assert!(sorted_str("[4,0,6,0]", "[4,0,6,0,7]"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            13,
            solve_part_1(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            solve_part_2(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            140
        );
    }
}
