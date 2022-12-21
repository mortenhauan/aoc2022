//! # Day 4: Camp Cleanup
//! Space needs to be cleared before the last supplies can be unloaded from the ships,
//! and so several Elves have been assigned the job of cleaning up sections of the
//! camp. Every section has a unique ID number, and each Elf is assigned a range of
//! section IDs.

fn overlap(assignment: &str, completely: bool) -> bool {
    let (elf1, elf2) = assignment.split_once(',').unwrap();
    let (elf1_start, elf1_end) = elf1.split_once('-').unwrap();
    let (elf2_start, elf2_end) = elf2.split_once('-').unwrap();

    let elf1_start = elf1_start.parse::<u16>().unwrap();
    let elf1_end = elf1_end.parse::<u16>().unwrap();
    let elf2_start = elf2_start.parse::<u16>().unwrap();
    let elf2_end = elf2_end.parse::<u16>().unwrap();

    if completely {
        (elf1_start >= elf2_start && elf1_end <= elf2_end)
            || (elf2_start >= elf1_start && elf2_end <= elf1_end)
    } else {
        elf1_end >= elf2_start && elf1_start <= elf2_end
    }
}

/// # Part One
/// However, as some of the Elves compare their section assignments with each other,
/// they've noticed that many of the assignments overlap. To try to quickly find
/// overlaps and reduce duplicated effort, the Elves pair up and make a big list of
/// the section assignments for each pair (your puzzle input).
///
/// For example, consider the following list of section assignment pairs:
///
/// ```text
/// 2-4,6-8
/// /// 2-3,4-5
/// 5-7,7-9
/// 2-8,3-7
/// 6-6,4-6
/// 2-6,4-8
/// ```
///
/// For the first few pairs, this list means:
///
/// - Within the first pair of Elves, the first Elf was assigned sections `2-4` (sections
/// `2`, `3`, and `4`), while the second Elf was assigned sections `6-8` (sections `6`, `7`, `8`).
/// - The Elves in the second pair were each assigned two sections.
/// - The Elves in the third pair were each assigned three sections: one got sections
/// `5`, `6`, and `7`, while the other also got `7`, plus `8` and `9`.
///
/// This example list uses single-digit section IDs to make it easier to draw; your
/// actual list might contain larger numbers. Visually, these pairs of section
/// assignments look like this:
///
/// ```text
/// .234.....  2-4
/// .....678.  6-8
///
/// .23......  2-3
/// ...45....  4-5
///
/// ....567..  5-7
/// ......789  7-9
///
/// .2345678.  2-8
/// ..34567..  3-7
///
/// .....6...  6-6
/// ...456...  4-6
///
/// .23456...  2-6
/// ...45678.  4-8
/// ```
///
/// Some of the pairs have noticed that one of their assignments fully contains the
/// other. For example, `2-8` fully contains `3-7`, and `6-6` is fully contained by `4-6`.
/// In pairs where one assignment fully contains the other, one Elf in the pair would
/// be exclusively cleaning sections their partner will already be cleaning, so these
/// seem like the most in need of reconsideration. In this example, there are `2` such
/// pairs.
///
/// # Example
/// **In how many assignment pairs does one range fully contain the other?**
///
/// ```
/// # use crate::day04::part1;
/// #
/// let input = "2-4,6-8
/// 2-3,4-5
/// 5-7,7-9
/// 2-8,3-7
/// 6-6,4-6
/// 2-6,4-8";
///
/// assert_eq!(part1(input), 2);
/// ```
#[must_use]
pub fn part1(data: &str) -> usize {
    data.lines()
        .filter(|assignment| overlap(assignment, true))
        .count()
}

/// # Part Two
/// It seems like there is still quite a bit of duplicate work planned. Instead, the Elves
/// would like to know the number of pairs that overlap at all.
///
/// In the above example, the first two pairs (`2-4,6-8` and `2-3,4-5`) don't overlap, while
/// the remaining four pairs (`5-7,7-9`, `2-8,3-7`, `6-6,4-6`, and `2-6,4-8`) do overlap:
///
/// - `5-7,7-9` overlaps in a single section, `7`.
/// - `2-8,3-7` overlaps all of the sections `3` through `7`.
/// - `6-6,4-6` overlaps in a single section, `6`.
/// - `2-6,4-8` overlaps in sections `4`, `5`, and `6`.
///
/// So, in this example, the number of overlapping assignment pairs is `4`.
///
/// # Example
/// **In how many assignment pairs do the ranges overlap?**
///
/// ```
/// # use crate::day04::part2;
/// #
/// let input = "2-4,6-8
/// 2-3,4-5
/// 5-7,7-9
/// 2-8,3-7
/// 6-6,4-6
/// 2-6,4-8";
///
/// assert_eq!(part2(input), 4);
#[must_use]
pub fn part2(data: &str) -> usize {
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
}
