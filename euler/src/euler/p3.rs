// https://projecteuler.net/problem=3

fn get_factors(n: i128) -> Vec<i128> {
    let mut factors: Vec<i128> = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
        i += 1;
    }
    factors
}

fn largest_prime_factor(n: i128) -> i128 {
    let mut factors = get_factors(n);
    factors.sort();
    factors.reverse();
    factors
        .iter()
        .find(|&&x| get_factors(x).is_empty())
        .unwrap()
        .clone()
}

pub fn run() -> i128 {
    largest_prime_factor(600851475143)
}
