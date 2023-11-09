// https://projecteuler.net/problem=2

fn fibonacci(max: i64) -> Vec<i128> {
    // use dynamic programming to cache results
    let mut fibs = vec![1i128, 2i128];
    let mut i = 2;
    while fibs[i - 1] + fibs[i - 2] < max as i128 {
        fibs.push(fibs[i - 1] + fibs[i - 2]);
        i += 1;
    }
    fibs
}

fn even_fib_sum(max: i64) -> i128 {
    fibonacci(max).iter().filter(|&x| x % 2 == 0).sum()
}

pub fn run() -> i128 {
    even_fib_sum(4_000_000)
}
