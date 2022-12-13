pub fn task1(data: &str) -> u32 {
    data.split("\n\n")
        .map(|foods| {
            foods
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .max()
        .unwrap_or_default()
}

pub fn task2(data: &str) -> u32 {
    let mut calories_of_elves = data
        .split("\n\n")
        .map(|foods| {
            foods
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories_of_elves.sort();

    calories_of_elves.into_iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_task1() {
        let result = task1(include_str!("task.test"));
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_task2() {
        let result = task2(include_str!("task.test"));
        assert_eq!(result, 45000);
    }
}
