extern crate project_euler_rust;
use project_euler_rust::base_ten::truncated::least_significant_digits;
/*

<p>The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime of the form 2<sup>6972593</sup>−1; it contains exactly 2,098,960 digits. Subsequently other Mersenne primes, of the form 2<sup><i>p</i></sup>−1, have been found which contain more digits.</p>
<p>However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits: 28433×2<sup>7830457</sup>+1.</p>
<p>Find the last ten digits of this prime number.</p>


see https://doc.rust-lang.org/std/num/struct.Wrapping.html
https://en.wikipedia.org/wiki/Mersenne_prime#List_of_known_Mersenne_primes
*/

#[test]
fn solution() {
    let mut prime = 1u64;

    for _ in 0..7830457 {
        prime *= 2;
        prime %= 10_000_000_000;
    }
    prime *= 28433;
    prime += 1;

    assert_eq!("8739992577", least_significant_digits(prime.into(), 10));
}

#[test]
fn given() {
    // last_digits(2 ^ 6972593 − 1) = 142924193791
    let mut prime = 1u64;

    for _ in 0..6972593 {
        prime *= 2;
        prime %= 10_000_000_000_000;
    }
    prime -= 1;

    assert_eq!("142924193791", least_significant_digits(prime.into(), 12));
}
