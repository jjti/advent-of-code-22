use std::collections::HashMap;

pub fn part_1() -> String {
    format!("{}", solve(include_str!("input/day12.txt"), false))
}

pub fn part_2() -> String {
    format!("{}", solve(include_str!("input/day12.txt"), true))
}

fn solve(input: &str, init_zero: bool) -> i32 {
    let mut distances: HashMap<(usize, usize), i32> = HashMap::new();
    let mut queue: Vec<(usize, usize)> = vec![];

    // Init grid heights, find start/end coordinates
    let mut heights: Vec<Vec<i8>> = Vec::new();
    let mut end = (0, 0);
    for (row, line) in input.lines().into_iter().enumerate() {
        heights.push(Vec::new());
        for (col, c) in line.chars().into_iter().enumerate() {
            heights[row].push(match c {
                'a'..='z' => {
                    if c == 'a' && init_zero {
                        distances.insert((row, col), 0);
                        queue.push((row, col));
                    }

                    c as i8 - 'a' as i8
                }
                'S' => {
                    distances.insert((row, col), 0);
                    queue.push((row, col));
                    0
                }
                'E' => {
                    end = (row, col);
                    'z' as i8 - 'a' as i8
                }
                _ => panic!("Unknown char: {}", c),
            });
        }
    }
    if end == (0, 0) {
        panic!("No end found");
    }

    // BFS

    if init_zero {}

    while !queue.is_empty() {
        let (row, col) = queue.remove(0);
        let distance = *distances.get_mut(&(row, col)).unwrap();

        // Add neighbors
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        match row {
            0 => neighbors.push((row + 1, col)),
            r if r == heights.len() - 1 => neighbors.push((row - 1, col)),
            r => {
                neighbors.push((r - 1, col));
                neighbors.push((r + 1, col));
            }
        }
        match col {
            0 => neighbors.push((row, col + 1)),
            c if c == heights[0].len() - 1 => neighbors.push((row, col - 1)),
            c => {
                neighbors.push((row, c - 1));
                neighbors.push((row, c + 1));
            }
        }

        for (r, c) in neighbors {
            // Check if we can move there
            let height = heights[r][c];
            if height - heights[row][col] > 1 {
                continue;
            }

            // If this distance is shorter or new, store it in the queue
            let new_distance = distance + 1;
            if let Some(old_distance) = distances.get(&(r, c)) {
                if new_distance >= *old_distance {
                    continue;
                }
            }

            // Add to queue
            distances.insert((r, c), new_distance);
            queue.push((r, c));
        }
    }

    // print!("{:?}", distances);
    *distances.get(&end).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            31,
            solve(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
                false
            )
        );

        assert_eq!(
            29,
            solve(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
                true
            )
        );
    }
}
