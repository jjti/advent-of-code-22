pub fn part_1() -> String {
    format!("{}", solve(include_str!("input/day13.txt")))
}

/// --- Part Two ---
// Now, you just need to put all of the packets in the right order. Disregard the blank lines in your list of received packets.

// The distress signal protocol also requires that you include two additional divider packets:

// [[2]]
// [[6]]
// Using the same rules as before, organize all packets - the ones in your list of received packets as well as the two divider packets - into the correct order.

// For the example above, the result of putting the packets in the correct order is:

// []
// [[]]
// [[[]]]
// [1,1,3,1,1]
// [1,1,5,1,1]
// [[1],[2,3,4]]
// [1,[2,[3,[4,[5,6,0]]]],8,9]
// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [[1],4]
// [[2]]
// [3]
// [[4,4],4,4]
// [[4,4],4,4,4]
// [[6]]
// [7,7,7]
// [7,7,7,7]
// [[8,7,6]]
// [9]
// Afterward, locate the divider packets. To find the decoder key for this distress signal, you need to determine the indices of the two divider packets and multiply them together. (The first packet is at index 1, the second packet is at index 2, and so on.) In this example, the divider packets are 10th and 14th, and so the decoder key is 140.
// Organize all of the packets into the correct order. What is the decoder key for the distress signal?
pub fn part_2() -> String {
    "".to_string()
}

fn solve(input: &str) -> usize {
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

fn sorted_str(l: &str, r: &str) -> bool {
    let left = parse(l);
    let right = parse(r);

    sorted(left, right) < 0
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

fn sorted(mut left: Vec<Val>, mut right: Vec<Val>) -> i8 {
    if left.is_empty() {
        if right.is_empty() {
            return 0;
        }
        return -1;
    }

    while !left.is_empty() {
        if right.is_empty() {
            // If the right list runs out of items first, the inputs are not in the right order.
            return 1;
        }

        let lval = left.remove(0);
        let rval = right.remove(0);

        match (lval, rval) {
            (Val::Int(l), Val::Int(r)) => {
                if l > r {
                    // If the left integer is higher than the right integer, the inputs are not in the right order.
                    return 1;
                } else if l < r {
                    // If the left integer is lower than the right integer, the inputs are in the right order.
                    return -1;
                }
                // Otherwise, the inputs are the same integer; continue checking the next part of the input.
                continue;
            }
            (Val::List(l), Val::List(r)) => {
                // If both values are lists, compare the first value of each list, then the second value, and so on
                match sorted(l, r) {
                    v if v < 0 => return -1,
                    v if v > 0 => return 1,
                    _ => (), // continue
                }
            }
            (Val::Int(l), Val::List(r)) => {
                // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
                match sorted(vec![Val::Int(l)], r) {
                    v if v < 0 => return -1,
                    v if v > 0 => return 1,
                    _ => (), // continue
                }
            }
            (Val::List(l), Val::Int(r)) => {
                // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
                match sorted(l, vec![Val::Int(r)]) {
                    v if v < 0 => return -1,
                    v if v > 0 => return 1,
                    _ => (), // continue
                }
            }
        }
    }

    // println!("left: {:?}, right: {:?}", left, right);
    if right.is_empty() {
        return 0;
    }
    -1
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
            solve(
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
}
