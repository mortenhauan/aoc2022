use colored::Colorize;
use day01;

fn main() {
    println!("{}", "Advent of Code 2022\n".purple().bold());

    // Day 01

    println!("{}", "Day 1".green().bold());
    println!(
        "The Elf carrying the most calories has {} calories",
        day01::task1(include_str!("../input/day01.txt"))
    );
    println!(
        "The three Elfs carrying the most callories has a total of {} calories",
        day01::task2(include_str!("../input/day01.txt"))
    );
}
