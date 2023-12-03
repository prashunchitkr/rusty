fn read_input() -> Vec<String> {
    std::fs::read_to_string("inputs/2023/day_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn solution1(input: Vec<String>) -> u32 {
    input.into_iter().fold(0, |acc, line| {
        let digits = line.chars().filter(|&c| c.is_numeric()).collect::<String>();
        let num = format!(
            "{}{}",
            digits.chars().nth(0).unwrap_or('0'),
            digits.chars().last().unwrap_or('0')
        )
        .parse::<u32>()
        .unwrap_or(0);

        acc + num
    })
}

fn run() -> u32 {
    solution1(read_input())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solution1(vec![
                String::from("1abc2"),
                String::from("pqr3stu8vwx"),
                String::from("a1b2c3d4e5f"),
                String::from("treb7uchet")
            ]),
            142
        );
    }

    #[test]
    fn execute() {
        assert_eq!(run(), 54634);
    }
}
