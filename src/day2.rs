use std::fs::read_to_string;

pub fn part_1() -> String {
    let input = read_to_string("src/input/day2.txt").unwrap();

    let mut sum = 0;
    for line in input.split('\n') {
        let mut cols = line.split_ascii_whitespace();
        let first = cols.next().unwrap();
        let last = cols.next().unwrap();

        match last {
            "X" => {
                sum += 1;

                match first {
                    "A" => sum += 3,
                    "B" => {}
                    "C" => sum += 6,
                    _ => panic!(),
                }
            }
            "Y" => {
                sum += 2;

                match first {
                    "A" => sum += 6,
                    "B" => sum += 3,
                    "C" => {}
                    _ => panic!(),
                }
            }
            "Z" => {
                sum += 3;

                match first {
                    "A" => {}
                    "B" => sum += 6,
                    "C" => sum += 3,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    format!("{}", sum)
}

pub fn part_2() -> String {
    let input = read_to_string("src/input/day2.txt").unwrap();

    let mut sum = 0;
    for line in input.split('\n') {
        let mut cols = line.split_ascii_whitespace();
        let first = cols.next().unwrap();
        let last = cols.next().unwrap();

        match last {
            "X" => match first {
                "A" => sum += 3,
                "B" => sum += 1,
                "C" => sum += 2,
                _ => panic!(),
            },
            "Y" => {
                sum += 3;

                match first {
                    "A" => sum += 1,
                    "B" => sum += 2,
                    "C" => sum += 3,
                    _ => panic!(),
                }
            }
            "Z" => {
                sum += 6;

                match first {
                    "A" => sum += 2,
                    "B" => sum += 3,
                    "C" => sum += 1,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    format!("{}", sum)
}
