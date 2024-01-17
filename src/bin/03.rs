advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input)
        .iter()
        .filter_map(|line| {
            let (first, second) = split_half(line);
            common_char(first, second).and_then(char_priority)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input)
        .chunks(3)
        .filter_map(|chunk| {
            let (first, second, third) = (chunk[0], chunk[1], chunk[2]);
            common_char_three(first, second, third).and_then(char_priority)
        })
        .sum::<u32>()
        .into()
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect()
}

fn split_half(input: &str) -> (&str, &str) {
    let mid = input.len() / 2;
    input.split_at(mid)
}

//// Returs a character that appears in both strings
fn common_char(first: &str, second: &str) -> Option<char> {
    first.chars().find(|&c| second.contains(c))
}

//// Returs a character that appears in all three strings
fn common_char_three(first: &str, second: &str, third: &str) -> Option<char> {
    first
        .chars()
        .find(|&c| second.contains(c) && third.contains(c))
}

fn char_priority(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_half() {
        assert_eq!(split_half("abcdef"), ("abc", "def"));
        assert_eq!(split_half("abcde"), ("ab", "cde"));
        assert_eq!(split_half("abc"), ("a", "bc"));
        assert_eq!(split_half("ab"), ("a", "b"));
        assert_eq!(split_half("a"), ("", "a"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&advent_of_code::template::read_file("examples", DAY)),
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
        )
    }

    #[test]
    fn test_common_char() {
        assert_eq!(common_char("abc", "def"), None);
        assert_eq!(common_char("abc", "aef"), Some('a'));
        assert_eq!(common_char("abc", "bdf"), Some('b'));
        assert_eq!(common_char("abc", "cdf"), Some('c'));
        assert_eq!(common_char("abc", "def"), None);
        assert_eq!(common_char("abc", "def"), None);
        assert_eq!(common_char("abc", "def"), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
