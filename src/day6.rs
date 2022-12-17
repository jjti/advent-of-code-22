use std::{collections::HashSet, fs::read_to_string};

pub fn part_1() -> String {
    let mut chars: Vec<u8> = Vec::new();
    let mut ans = 0;
    for (index, char) in read_to_string("src/input/day6.txt")
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
    {
        if index > 3 {
            chars.remove(0);
        }
        chars.push(*char);
        if chars.iter().collect::<HashSet<&u8>>().len() == 4 {
            ans = index + 1;
            break;
        }
    }

    format!("{}", ans)
}

pub fn part_2() -> String {
    let mut chars: Vec<u8> = Vec::new();
    let mut ans = 0;
    for (index, char) in read_to_string("src/input/day6.txt")
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
    {
        if index > 13 {
            chars.remove(0);
        }
        chars.push(*char);
        if chars.iter().collect::<HashSet<&u8>>().len() == 14 {
            ans = index + 1;
            break;
        }
    }

    format!("{}", ans)
}
