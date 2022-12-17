use std::{collections::HashSet, fs::read_to_string};

pub fn part_1() -> String {
    // A = 65; a = 97
    let sum: u32 = read_to_string("./src/input/day3.txt")
        .unwrap()
        .split('\n')
        .map(score)
        .sum();

    format!("{}", sum)
}

fn score(line: &str) -> u32 {
    let cols = line.split_at(line.len() / 2);
    let seen: HashSet<&u8> = cols.0.as_bytes().iter().collect();
    for c in cols.1.as_bytes() {
        if seen.contains(c) {
            return score_char(c);
        }
    }
    panic!()
}

fn score_char(c: &u8) -> u32 {
    match *c {
        65..=91 => (*c - 65 + 27) as u32,
        97..=123 => (*c - 97 + 1) as u32,
        _ => panic!(),
    }
}

pub fn part_2() -> String {
    // A = 65; a = 97
    let mut sum = 0;
    let set: &mut HashSet<&u8> = &mut HashSet::new();
    for (i, line) in read_to_string("./src/input/day3.txt")
        .unwrap()
        .split('\n')
        .enumerate()
    {
        if i % 3 == 0 {
            line.as_bytes().iter().for_each(|c| {
                set.insert(c);
            });
        } else {
            let other: HashSet<&u8> = line.as_bytes().iter().collect();
            set.retain(|&k| other.contains(k));
        }
        if i % 3 == 2 {
            assert_eq!(1, set.len());
            sum += score_char(set.iter().next().unwrap());
            set.clear();
        }
    }

    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(16, score("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(42, score("PmmdzqPrVvPwwTWBwg"));
    }
}
