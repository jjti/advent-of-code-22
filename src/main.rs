mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("day1: {}", day1::solve());
    println!("day2: {} {}", day2::part_1(), day2::part_2());
    println!("day3: {} {}", day3::part_1(), day3::part_2());
    println!("day4: {} {}", day4::part_1(), day4::part_2());
    println!("day5: {} {}", day5::part_1(), day5::part_2());
    println!("day6: {} {}", day6::part_1(), day6::part_2());
    println!("day7: {} {}", day7::part_1(), day7::part_2());
    println!("day8: {} {}", day8::part_1(), day8::part_2());
    println!("day9: {} {}", day9::part_1(), day9::part_2());
    println!("day10: {}\n{}", day10::part_1(), day10::part_2());
}
