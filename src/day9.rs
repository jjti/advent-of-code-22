use std::collections::HashSet;

pub fn part_1() -> String {
    format!("{}", solve(include_str!("input/day9.txt"), 2))
}

pub fn part_2() -> String {
    format!("{}", solve(include_str!("input/day9.txt"), 10))
}

fn solve(input: &str, knots: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); knots];
    for line in input.lines() {
        let cols: Vec<&str> = line.split_whitespace().into_iter().collect();
        let dir = cols[0];
        let dist = cols[1].parse::<i32>().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => panic!(),
            }
            for i in 1..knots.len() {
                knots[i] = move_tail(knots[i - 1], knots[i]);
            }
            visited.insert(knots[knots.len() - 1]);

            // println!("dir: {}, head: {:?}, tail: {:?}", dir, head, tail);
        }
    }
    visited.len()
}

/// move_tail moves the tail towards the head, returning its new position
fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // If they're touching, do nothing
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
        return tail;
    }

    // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
    if head.0.abs_diff(tail.0) == 2 && head.1 == tail.1 {
        return (tail.0 + (head.0 - tail.0).signum(), tail.1);
    }
    if head.1.abs_diff(tail.1) == 2 && head.0 == tail.0 {
        return (tail.0, tail.1 + (head.1 - tail.1).signum());
    }

    // Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:
    (
        tail.0 + (head.0 - tail.0).signum(),
        tail.1 + (head.1 - tail.1).signum(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            solve(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
                2
            ),
            13
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            solve(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
                10
            ),
            36
        );
    }
}
