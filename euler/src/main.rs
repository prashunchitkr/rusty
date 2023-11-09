mod euler;

fn main() {
    let n = std::env::args()
        .nth(1)
        .expect("No problem given")
        .parse::<i32>()
        .unwrap();

    println!(
        "Running problem {0}\nMore info at: https://projecteuler.net/problem={0}",
        n
    );

    match n {
        1 => println!("{}", euler::p1::run()),
        2 => println!("{}", euler::p2::run()),
        3 => println!("{}", euler::p3::run()),
        4 => euler::p4::run(),
        5 => println!("{}", euler::p5::run()),
        6 => println!("{}", euler::p6::run()),
        _ => println!("No solution for problem {}", n),
    }
    return;
}
