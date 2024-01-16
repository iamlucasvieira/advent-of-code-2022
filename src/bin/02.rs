advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input)
        .iter()
        .map(|(oponent, player)| player.score(oponent) + player.value())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input)
        .iter()
        .map(|(oponent, command)| {
            let player = command.from_command(oponent);
            player.score(oponent) + player.value()
        })
        .sum::<u32>()
        .into()
}

#[derive(Debug, PartialEq, Clone)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

enum Command {
    Win,
    Lose,
    Tie,
}

impl Game {
    fn from_str(input: &str) -> Result<Game, &'static str> {
        match input {
            "A" | "X" => Ok(Game::Rock),
            "B" | "Y" => Ok(Game::Paper),
            "C" | "Z" => Ok(Game::Scissors),
            _ => Err("Invalid input"),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }

    fn score(&self, other: &Game) -> u32 {
        match (self, other) {
            (Game::Rock, Game::Paper) => 0,
            (Game::Rock, Game::Scissors) => 6,
            (Game::Paper, Game::Rock) => 6,
            (Game::Paper, Game::Scissors) => 0,
            (Game::Scissors, Game::Rock) => 0,
            (Game::Scissors, Game::Paper) => 6,
            _ => 3,
        }
    }

    fn from_command(&self, other: &Game) -> Game {
        match self.as_command() {
            Command::Win => match other {
                Game::Rock => Game::Paper,
                Game::Paper => Game::Scissors,
                Game::Scissors => Game::Rock,
            },
            Command::Lose => match other {
                Game::Rock => Game::Scissors,
                Game::Paper => Game::Rock,
                Game::Scissors => Game::Paper,
            },
            // Return the same as other
            Command::Tie => other.clone(),
        }
    }

    fn as_command(&self) -> Command {
        match self {
            Game::Rock => Command::Lose,
            Game::Paper => Command::Tie,
            Game::Scissors => Command::Win,
        }
    }
}

fn parse(input: &str) -> Vec<(Game, Game)> {
    input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            let a = iter.next().and_then(|x| Game::from_str(x).ok());
            let b = iter.next().and_then(|x| Game::from_str(x).ok());

            match (a, b) {
                (Some(a), Some(b)) => Some((a, b)),
                (None, Some(_)) => {
                    println!("Missing first game in line: '{}'", line);
                    None
                }
                (Some(_), None) => {
                    println!("Missing second game in line: '{}'", line);
                    None
                }
                (None, None) => {
                    println!("Both games are missing in line: '{}'", line);
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        assert_eq!(Game::from_str("A").ok(), Some(Game::Rock));
        assert_eq!(Game::from_str("B").ok(), Some(Game::Paper));
        assert_eq!(Game::from_str("C").ok(), Some(Game::Scissors));
        assert_eq!(Game::from_str("X").ok(), Some(Game::Rock));
        assert_eq!(Game::from_str("Y").ok(), Some(Game::Paper));
        assert_eq!(Game::from_str("Z").ok(), Some(Game::Scissors));
        assert_eq!(Game::from_str("D").err(), Some("Invalid input"));
    }

    #[test]
    fn test_game_value() {
        assert_eq!(Game::Rock.value(), 1);
        assert_eq!(Game::Paper.value(), 2);
        assert_eq!(Game::Scissors.value(), 3);
    }

    #[test]
    fn test_game_score() {
        assert_eq!(Game::Rock.score(&Game::Rock), 3);
        assert_eq!(Game::Rock.score(&Game::Paper), 0);
        assert_eq!(Game::Rock.score(&Game::Scissors), 6);
    }

    #[test]
    fn test_parse() {
        let result = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            vec![
                (Game::Rock, Game::Paper),
                (Game::Paper, Game::Rock),
                (Game::Scissors, Game::Scissors),
            ]
        );
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
