use std::fs::read_to_string;

pub fn part_1() -> String {
    let sum = read_to_string("./src/day4/input.txt")
        .expect("file not found")
        .lines()
        .filter(|l| {
            let cols: Vec<&str> = l.split(',').collect();
            contains(range(cols[0]), range(cols[1]))
        })
        .count();

    format!("{}", sum)
}

fn range(col: &str) -> (i32, i32) {
    let cols: Vec<&str> = col.split('-').collect();

    (cols[0].parse().unwrap(), cols[1].parse().unwrap())
}

/// returns whether r1 is fully contained in r2 or visa versa.
fn contains(r1: (i32, i32), r2: (i32, i32)) -> bool {
    (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
}

// part_2 is like part_1 except it counts the lines where there's any overlap between the ranges at all.
pub fn part_2() -> String {
    let sum = read_to_string("./src/day4/input.txt")
        .unwrap()
        .split('\n')
        .filter(|l| {
            let cols: Vec<&str> = l.split(',').collect();
            let r1 = range(cols[0]);
            let r2 = range(cols[1]);

            (r1.0 <= r2.0 && r1.1 >= r2.0) || (r2.0 <= r1.0 && r2.1 >= r1.0)
        })
        .count();

    format!("{}", sum)
}

#[cfg(test)]
mod test {
    use super::*;

    /// a test of the range function
    #[test]
    fn test_range() {
        assert_eq!((1, 2), range("1-2"));
    }

    /// a test of the overlap function
    #[test]
    fn test_overlap() {
        assert_eq!(true, contains((1, 2), (1, 2)));
        assert_eq!(true, contains((1, 2), (0, 3)));
        assert_eq!(true, contains((0, 3), (1, 2)));
        assert_eq!(false, contains((0, 1), (2, 3)));
        assert_eq!(false, contains((2, 3), (0, 1)));
    }
}
