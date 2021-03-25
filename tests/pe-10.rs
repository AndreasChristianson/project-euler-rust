extern crate project_euler_rust;

use project_euler_rust::generators::primes::*;

/*
<p>The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.</p>
<p>Find the sum of all the primes below two million.</p>
*/

#[test]
fn given() {
    let generator: PrimeSequence<u32> = primes();
    let sum: u32 = generator.take_while(|&p| p < 10).sum();
    assert_eq!(sum, 17);
}

#[test]
fn solution() {
    let generator: PrimeSequence<u128> = primes();
    let sum: u128 = generator.take_while(|&p| p < 2_000_000).sum();
    assert_eq!(sum, 142913828922);
}
