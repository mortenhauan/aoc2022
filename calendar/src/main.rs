use colored::Colorize;
use std::time::Instant;

fn main() {
    let calendar_timer = Instant::now();
    println!("{}", "Advent of Code 2022".purple().bold());

    // Day 01
    let task_timer = Instant::now();
    println!("\n{}", "Day 1".green().bold());
    println!(
        "The Elf carrying the most calories has {} calories",
        day01::task1(include_str!("../input/day01.txt"))
    );
    println!(
        "The three Elfs carrying the most callories has a total of {} calories",
        day01::task2(include_str!("../input/day01.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 01: {:?}", duration);

    // Day 02
    let task_timer = Instant::now();
    println!("\n{}", "Day 2".green().bold());
    println!(
        "If I follow my guessed strategy guide, I will get a score of {}",
        day02::task1(include_str!("../input/day02.txt"))
    );
    println!(
        "If I follow my correct strategy guide, I will get a score of {}",
        day02::task2(include_str!("../input/day02.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 02: {:?}", duration);

    // Day 03
    let task_timer = Instant::now();
    println!("\n{}", "Day 3".green().bold());
    println!(
        "The sum of priorities for items are: {}",
        day03::task1(include_str!("../input/day03.txt"))
    );
    println!(
        "The sum of priorities for badges are: {}",
        day03::task2(include_str!("../input/day03.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 03: {:?}", duration);

    let duration = calendar_timer.elapsed();
    println!("\nTime used on Advent of Code 2022: {:?}", duration);
}
