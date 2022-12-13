fn overlap(assignment: &str, completely: bool) -> bool {
    let (elf1, elf2) = assignment.split_once(",").unwrap();
    let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
    let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();

    let elf1_start = elf1_start.parse::<u16>().unwrap();
    let elf1_end = elf1_end.parse::<u16>().unwrap();
    let elf2_start = elf2_start.parse::<u16>().unwrap();
    let elf2_end = elf2_end.parse::<u16>().unwrap();

    if completely {
        return (elf1_start >= elf2_start && elf1_end <= elf2_end)
            || (elf2_start >= elf1_start && elf2_end <= elf1_end);
    } else {
        return elf1_end >= elf2_start && elf1_start <= elf2_end;
    }
}

pub fn task1(data: &str) -> usize {
    data.lines()
        .filter(|assignment| overlap(assignment, true))
        .count()
}

pub fn task2(data: &str) -> usize {
    data.lines()
        .filter(|assignment| overlap(assignment, false))
        .count()
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn test_overlapping_completely() {
        assert_eq!(overlap("2-8,3-7", true), true);
        assert_eq!(overlap("6-6,4-6", true), true);
        assert_eq!(overlap("5-6,4-6", true), true);
    }

    #[test]
    fn test_not_overlapping_completely() {
        assert_eq!(overlap("2-4,6-8", true), false);
        assert_eq!(overlap("2-3,4-5", true), false);
        assert_eq!(overlap("2-6,4-8", true), false);
        assert_eq!(overlap("8-10,4-8", true), false);
    }

    #[test]
    fn test_overlapping() {
        assert_eq!(overlap("2-8,3-7", false), true);
        assert_eq!(overlap("6-6,4-6", false), true);
        assert_eq!(overlap("5-7,7-9", false), true);
        assert_eq!(overlap("2-6,4-8", false), true);
        assert_eq!(overlap("8-10,4-8", false), true);
    }

    #[test]
    fn test_not_overlapping() {
        assert_eq!(overlap("2-4,6-8", false), false);
        assert_eq!(overlap("2-3,4-5", false), false);
        assert_eq!(overlap("6-8,4-5", false), false);
    }

    #[test]
    fn test_task1() {
        let result = task1(include_str!("task.test"));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_task2() {
        let result = task2(include_str!("task.test"));
        assert_eq!(result, 4);
    }
}
