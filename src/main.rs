mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("day1: {}", day1::solve());
    println!("day2: {} {}", day2::part_1(), day2::part_2());
    println!("day3: {} {}", day3::part_1(), day3::part_2());
    println!("day4: {} {}", day4::part_1(), day4::part_2());
}
