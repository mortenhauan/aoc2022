//! # Day 2: Rock Paper Scissors
//! The Elves begin to set up camp on the beach. To decide whose tent gets
//! to be closest to the snack storage, a giant Rock Paper Scissors tournament
//! is already in progress.
//!
//! Rock Paper Scissors is a game between two players. Each game contains many
//! rounds; in each round, the players each simultaneously choose one of Rock,
//! Paper, or Scissors using a hand shape. Then, a winner for that round is
//! selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
//! Rock. If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an encrypted
//! strategy guide (your puzzle input) that they say will be sure to help
//! you win. "The first column is what your opponent is going to play: `A`
//! for Rock, `B` for Paper, and `C` for Scissors. The second column--"
//! Suddenly, the Elf is called away to help with someone's tent.

/// # Part 1
/// The second column, you reason, must be what you should play in response: `X`
/// for Rock, `Y` for Paper, and `Z` for Scissors. Winning every time would be
/// suspicious, so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score.
/// Your total score is the sum of your scores for each round. The score for
/// a single round is the score for the shape you selected (1 for Rock, 2 for
/// Paper, and 3 for Scissors) plus the score for the outcome of the round (0
/// if you lost, 3 if the round was a draw, and 6 if you won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you
/// should calculate the score you would get if you were to follow the strategy
/// guide.
///
/// For example, suppose you were given the following strategy guide:
///
/// ```text
/// A Y
/// B X
/// C Z
/// ```
///
/// This strategy guide predicts and recommends the following:
///
/// - In the first round, your opponent will choose Rock (`A`), and you should choose
/// Paper (`Y`). This ends in a win for you with a score of 8 (2 because you chose
/// Paper + 6 because you won).
/// - In the second round, your opponent will choose Paper (`B`), and you should choose
/// Rock (`X`). This ends in a loss for you with a score of 1 (1 + 0).
/// - The third round is a draw with both players choosing Scissors, giving you a score
/// of 3 + 3 = 6.
///
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
///
/// # Example
/// **What would your total score be if everything goes exactly according to your strategy guide?**
///
/// ```
/// # use crate::day02::task1;
/// #
/// let input = "A Y
/// B X
/// C Z";
///
/// let result = task1(input);
/// assert_eq!(result, 15);
/// ```

pub fn task1(data: &str) -> u32 {
    data.lines()
        .map(|round| {
            match round {
                "A X" => 4, // Rock vs Rock = Draw
                "A Y" => 8, // Rock vs Paper = Won
                "A Z" => 3, // Rock vs Scissors = Lost
                "B X" => 1, // Paper vs Rock = Lost
                "B Y" => 5, // Paper vs Paper = Draw
                "B Z" => 9, // Paper vs Scissors = Won
                "C X" => 7, // Scissors vs Rock = Won
                "C Y" => 2, // Scissors vs Paper = Lost
                "C Z" => 6, // Scissors vs Scissors = Draw
                _ => 0,
            }
        })
        .sum::<u32>()
}

/// # Part 2
/// The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
/// the second column says how the round needs to end: X means you need to lose,
/// Y means you need to end the round in a draw, and Z means you need to win.
/// Good luck!"
///
/// The total score is still calculated in the same way, but now you need to figure
/// out what shape to choose so the round ends as indicated. The example above
/// now goes like this:
///
/// - In the first round, your opponent will choose Rock (`A`), and you need the round
/// to end in a draw (`Y`), so you also choose Rock. This gives you a score of 1 + 3 = 4.
/// - In the second round, your opponent will choose Paper (`B`), and you choose Rock
/// so you lose (`X`) with a score of 1 + 0 = 1.
/// - In the third round, you will defeat your opponent's Scissors with Rock for
/// a score of 1 + 6 = 7.
///
/// Now that you're correctly decrypting the ultra top secret strategy guide, you
/// would get a total score of 12.

/// # Example
/// Following the Elf's instructions for the second column, **what would your total
/// score be if everything goes exactly according to your strategy guide?**
///
/// ```
/// # use crate::day02::task2;
/// #
/// let input = "A Y
/// B X
/// C Z";
///
/// let result = task2(input);
/// assert_eq!(result, 12);
/// ```
///

pub fn task2(data: &str) -> u32 {
    data.lines()
        .map(|round| {
            match round {
                "A X" => 3, // Rock and Loose = Scissors
                "A Y" => 4, // Rock and Draw = Rock
                "A Z" => 8, // Rock and Win = Paper
                "B X" => 1, // Paper and Loose = Rock
                "B Y" => 5, // Paper and Draw = Paper
                "B Z" => 9, // Paper and Win = Scissors
                "C X" => 2, // Scissors and Loose = Paper
                "C Y" => 6, // Scissors and Draw = Scissors
                "C Z" => 7, // Scissors and Win = Rock
                _ => 0,
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_task1_all_possible_outcomes() {
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        let loose = 0;
        let draw = 3;
        let win = 6;

        let rock_vs_rock = task1("A X");
        assert_eq!(rock_vs_rock, rock + draw);

        let rock_vs_paper = task1("A Y");
        assert_eq!(rock_vs_paper, paper + win);

        let rock_vs_scissors = task1("A Z");
        assert_eq!(rock_vs_scissors, scissors + loose);

        let paper_vs_rock = task1("B X");
        assert_eq!(paper_vs_rock, rock + loose);

        let paper_vs_paper = task1("B Y");
        assert_eq!(paper_vs_paper, paper + draw);

        let paper_vs_scissors = task1("B Z");
        assert_eq!(paper_vs_scissors, scissors + win);

        let scissors_vs_rock = task1("C X");
        assert_eq!(scissors_vs_rock, rock + win);

        let scissors_vs_paper = task1("C Y");
        assert_eq!(scissors_vs_paper, paper + loose);

        let scissors_vs_scissors = task1("C Z");
        assert_eq!(scissors_vs_scissors, scissors + draw);
    }

    #[test]
    fn test_task2_all_possible_outcomes() {
        let rock = 1;
        let paper = 2;
        let scissors = 3;
        let loose = 0;
        let draw = 3;
        let win = 6;

        let rock_and_loose = task2("A X");
        assert_eq!(rock_and_loose, scissors + loose);

        let rock_and_draw = task2("A Y");
        assert_eq!(rock_and_draw, rock + draw);

        let rock_and_win = task2("A Z");
        assert_eq!(rock_and_win, paper + win);

        let paper_and_loose = task2("B X");
        assert_eq!(paper_and_loose, rock + loose);

        let paper_and_draw = task2("B Y");
        assert_eq!(paper_and_draw, paper + draw);

        let paper_and_win = task2("B Z");
        assert_eq!(paper_and_win, scissors + win);

        let scissors_and_loose = task2("C X");
        assert_eq!(scissors_and_loose, paper + loose);

        let scissors_and_draw = task2("C Y");
        assert_eq!(scissors_and_draw, scissors + draw);

        let scissors_and_win = task2("C Z");
        assert_eq!(scissors_and_win, rock + win);
    }
}
