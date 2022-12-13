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
    fn test_task1() {
        let result = task1(include_str!("task.test"));
        assert_eq!(result, 15);
    }

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
    fn test_task2() {
        let result = task2(include_str!("task.test"));
        assert_eq!(result, 12);
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
