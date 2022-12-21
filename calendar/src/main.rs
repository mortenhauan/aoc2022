use std::time::Instant;

fn main() {
    let calendar_timer = Instant::now();
    println!("\x1b[1;35mAdvent of Code 2022\x1b[0m");

    // Day 01
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 1\x1b[0m");
    println!(
        "Part 1: {}",
        day01::part1(include_str!("../input/day01.txt"))
    );
    println!(
        "Part 2: {}",
        day01::part2(include_str!("../input/day01.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 01: {:?}", duration);

    // Day 02
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 2\x1b[0m");
    println!(
        "Part 1: {}",
        day02::part1(include_str!("../input/day02.txt"))
    );
    println!(
        "Part 2: {}",
        day02::part2(include_str!("../input/day02.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 02: {:?}", duration);

    // Day 03
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 3\x1b[0m");
    println!(
        "Part 1: {}",
        day03::part1(include_str!("../input/day03.txt"))
    );
    println!(
        "Part 2: {}",
        day03::part2(include_str!("../input/day03.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 03: {:?}", duration);

    // Day 04
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 4\x1b[0m");
    println!(
        "Part 1: {}",
        day04::part1(include_str!("../input/day04.txt"))
    );
    println!(
        "Part 2: {}",
        day04::part2(include_str!("../input/day04.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 04: {:?}", duration);

    // Day 05
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 5\x1b[0m");
    println!(
        "Part 1: {}",
        day05::part1(include_str!("../input/day05.txt"))
    );
    println!(
        "Part 2: {}",
        day05::part2(include_str!("../input/day05.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 05: {:?}", duration);

    // Day 06
    let task_timer = Instant::now();
    println!("\n\x1b[1;32mDay 6\x1b[0m");
    println!(
        "Part 1: {}",
        day06::part1(include_str!("../input/day06.txt"))
    );
    println!(
        "Part 2: {}",
        day06::part2(include_str!("../input/day06.txt"))
    );
    let duration = task_timer.elapsed();
    println!("Time used on Day 06: {:?}", duration);

    // Summary
    let duration = calendar_timer.elapsed();
    println!("\nTime used on Advent of Code 2022: {:?}", duration);
}
