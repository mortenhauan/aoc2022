use colored::Colorize;
use std::time::Instant;

fn main() {
    let calendar_timer = Instant::now();
    println!("{}", "Advent of Code 2022".purple().bold());

    // Day 01
    let task_timer = Instant::now();
    println!("\n{}", "Day 1".green().bold());
    println!(
        "Task1: {}",
        day01::task1(include_str!("../input/day01.txt"))
    );
    println!(
        "Task2: {}",
        day01::task2(include_str!("../input/day01.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 01: {:?}", duration);

    // Day 02
    let task_timer = Instant::now();
    println!("\n{}", "Day 2".green().bold());
    println!(
        "Task1: {}",
        day02::task1(include_str!("../input/day02.txt"))
    );
    println!(
        "Task2: {}",
        day02::task2(include_str!("../input/day02.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 02: {:?}", duration);

    // Day 03
    let task_timer = Instant::now();
    println!("\n{}", "Day 3".green().bold());
    println!(
        "Task1: {}",
        day03::task1(include_str!("../input/day03.txt"))
    );
    println!(
        "Task2: {}",
        day03::task2(include_str!("../input/day03.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 03: {:?}", duration);

    // Day 04
    let task_timer = Instant::now();
    println!("\n{}", "Day 4".green().bold());
    println!(
        "Task1: {}",
        day04::task1(include_str!("../input/day04.txt"))
    );
    println!(
        "Task2: {}",
        day04::task2(include_str!("../input/day04.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 04: {:?}", duration);

    // Summary
    let duration = calendar_timer.elapsed();
    println!("\nTime used on Advent of Code 2022: {:?}", duration);
}
