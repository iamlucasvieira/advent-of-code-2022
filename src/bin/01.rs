advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut group_sums = parse(input);
    group_sums.sort();
    group_sums.reverse();
    group_sums.iter().take(3).sum::<u32>().into()
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
