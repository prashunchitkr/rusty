// https://projecteuler.net/problem=1

fn get_divisible_sum(nums_if_divisible: Vec<i32>, max: i32) -> i32 {
    (1..max)
        .into_iter()
        .filter(|x| nums_if_divisible.iter().any(|y| x % y == 0))
        .sum()
}

pub fn run() -> i32 {
    get_divisible_sum(Vec::from([3, 5]), 1_000)
}
