fn sum_of_squares(n: i32) -> i64 {
    (1..=n).map(|x| x as i64).map(|x| x * x).sum()
}

fn square_of_sum(n: i32) -> i64 {
    let sum: i64 = (1..=n).map(|x| x as i64).sum();
    sum * sum
}

pub fn run() -> i64 {
    square_of_sum(100) - sum_of_squares(100)
}
