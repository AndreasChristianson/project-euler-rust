extern crate project_euler_rust;

use project_euler_rust::generators::primes::primes;
/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

#[test]
fn given() {
    assert_eq!(primes::<u64>().take(6).last().unwrap(), 13);
}

#[test]
fn solution() {
    assert_eq!(primes::<u128>().take(10_001).last().unwrap(), 104743);
}
