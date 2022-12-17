use std::fs::read_to_string;

pub fn part_1() -> String {
    let stacks = &mut get_stacks();

    for instruction in get_instructions() {
        for _ in 0..instruction.0 {
            let src = stacks[instruction.1].pop().unwrap();
            stacks[instruction.2].push(src);
        }
    }

    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

pub fn part_2() -> String {
    let stacks = &mut get_stacks();

    for instruction in get_instructions() {
        let mut take: Vec<char> = Vec::new();
        for _ in 0..instruction.0 {
            let src = stacks[instruction.1].pop().unwrap();
            take.push(src);
        }

        take.reverse();
        for c in take {
            stacks[instruction.2].push(c);
        }
    }

    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

fn get_stacks() -> Vec<Vec<char>> {
    // init the stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in read_to_string("./src/input/day5.txt").unwrap().lines() {
        if line.starts_with(" 1") {
            break;
        }

        for (i, c) in line.chars().into_iter().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn get_instructions() -> Vec<(usize, usize, usize)> {
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();
    for line in read_to_string("./src/input/day5.txt").unwrap().lines() {
        if line.starts_with("move") {
            let cols = line.split_whitespace().collect::<Vec<&str>>();
            instructions.push((
                cols[1].parse::<usize>().unwrap(),
                cols[3].parse::<usize>().unwrap() - 1,
                cols[5].parse::<usize>().unwrap() - 1,
            ));
        }
    }
    instructions
}
