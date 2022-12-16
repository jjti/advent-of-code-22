pub fn part_1() -> String {
    solve(|x| x / 3, 20)
}

pub fn part_2() -> String {
    solve(|x| x % 9699690, 10_000)
}

fn solve(worry_adjust: fn(i64) -> i64, rounds: usize) -> String {
    // create monkeys
    let mut monkeys = get_monkeys();

    // rounds of passing
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let (push_left, push_right) = monkeys[i].inspect_and_throw(worry_adjust);
            let left = monkeys[i].left;
            let right = monkeys[i].right;

            // Push items to other monkeys
            for item in push_left {
                monkeys[left].items.push(item);
            }
            for item in push_right {
                monkeys[right].items.push(item);
            }
        }
    }

    // get inspection counts, sort
    let mut inspections: Vec<i64> = monkeys.iter().map(|m| m.inspected).collect();
    inspections.sort();
    inspections.reverse();

    format!("{}", inspections[0] * inspections[1])
}

struct Monkey {
    items: Vec<i64>,
    op: fn(i64) -> i64,
    test: fn(i64) -> bool,
    left: usize,
    right: usize,

    inspected: i64,
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        op: fn(i64) -> i64,
        test: fn(i64) -> bool,
        left: usize,
        right: usize,
    ) -> Monkey {
        Monkey {
            items,
            op,
            test,
            left,
            right,
            inspected: 0,
        }
    }

    fn inspect_and_throw(&mut self, worry_adjust: fn(i64) -> i64) -> (Vec<i64>, Vec<i64>) {
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();

        while !self.items.is_empty() {
            self.inspected += 1;

            let mut item = self.items.pop().unwrap();

            // Operation shows how your worry level changes as that monkey inspects an item.
            item = (self.op)(item);

            // After each monkey inspects an item but before it tests your worry level,
            // your relief that the monkey's inspection didn't damage the item causes
            // your worry level to be divided by three and rounded down to the nearest integer.
            item = worry_adjust(item);

            // Test shows how the monkey uses your worry level to decide where to throw an item next.
            if (self.test)(item) {
                left.push(item);
            } else {
                right.push(item);
            }
        }

        (left, right)
    }
}

fn get_monkeys() -> Vec<Monkey> {
    vec![
        // Monkey 0
        Monkey::new(
            vec![84, 66, 62, 69, 88, 91, 91],
            |x| x * 11,
            |x| x % 2 == 0,
            4,
            7,
        ),
        // Monkey 1
        Monkey::new(vec![98, 50, 76, 99], |x| x * x, |x| x % 7 == 0, 3, 6),
        // Monkey 2
        Monkey::new(vec![72, 56, 94], |x| x + 1, |x| x % 13 == 0, 4, 0),
        // Monkey 3
        Monkey::new(
            vec![55, 88, 90, 77, 60, 67],
            |x| x + 2,
            |x| x % 3 == 0,
            6,
            5,
        ),
        // Monkey 4
        Monkey::new(
            vec![69, 72, 63, 60, 72, 52, 63, 78],
            |x| x * 13,
            |x| x % 19 == 0,
            1,
            7,
        ),
        // Monkey 5
        Monkey::new(vec![89, 73], |x| x + 5, |x| x % 17 == 0, 2, 0),
        // Monkey 6
        Monkey::new(vec![78, 68, 98, 88, 66], |x| x + 6, |x| x % 11 == 0, 2, 5),
        // Monkey 7
        Monkey::new(vec![70], |x| x + 7, |x| x % 5 == 0, 1, 3),
    ]
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_one_round() {
//         let monkeys = get_monkeys();

//     }
// }
