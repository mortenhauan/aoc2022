//! # Day 5: Supply Stacks
//! The expedition can depart as soon as the final supplies have been unloaded
//! from the ships. Supplies are stored in stacks of marked crates, but because
//! the needed supplies are buried under many other crates, the crates need to
//! be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks.
//! To ensure none of the crates get crushed or fall over, the crane operator
//! will rearrange them in a series of carefully-planned steps. After the
//! crates are rearranged, the desired crates will be at the top of each stack.

/// # Panics
/// This function will panic if the input is not a valid stack.
fn parse_command(command: &str) -> (usize, usize, usize) {
    let captures = command.split_whitespace().collect::<Vec<_>>();

    let number_of_elements_to_move = captures[1].parse::<usize>().unwrap();
    let from = captures[3].parse::<usize>().unwrap();
    let to = captures[5].parse::<usize>().unwrap();

    (number_of_elements_to_move, from, to)
}

/// # Panics
/// This function will panic if the command is not valid.
fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    let number_of_stacks = (crates.lines().last().unwrap().len() + 2) / 4;
    let mut stacks = vec![Vec::<char>::new(); number_of_stacks];

    for line in crates.to_string().lines().rev().skip(1) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push(c);
                }
            });
    }

    stacks
}

/// # Part One
/// The Elves don't want to interrupt the crane operator during this delicate
/// procedure, but they forgot to ask her which crate will end up where, and
/// they want to be ready to unload them as soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates and the
/// rearrangement procedure (your puzzle input). For example:
///
/// ```text
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
/// ```
///
/// In this example, there are three stacks of crates. Stack 1 contains two
/// crates: crate `Z` is on the bottom, and crate N is on top. Stack 2 contains
/// three crates; from bottom to top, they are crates `M`, `C`, and `D`. Finally,
/// stack 3 contains a single crate, `P`.
///
/// Then, the rearrangement procedure is given. In each step of the procedure,
/// a quantity of crates is moved from one stack to a different stack. In the
/// first step of the above rearrangement procedure, one crate is moved from
/// stack 2 to stack 1, resulting in this configuration:
///
/// ```text
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///  ```
///
/// In the second step, three crates are moved from stack 1 to stack 3. Crates
/// are moved *one at a time*, so the first crate to be moved (`D`) ends up below
/// the second and third crates:
///
/// ```text
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
///  ```
///
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates
/// are moved **one at a time**, crate `C` ends up below crate `M`:
///
/// ```text
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
/// ```
///
/// Finally, one crate is moved from stack 1 to stack 2:
///
/// ```text
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
/// ```
///
/// The Elves just need to know **which crate will end up on top of each stack**;
/// in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in
/// stack 3, so you should combine these together and give the Elves the message
/// `CMZ`.
///
/// # Example
/// **After the rearrangement procedure completes, what crate ends up on top of
/// each stack?**
///
/// ```
/// # use crate::day05::part1;
/// #
/// let input = "    [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2";
///
/// assert_eq!(part1(input), "CMZ");
/// ```
/// # Panics
/// This function will panic if the input is not valid.
#[must_use]
pub fn part1(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_crates(crates);

    for command in commands.lines() {
        let (number, from, to) = parse_command(command);

        for _ in 0..number {
            let crate_ = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(crate_);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

/// # Part Two
/// As you watch the crane operator expertly rearrange the crates, you notice
/// the process isn't following your prediction.
///
/// Some mud was covering the writing on the side of the crane, and you quickly
/// wipe it away. The crane isn't a CrateMover 9000 - it's a **CrateMover 9001**.
///
/// The CrateMover 9001 is notable for many new and exciting features: air
/// conditioning, leather seats, an extra cup holder, and **the ability to pick up
/// and move multiple crates at once**.
///
/// Again considering the example above, the crates begin in the same
/// configuration:
///
/// ```text
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// Moving a single crate from stack 2 to stack 1 behaves the same as before:
///
/// ```text
/// [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// However, the action of moving three crates from stack 1 to stack 3 means
/// that those three moved crates **stay in the same order**, resulting in this new
/// configuration:
///
/// ```text
///         [D]
///         [N]
///     [C] [Z]
///     [M] [P]
///  1   2   3
/// ```
///
/// Next, as both crates are moved from stack 2 to stack 1, they retain their
/// order as well:
///
/// ```text
///         [D]
///         [N]
/// [C]     [Z]
/// [M]     [P]
///  1   2   3
/// ```
///
/// Finally, a single crate is still moved from stack 1 to stack 2, but now it's
/// crate C that gets moved:
///
/// ```text
///         [D]
///         [N]
///         [Z]
/// [M] [C] [P]
///  1   2   3
/// ```
///
/// In this example, the CrateMover 9001 has put the crates in a totally
/// different order: `MCD`.
///
/// Before the rearrangement process finishes, update your simulation so that the
/// Elves know where they should stand to be ready to unload the final supplies.
///
/// # Example
/// **After the rearrangement procedure completes, what crate ends up on top of
/// each stack?**
///
/// ```
/// # use crate::day05::part2;
/// #
/// let input = "    [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2";
///
/// assert_eq!(part2(input), "MCD");
/// ```
/// # Panics
/// This function will panic if the input is not valid.
#[must_use]
pub fn part2(input: &str) -> String {
    let (crates, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_crates(crates);

    for command in commands.lines() {
        let (number, from, to) = parse_command(command);

        let split = stacks[from - 1].len() - number;
        let mut crates = stacks[from - 1].split_off(split);
        stacks[to - 1].append(&mut crates);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod day05 {
    use super::*;

    #[test]
    fn test_parse_commands() {
        assert_eq!(parse_command("move 1 from 2 to 1"), (1, 2, 1));
        assert_eq!(parse_command("move 32 from 1 to 3"), (32, 1, 3));
        assert_eq!(parse_command("move 2 from 21 to 1"), (2, 21, 1));
        assert_eq!(parse_command("move 1 from 1 to 25"), (1, 1, 25));
    }

    #[test]
    fn test_parse_crates() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";

        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(parse_crates(input), expected);
    }

    #[test]
    fn test_part1() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part1(input), "CMZ");
    }

    #[test]
    fn test_part2() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part2(input), "MCD");
    }
}
