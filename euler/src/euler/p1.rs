// https://projecteuler.net/problem=1

fn get_divisible_sum(nums_if_divisible: Vec<i32>, max: i32) -> i32 {
    (1..max)
        .into_iter()
        .filter(|x| nums_if_divisible.iter().any(|y| x % y == 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_divisible_sum(vec![3, 5], 10), 23);
    }
}
