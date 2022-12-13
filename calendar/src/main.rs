use colored::Colorize;
use day01;
use day02;

fn main() {
    println!("{}", "Advent of Code 2022".purple().bold());

    // Day 01
    println!("\n{}", "Day 1".green().bold());
    println!(
        "The Elf carrying the most calories has {} calories",
        day01::task1(include_str!("../input/day01.txt"))
    );
    println!(
        "The three Elfs carrying the most callories has a total of {} calories",
        day01::task2(include_str!("../input/day01.txt"))
    );

    // Day 02
    println!("\n{}", "Day 2".green().bold());
    println!(
        "If I follow my guessed strategy guide, I will get a score of {}",
        day02::task1(include_str!("../input/day02.txt"))
    );
    println!(
        "If I follow my correct strategy guide, I will get a score of {}",
        day02::task2(include_str!("../input/day02.txt"))
    );
}
