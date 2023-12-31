// https://projecteuler.net/problem=5

fn is_evenly_divisible(n: u64, max: u64) -> bool {
    for i in 1..max {
        if n % i != 0 {
            return false;
        }
    }
    true
}

fn smallest_possible_evenly_divisible(max: u64) -> u64 {
    let mut n = max;
    while !is_evenly_divisible(n, max) {
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(smallest_possible_evenly_divisible(10), 2520);
    }
}
