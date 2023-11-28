// https://projecteuler.net/problem=4

fn reverse(n: u32) -> u32 {
    let (mut rev, mut org) = (0, n);

    while org > 0 {
        rev = (rev * 10) + (org % 10);
        org /= 10;
    }

    rev
}

fn is_palindrome(n: u32) -> bool {
    n == reverse(n)
}

fn find_largest_palindrome(digits: u32) -> Result<u32, u32> {
    let min = 10u32.pow(digits - 1);
    let max = 10u32.pow(digits) - 1;

    let mut combinations = Vec::new();

    for x in min..=max {
        for y in (min..=max).rev() {
            combinations.push((x, y));
        }
    }

    let mut result: Vec<u32> = combinations
        .into_iter()
        .map(|(x, y)| x * y)
        .filter(|&n| is_palindrome(n))
        .collect();

    result.sort();
    result.reverse();

    match result.get(0) {
        Some(&x) => Ok(x),
        None => Err(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_largest_palindrome(2).unwrap(), 9009);
    }
}
