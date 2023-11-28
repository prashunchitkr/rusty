fn sum_of_squares(n: i32) -> i64 {
    (1..=n).map(|x| x as i64).map(|x| x * x).sum()
}

fn square_of_sum(n: i32) -> i64 {
    let sum: i64 = (1..=n).map(|x| x as i64).sum();
    sum * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(square_of_sum(10), 3025);
        assert_eq!(square_of_sum(10) - sum_of_squares(10), 2640);
    }
}
