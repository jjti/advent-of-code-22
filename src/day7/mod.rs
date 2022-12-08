use std::collections::HashMap;

pub fn part_1() -> String {
    format!("{}", calc(include_str!("input.txt")))
}

pub fn part_2() -> String {
    let dirs = walk(include_str!("input.txt"));
    let used = *dirs.get(".").expect("missing root");
    let free = 70_000_000 - used;
    let required = 30_000_000 - free;

    let mut min_size = i32::MAX;
    for (_, size) in dirs.into_iter() {
        if size >= required && size < min_size {
            min_size = size;
        }
    }

    format!("{}", min_size)
}

fn walk(input: &str) -> HashMap<String, i32> {
    let mut nodes: HashMap<String, i32> = HashMap::new();
    let mut pwd = ".".to_string();
    nodes.insert(pwd.clone(), 0);

    for line in input.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();

        match cols[0] {
            "$" => match cols[1] {
                "cd" => match cols[2] {
                    ".." => {
                        let mut parts = pwd.split('/').collect::<Vec<&str>>();
                        parts.pop();
                        pwd = parts.join("/");
                    }
                    "/" => {
                        pwd = ".".to_string();
                    }
                    _ => {
                        pwd = format!("{}/{}", pwd, cols[2]);
                        if !nodes.contains_key(&pwd) {
                            nodes.insert(pwd.clone(), 0);
                        }
                    }
                },
                "ls" => continue,
                _ => panic!(),
            },
            "dir" => continue,
            _ => {
                let size = cols[0].parse::<i32>().unwrap();

                let mut parts = pwd.split('/').collect::<Vec<&str>>();
                for _ in 0..parts.len() {
                    let path = parts.join("/").clone();

                    let val = *nodes.get(&path).unwrap_or(&0);
                    nodes.insert(path, val + size);

                    parts.pop();
                }
            }
        }
    }

    nodes
}

fn calc(input: &str) -> i32 {
    let mut sum = 0;
    for (_, size) in walk(input).into_iter() {
        if size <= 100_000 {
            sum += size;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            95437,
            calc(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            )
        );
    }
}
