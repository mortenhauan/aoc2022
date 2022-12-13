fn priority(item: char) -> u32 {
    let item = item as u32;

    if item > 96 {
        item - 96
    } else {
        item - 64 + 26
    }
}

pub fn task1(data: &str) -> u32 {
    data.lines()
        .map(|content| content.split_at(content.len() / 2))
        .map(|(compartment1, compartment2)| {
            compartment1
                .chars()
                .filter(|item| compartment2.contains(*item))
                .map(priority)
                .next()
                .unwrap()
        })
        .sum::<u32>()
}

pub fn task2(data: &str) -> u32 {
    data.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            group[0]
                .chars()
                .find(|item| group[1].contains(*item) && group[2].contains(*item))
                .unwrap()
        })
        .map(priority)
        .sum::<u32>()
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_letter_a() {
        let result = priority('a');
        assert_eq!(result, 1);
    }

    #[test]
    fn test_letter_z() {
        let result = priority('z');
        assert_eq!(result, 26);
    }

    #[test]
    fn test_letter_capital_a() {
        let result = priority('A');
        assert_eq!(result, 27);
    }

    #[test]
    fn test_letter_capital_z() {
        let result = priority('Z');
        assert_eq!(result, 52);
    }

    #[test]
    fn test_task1() {
        let result = task1(include_str!("task.test"));
        assert_eq!(result, 157);
    }

    #[test]
    fn test_task2() {
        let result = task2(include_str!("task.test"));
        assert_eq!(result, 70);
    }
}
