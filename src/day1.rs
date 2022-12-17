use std::fs::read_to_string;

pub fn solve() -> String {
    let file = read_to_string("src/input/day1.txt").unwrap();

    let mut vals: Vec<i32> = Vec::new();
    let mut cur = 0;
    for line in file.split('\n') {
        if line.is_empty() {
            vals.push(cur);
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }
    vals.sort();
    vals.reverse();

    format!(
        "{} {}",
        vals.first().unwrap(),
        vals.iter().take(3).sum::<i32>()
    )
}
