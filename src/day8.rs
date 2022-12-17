pub fn part_1() -> String {
    format!("{}", count_visible(include_str!("input/day8.txt")))
}

fn get_heights(input: &str) -> Vec<Vec<i8>> {
    // init all the heights
    let mut all_heights: Vec<Vec<i8>> = Vec::new();
    for (row, line) in input.lines().into_iter().enumerate() {
        all_heights.push(Vec::new());
        for (_, char) in line.chars().into_iter().enumerate() {
            let height: i8 = char.to_digit(10).unwrap() as i8;
            all_heights[row].push(height as i8);
        }
    }
    all_heights
}

fn count_visible(input: &str) -> u32 {
    let all_heights = get_heights(input);

    // check whether each is visible
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for (row, heights) in all_heights.iter().enumerate() {
        visible.push(Vec::new());
        for (col, height) in heights.iter().enumerate() {
            let left = all_heights[row][0..col].iter().max().unwrap_or(&-1);
            let right = all_heights[row][col + 1..].iter().max().unwrap_or(&-1);
            let up = all_heights[0..row]
                .iter()
                .map(|row| row[col])
                .max()
                .unwrap_or(-1);
            let down = all_heights[row + 1..]
                .iter()
                .map(|row| row[col])
                .max()
                .unwrap_or(-1);

            // is this greater than all heights in some direction?
            visible[row].push(height > left || height > right || *height > up || *height > down);
        }
    }

    // println!("{:?}", visible);

    // count
    visible
        .iter()
        .map(|row| row.iter().filter(|&b| *b).count() as u32)
        .sum()
}

pub fn part_2() -> String {
    format!("{}", best_view(include_str!("input/day8.txt")))
}

fn best_view(input: &str) -> i32 {
    let all_heights = get_heights(input);

    let mut ans: i32 = 0;
    for (row, heights) in all_heights.iter().enumerate() {
        for (col, height) in heights.iter().enumerate() {
            if row == 0 || col == 0 || row == all_heights.len() - 1 || col == heights.len() - 1 {
                continue;
            }

            let mut left = 0;
            for other_height in all_heights[row][0..col].iter().rev() {
                left += 1;
                if other_height >= height {
                    break;
                }
            }

            let mut right = 0;
            for other_height in all_heights[row][col + 1..].iter() {
                right += 1;
                if other_height >= height {
                    break;
                }
            }

            let mut up = 0;
            for other_row in all_heights[0..row].iter().rev() {
                up += 1;
                if other_row[col] >= *height {
                    break;
                }
            }

            let mut down = 0;
            for other_row in all_heights[row + 1..].iter() {
                down += 1;
                if other_row[col] >= *height {
                    break;
                }
            }

            ans = ans.max(left * right * up * down);
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_visible() {
        assert_eq!(
            21,
            count_visible(
                "30373
25512
65332
33549
35390"
            )
        );
    }

    #[test]
    fn test_best_view() {
        assert_eq!(
            8,
            best_view(
                "30373
25512
65332
33549
35390"
            )
        );
    }
}
