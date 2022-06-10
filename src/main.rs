use std::env;

fn main() {
    let mut numbers_to_compute = Vec::new();

    // Parse arguments from command-line
    // First argument is program's name
    for arg in env::args().skip(1) {
        numbers_to_compute.push(arg.parse().unwrap_or_else(|_| panic!("{} is not a valid unsigned number", arg)));
    }

    if numbers_to_compute.is_empty() {
        eprintln!("You must provide at least a number. Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    // gcd(0, x) = x for all valid x, so we never change the answer here
    let mut current_gcd = 0;
    // gcd(a, b, c) = gcd(a, gcd(b, c))
    // so we can keep applying the gcd with the previous answer and each new number
    for &number in &numbers_to_compute {
        current_gcd = gcd(current_gcd, number);
    }

    println!("gcd of {:?} is {}", numbers_to_compute, current_gcd);
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[test]
fn test_gcd() {
    // simple gcds
    assert_eq!(gcd(3, 5), 1);
    assert_eq!(gcd(2, 8), 2);
    assert_eq!(gcd(8, 2), 2);

    // gcd with 2 big numbers
    assert_eq!(gcd(2 * 3 * 3 * 5 * 5 * 7 * 101, 3 * 5 * 5 * 11 * 101), 3 * 5 * 5 * 101);

    // gcd(0, x) returns x
    assert_eq!(gcd(11, 0), 11);
    assert_eq!(gcd(0, 11), 11);
    assert_eq!(gcd(2, 0), 2);

    // no previous answers change after computing with 0
    assert_eq!(gcd(3, 5), 1);
}
